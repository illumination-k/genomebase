use std::collections::BTreeSet;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::GeneModel;

use self::transcript::{
    annotation::kog::{Kog, KogID},
    Trasncript,
};

mod transcript;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    id: String,
    gene_model: GeneModel,
    transcripts: Vec<Trasncript>,
}

impl Gene {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn gene_model(&self) -> &GeneModel {
        &self.gene_model
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
