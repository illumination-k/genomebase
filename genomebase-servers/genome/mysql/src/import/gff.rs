use std::{collections::HashMap, fs::File};

use anyhow::{Ok, Result};
use bio::io::gff::{GffType, Reader as GffReader, Record as GffRecord};
use genome_domain::{
    gene::{
        transcript::{annotation::FunctionalAnnotation, Strand, Transcript},
        Gene,
    },
    AnnotationVersion,
};
use sqlx::{MySql, MySqlPool, QueryBuilder};
use std::io::BufReader;
use uuid::Uuid;

pub async fn from_gff(
    gff_path: &str,
    annotation_version_id: Uuid,
    annotation_version: AnnotationVersion,
    pool: MySqlPool,
) -> Result<()> {
    let mut reader = GffReader::new(BufReader::new(File::open(gff_path)?), GffType::GFF3);


    type GeneID = String;
    type TranscriptID = String;

    let mut gene_id2gene: HashMap<GeneID, Gene> = HashMap::new();
    let mut transcript_id2transcript: HashMap<TranscriptID, (GeneID, Transcript)> = HashMap::new();
    let mut transcript_id2gff_records: HashMap<TranscriptID, Vec<GffRecord>> = HashMap::new();

    let default_gene_children: [&str; 5] = ["mRNA", "tRNA", "rRNA", "miRNA", "pre-miRNA"];
    for record in reader.records() {
        let record = record?;

        if record.attributes().get("ID").is_none() {
            // warn?
            continue;
        }

        // unwrap is safe because we checked it above
        let id = record.attributes().get("ID").unwrap();

        if default_gene_children.contains(&record.feature_type()) {
            // parent must exist because of gff3 specfications
            let gene_id = record.attributes().get("Parent").expect("msg").to_owned();

            gene_id2gene
                .entry(gene_id.clone())
                .or_insert(Gene::new(Uuid::new_v4(), gene_id.clone(), vec![]));

            transcript_id2transcript.entry(id.to_owned()).or_insert(
                (gene_id, Transcript::new(
                    Uuid::new_v4(),
                    id.to_owned(),
                    record.feature_type().to_owned(),
                    *record.start() as i32,
                    *record.end() as i32,
                    Strand::Minus,
                    vec![],
                    FunctionalAnnotation::new(None),
                )),
            );
        } else {
            let transcript_id = record.attributes().get("Parent").expect("msg").to_owned();
            transcript_id2gff_records.entry(transcript_id).or_insert(vec![]).push(record);
        }
    }

    for (transcript_id, gff_records) in transcript_id2gff_records {
        let (_, transcript) = transcript_id2transcript.get_mut(&transcript_id).unwrap();
        transcript.extend_gff_records(gff_records);
    }

    for (gene_id, transcript) in transcript_id2transcript.into_values() {
        let gene = gene_id2gene.get_mut(&gene_id).unwrap();
        gene.push_transcript(transcript);
    }

    let mut gene_query_builder =
        QueryBuilder::<MySql>::new("INSERT INTO genes (id, gene_id, annotation_version_id) ");

    let mut transcript_query_builder = QueryBuilder::<MySql>::new(
        "INSERT INTO transcripts (id, transcript_id, gene_id, annotation_version_id, start, end, transcript_type, strand) ",
    );

    let mut gff_record_query_builder = QueryBuilder::<MySql>::new(
        "INSERT INTO gff_records (transcript_id, annotation_version_id, start, end, seqname, source, type, score, phase, attributes) ",
    );

    gene_query_builder.push_values(gene_id2gene.into_values(), |mut b, gene| {
        b.push_bind(gene.id().as_bytes().to_vec());
        b.push_bind(gene.gene_id().to_string());
        b.push_bind(annotation_version_id.as_bytes().as_slice());

        transcript_query_builder.push_values(gene.transcripts().iter(), |mut tb, transcript| {
            tb.push_bind(transcript.id().as_bytes().to_vec());
            tb.push_bind(transcript.transcript_id().to_string());
            tb.push_bind(gene.id().as_bytes().to_vec());
            tb.push_bind(annotation_version_id.as_bytes().as_slice());
            tb.push_bind(transcript.start());
            tb.push_bind(transcript.end());
            tb.push_bind(transcript.transcript_type().to_string());
            tb.push_bind(transcript.strand().to_string());

            gff_record_query_builder.push_values(
                transcript.gff_records().iter(),
                |mut gb, gff_record| {
                    gb.push_bind(transcript.id().as_bytes().to_vec());
                    gb.push_bind(annotation_version_id.as_bytes().as_slice());
                    gb.push_bind(*gff_record.start() as i32);
                    gb.push_bind(*gff_record.end() as i32);
                    gb.push_bind(gff_record.seqname().to_string());
                    gb.push_bind(gff_record.source().to_string());
                    gb.push_bind(gff_record.feature_type().to_string());
                    gb.push_bind(
                        gff_record
                            .score()
                            .map(|s| s.to_string())
                            .unwrap_or(".".to_string()),
                    );
                    gb.push_bind(gff_record.frame().to_string());
                    gb.push_bind(serde_json::to_string(gff_record.attributes()).unwrap());
                },
            );
        });
    });

    let mut transcation = pool.begin().await?;

    gene_query_builder.build().execute(&mut transcation).await?;
    transcript_query_builder
        .build()
        .execute(&mut transcation)
        .await?;
    gff_record_query_builder
        .build()
        .execute(&mut transcation)
        .await?;

    transcation.commit().await?;

    Ok(())
}
