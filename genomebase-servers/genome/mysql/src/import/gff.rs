use std::{collections::HashMap, io::Read};

use anyhow::{anyhow, Result};
use bio::io::gff::{Reader as GffReader, Record as GffRecord};
use genome_domain::gene::{
    transcript::{annotation::FunctionalAnnotation, Transcript},
    Gene, Strand,
};
use sqlx::{MySql, MySqlPool, QueryBuilder};
use uuid::Uuid;

pub async fn from_gff<R: Read>(
    gff_reader: GffReader<R>,
    annotation_version_id: Uuid,
    pool: MySqlPool,
) -> Result<()> {
    let genes = gff_to_genes(gff_reader)?;

    let (mut gene_query_builder, mut transcript_query_builder, mut gff_record_query_builder) =
        build_query_bulders(genes, annotation_version_id);

    let mut transcation = pool.begin().await?;

    gene_query_builder.build().execute(&mut transcation).await?;

    transcript_query_builder
        .build()
        .execute(&mut transcation)
        .await?;

    // gff_record_query_builder
    //     .build()
    //     .execute(&mut transcation)
    //     .await?;

    transcation.commit().await?;

    Ok(())
}

struct TranscriptDTO {
    gene_id: Uuid,
    transcript: Transcript,
}

struct GffTranscriptDto {
    transcript_id: Uuid,
    gff_record: GffRecord,
}

struct GffGeneDto {
    gene_id: Uuid,
    gff_record: GffRecord,
}

fn build_query_bulders<'a>(
    genes: Vec<Gene>,
    annotation_version_id: Uuid,
) -> (
    QueryBuilder<'a, MySql>,
    QueryBuilder<'a, MySql>,
    QueryBuilder<'a, MySql>,
) {
    let mut gene_query_builder = QueryBuilder::<MySql>::new(
        "INSERT INTO genes (id, gene_id, annotation_version_id, start, end, strand) ",
    );

    let mut transcript_query_builder = QueryBuilder::<MySql>::new(
        "INSERT INTO transcripts (id, transcript_id, gene_id, annotation_version_id, start, end, transcript_type, strand) ",
    );

    let mut gff_record_query_builder = QueryBuilder::<MySql>::new(
        "INSERT INTO gff_records (transcript_id, annotation_version_id, start, end, seqname, source, type, score, phase, attributes) ",
    );

    let mut transcript_dtos = vec![];
    let mut gff_transcript_dtos = vec![];
    let mut gff_gene_dtos = vec![];

    for gene in genes.iter() {
        for t in gene.transcripts().iter() {
            transcript_dtos.push(TranscriptDTO {
                gene_id: gene.id().clone(),
                transcript: t.clone(),
            });
            for gr in t.gff_records().iter() {
                gff_transcript_dtos.push(GffTranscriptDto {
                    transcript_id: t.id().clone(),
                    gff_record: gr.clone(),
                });
                gff_gene_dtos.push(GffGeneDto {
                    gene_id: gene.id().clone(),
                    gff_record: gr.clone(),
                });
            }
        }
    }

    gene_query_builder.push_values(genes.iter(), |mut b, gene| {
        b.push_bind(gene.id().as_bytes().to_vec());
        b.push_bind(gene.gene_id().to_string());
        b.push_bind(annotation_version_id.as_bytes().to_vec());
        b.push_bind(gene.start());
        b.push_bind(gene.end());
        b.push_bind(gene.strand().to_string());
    });

    transcript_query_builder.push_values(transcript_dtos.into_iter(), |mut b, t| {
        b.push_bind(t.transcript.id().as_bytes().to_vec());
        b.push_bind(t.transcript.transcript_id().to_string());
        b.push_bind(t.gene_id.as_bytes().to_vec());
        b.push_bind(annotation_version_id.as_bytes().to_vec());
        b.push_bind(t.transcript.start());
        b.push_bind(t.transcript.end());
        b.push_bind(t.transcript.transcript_type().to_string());
        b.push_bind(t.transcript.strand().to_string());
    });

    (
        gene_query_builder,
        transcript_query_builder,
        gff_record_query_builder,
    )
}

fn gff_to_genes<R: Read>(mut gff_reader: GffReader<R>) -> Result<Vec<Gene>> {
    type GeneID = String;
    type TranscriptID = String;

    let mut gene_id2gene: HashMap<GeneID, Gene> = HashMap::new();
    let mut transcript_id2transcript: HashMap<TranscriptID, (GeneID, Transcript)> = HashMap::new();
    let mut transcript_id2gff_records: HashMap<TranscriptID, Vec<GffRecord>> = HashMap::new();

    let default_gene_children: [&str; 5] = ["mRNA", "tRNA", "rRNA", "miRNA", "pre-miRNA"];
    for record in gff_reader.records() {
        let record = record?;

        if record.attributes().get("ID").is_none() {
            // warn?
            continue;
        }

        // unwrap is safe because we checked it above
        let id = record
            .attributes()
            .get("ID")
            .ok_or(anyhow!("ID not found in attributes: {:?}", record))?;

        if record.feature_type() == "gene" {
            gene_id2gene.entry(id.clone()).or_insert(Gene::new(
                Uuid::new_v4(),
                id.clone(),
                *record.start() as i32,
                *record.end() as i32,
                Strand::Minus,
                vec![],
            ));
        } else if default_gene_children.contains(&record.feature_type()) {
            // parent must exist because of gff3 specfications
            let gene_id = record
                .attributes()
                .get("Parent")
                .ok_or(anyhow!(
                    "Parent is not exist even if {}\n record: {:?}",
                    record.feature_type(),
                    record
                ))?
                .to_owned();

            transcript_id2transcript.entry(id.to_owned()).or_insert((
                gene_id,
                Transcript::new(
                    Uuid::new_v4(),
                    id.to_owned(),
                    record.feature_type().to_owned(),
                    *record.start() as i32,
                    *record.end() as i32,
                    Strand::Minus,
                    vec![],
                    FunctionalAnnotation::new(None),
                ),
            ));
        } else if ["cds", "exon", "intron"]
            .contains(&(record.feature_type().to_lowercase().as_str()))
        {
            let transcript_id = record
                .attributes()
                .get("Parent")
                .ok_or(anyhow!(
                    "Parent is not exist even if {}\n record: {:?}",
                    record.feature_type(),
                    record
                ))?
                .to_owned();
            transcript_id2gff_records
                .entry(transcript_id)
                .or_insert(vec![])
                .push(record);
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

    Ok(gene_id2gene.into_values().collect())
}

#[cfg(test)]
mod test_import_gff {
    use bio::io::gff::GffType;
    use genome_domain::AnnotationVersion;

    use std::{
        fs::File,
        io::{BufReader, Write},
    };

    #[allow(unused_imports)]
    use crate::{repositories::organism::OrganismRepository, MIGRATOR};

    use genome_domain::{GenomeVersion, IOrganismRepository, Organism};
    use tempfile::NamedTempFile;

    use super::*;

    const SAMPLE_GFF: &[u8] = b"# gff3
chr1\t.\tgene\t1\t100\t.\t+\t.\tID=gene1;gene_name=Gene1
chr1\t.\tmRNA\t1\t100\t.\t+\t.\tID=mRNA1;Parent=gene1;transcript_name=Transcript1
chr1\t.\texon\t1\t20\t.\t+\t.\tID=exon1;Parent=mRNA1;exon_number=1
chr1\t.\tcds\t1\t10\t.\t+\t0\tID=cds1;Parent=mRNA1
chr1\t.\texon\t21\t40\t.\t+\t.\tID=exon2;Parent=mRNA1;exon_number=2
chr1\t.\tcds\t21\t30\t.\t+\t0\tID=cds2;Parent=mRNA1
chr1\t.\texon\t41\t60\t.\t+\t.\tID=exon3;Parent=mRNA1;exon_number=3
chr1\t.\tcds\t31\t40\t.\t+\t0\tID=cds3;Parent=mRNA1
chr1\t.\tmRNA\t1\t80\t.\t+\t.\tID=mRNA2;Parent=gene1;transcript_name=Transcript2
chr1\t.\texon\t1\t20\t.\t+\t.\tID=exon4;Parent=mRNA2;exon_number=1
chr1\t.\tcds\t1\t10\t.\t+\t0\tID=cds4;Parent=mRNA2
chr1\t.\texon\t21\t40\t.\t+\t.\tID=exon5;Parent=mRNA2;exon_number=2
chr1\t.\tcds\t21\t30\t.\t+\t0\tID=cds5;Parent=mRNA2
chr1\t.\texon\t61\t80\t.\t+\t.\tID=exon6;Parent=mRNA2;exon_number=3
chr1\t.\tcds\t31\t40\t.\t+\t0\tID=cds6;Parent=mRNA2
chr1\t.\tgene\t1\t100\t.\t+\t.\tID=gene2a;gene_name=Gene2a
chr1\t.\trRNA\t201\t300\t.\t-\t.\tID=rRNA1;Parent=gene2a;transcript_name=Transcript3
chr1\t.\texon\t201\t220\t.\t-\t.\tID=exon7;Parent=rRNA1;exon_number=1
chr1\t.\tcds\t211\t220\t.\t-\t0\tID=cds7;Parent=rRNA1
chr1\t.\texon\t241\t260\t.\t-\t.\tID=exon8;Parent=rRNA1;exon_number=2
chr1\t.\tcds\t241\t251\t.\t-\t0\tID=cds8;Parent=rRNA1
chr1\t.\texon\t281\t300\t.\t-\t.\tID=exon9;Parent=rRNA1;exon_number=3
chr1\t.\tgene\t1\t100\t.\t+\t.\tID=gene3;gene_name=Gene3
chr1\t.\tmiRNA\t501\t600\t.\t+\t.\tID=miRNA1;Parent=gene3;miRNA_name=miR-1
";

    fn write_sample_gff() -> Result<File> {
        let mut file = NamedTempFile::new()?;
        file.write_all(SAMPLE_GFF)?;
        file.flush()?;
        Ok(file.reopen()?)
    }

    #[test]
    fn test_gff_to_genes() -> Result<()> {
        let genes = gff_to_genes(GffReader::new(
            BufReader::new(write_sample_gff()?),
            GffType::GFF3,
        ))?;
        dbg!(&genes);

        Ok(())
    }

    #[test]
    fn test_query_builder() -> Result<()> {
        let genes = gff_to_genes(GffReader::new(
            BufReader::new(write_sample_gff()?),
            GffType::GFF3,
        ))?;

        let (gene_query_builder, transcript_query_builder, gff_record_query_builder) =
            build_query_bulders(genes, Uuid::new_v4());

        dbg!(gene_query_builder.sql());
        dbg!(transcript_query_builder.sql());
        Ok(())
    }

    #[sqlx::test]
    async fn test_import(pool: MySqlPool) -> Result<()> {
        let genome_version_id = Uuid::new_v4();
        let annotation_version_id = Uuid::new_v4();
        let organism = Organism::new(
            0,
            "test".to_string(),
            vec![GenomeVersion::new(
                genome_version_id,
                0,
                0,
                0,
                vec![AnnotationVersion::new(annotation_version_id, 0, 0, 0)],
            )],
        );

        let organism_repository = OrganismRepository::new(pool.clone());
        organism_repository.save_organism(&organism).await?;

        from_gff(
            GffReader::new(BufReader::new(write_sample_gff()?), GffType::GFF3),
            annotation_version_id,
            pool,
        )
        .await?;

        Ok(())
    }
}
