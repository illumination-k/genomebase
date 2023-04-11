use std::collections::BTreeSet;
use uuid::Uuid;

use anyhow::Result;
use derive_new::new;
use serde::{Deserialize, Serialize};

use self::transcript::{
    annotation::kog::{Kog, KogID},
    Trasncript,
};

pub mod transcript;

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct Gene {
    id: Uuid,
    gene_id: String,
    transcripts: Vec<Trasncript>,
}

impl Gene {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn gene_id(&self) -> &String {
        &self.gene_id
    }

    pub fn transcripts(&self) -> &Vec<Trasncript> {
        &self.transcripts
    }

    pub fn kogs(&self) -> Vec<&Kog> {
        let bset: BTreeSet<_> = self
            .transcripts
            .iter()
            .filter_map(|t| t.annotation().kog().as_ref())
            .collect();

        bset.into_iter().collect()
    }

    pub fn push_transcript(&mut self, transcript: Trasncript) {
        self.transcripts.push(transcript);
    }
}

#[async_trait::async_trait]
pub trait IGeneRepository {
    async fn retrive_gene(&self, id: &str) -> Result<Option<Gene>>;
    async fn list_genes(&self, ids: Option<&[&str]>) -> Result<Vec<Gene>>;
    async fn save_gene(&self, gene: Gene) -> Result<()>;
    async fn delete_gene(&self, id: &str) -> Result<()>;
    async fn count(&self) -> Result<usize>;

    async fn get_kog_annotated_genes(&self, kog_id: &KogID) -> Result<Vec<Gene>>;
    async fn count_kog_annotated_genes(&self, kog_id: &KogID) -> Result<usize>;
}
