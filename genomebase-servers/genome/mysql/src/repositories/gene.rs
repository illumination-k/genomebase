use std::collections::HashMap;

use anyhow::Result;
use genome_domain::{
    gene::{
        transcript::{
            annotation::{
                kog::{Kog, KogCategory, KogID},
                FunctionalAnnotation,
            },
            Strand, Transcript,
        },
        Gene, IGeneRepository,
    },
    TermID,
};
use sqlx::{query, query_as, MySqlPool};
use uuid::Uuid;

pub struct GeneRepository {
    pool: MySqlPool,
    annotation_version_id: Uuid,
}

impl GeneRepository {
    pub fn new(pool: MySqlPool, annotation_version_id: Uuid) -> Self {
        Self {
            pool,
            annotation_version_id,
        }
    }
}

#[derive(Debug)]
struct GeneTranscriptInfo {
    gene_id: Vec<u8>,
    gene_gene_id: String,
    transcript_id: Vec<u8>,
    transcript_transcript_id: String,
    transcript_transcript_type: String,
    transcript_strand: String,
    transcript_start: i32,
    transcript_end: i32,
    kog_id: Option<String>,
    kog_category: Option<String>,
    kog_description: Option<String>,
    go_evidence_code: Option<String>,
    go_id: Option<String>,
    go_name: Option<String>,
    go_namespace: Option<String>,
    domain_start: Option<i32>,
    domain_end: Option<i32>,
    domain_id: Option<String>,
    domain_name: Option<String>,
}

#[async_trait::async_trait]
impl IGeneRepository for GeneRepository {
    async fn retrive_gene(&self, id: &str) -> Result<Gene> {
        let records = query_as!(
            GeneTranscriptInfo,
            r#"
            SELECT
                g.id as gene_id,
                g.gene_id as gene_gene_id,
                t.id as transcript_id,
                t.transcript_id as transcript_transcript_id,
                t.transcript_type as transcript_transcript_type,
                t.strand as transcript_strand,
                t.start as transcript_start,
                t.end as transcript_end,
                kogs.id as kog_id,
                kogs.category as kog_category,
                kogs.description as kog_description,
                goa.evidence_code as go_evidence_code,
                goterm.id as go_id,
                goterm.name as go_name,
                goterm.namespace as go_namespace,
                da.start as domain_start,
                da.end as domain_end,
                domain.id as domain_id,
                domain.name as domain_name
            FROM
                genes g
            INNER JOIN
                transcripts t ON t.gene_id = g.id AND g.gene_id = ? AND g.annotation_version_id = ?
            LEFT JOIN
                kog_annotations k ON k.transcript_id = t.id
            LEFT JOIN
                kogs kogs ON kogs.id = k.kog_id
            LEFT JOIN
                go_terms_annotation goa ON goa.transcript_id = t.id
            LEFT JOIN
                go_terms goterm ON goterm.id = goa.go_term_id
            LEFT JOIN
                domain_annotations da ON da.transcript_id = t.id
            LEFT JOIN
                domains domain ON domain.id = da.domain_id
            "#,
            id,
            self.annotation_version_id.as_bytes().as_slice()
        )
        .fetch_all(&self.pool)
        .await?;

        let mut gene: Option<Gene> = None;
        let mut transcripts: HashMap<Uuid, Transcript> = HashMap::new();

        for record in records.into_iter() {
            if gene.is_none() {
                gene = Some(Gene::new(
                    Uuid::from_slice(&record.gene_id)?,
                    record.gene_gene_id,
                    vec![],
                ));
            }

            let transcript_id = Uuid::from_slice(&record.transcript_id)?;
            let cur_transcript = transcripts.entry(transcript_id).or_insert(Transcript::new(
                transcript_id,
                record.transcript_transcript_id,
                record.transcript_transcript_type,
                record.transcript_start,
                record.transcript_start,
                Strand::Minus,
                vec![],
                FunctionalAnnotation::new(None),
            ));

            if let (Some(kog_id), Some(kog_category), Some(kog_description)) =
                (record.kog_id, record.kog_category, record.kog_description)
            {
                cur_transcript.annotation_mut().kog = Some(Kog::new(
                    KogID::try_new(&kog_id)?,
                    KogCategory::A,
                    kog_description,
                ));
            }

            if let (
                Some(go_term_id),
                Some(go_term_name),
                Some(go_term_namespace),
                Some(go_evidence_code),
            ) = (
                record.go_id,
                record.go_name,
                record.go_namespace,
                record.go_evidence_code,
            ) {
                todo!()
            }

            if let (Some(domain_start), Some(domain_end), Some(domain_id), Some(domain_name)) = (
                record.domain_start,
                record.domain_end,
                record.domain_id,
                record.domain_name,
            ) {
                todo!()
            }
        }

        if let Some(mut gene) = gene {
            gene.extend_transcripts(transcripts.into_values());
            Ok(gene)
        } else {
            Err(anyhow::anyhow!("Gene not found"))
        }
    }

    async fn list_genes(&self, ids: Option<&[&str]>) -> Result<Vec<Gene>> {
        todo!()
    }

    async fn save_gene(&self, gene: Gene) -> Result<()> {
        todo!()
    }
    async fn delete_gene(&self, id: &str) -> Result<()> {
        todo!()
    }

    async fn count(&self) -> Result<usize> {
        let rec = query!(
            r#"
            SELECT
                COUNT(id) as count
            FROM
                genes
            WHERE
                annotation_version_id = ?
            "#,
            self.annotation_version_id.as_bytes().as_slice()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.count as usize)
    }

    async fn get_kog_annotated_genes(&self, kog_id: &KogID) -> Result<Vec<Gene>> {
        todo!()
    }

    async fn count_kog_annotated_genes(&self, kog_id: &KogID) -> Result<usize> {
        todo!()
    }
}
