use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Chromosome {
    Char(char),
    Number(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub struct GenomePosition {
    chromosome: Chromosome,
    pub start: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transcript {
    id: Uuid,
    pub tx_id: String,
    pub gene_id: String,
    pub position: GenomePosition,
    pub cds: Vec<GenomePosition>,
    pub exons: Vec<GenomePosition>,
}

impl Transcript {
    pub fn new(
        tx_id: &str,
        gene_id: &str,
        chromosome: Chromosome,
        start: u64,
        end: u64,
        cds: Vec<GenomePosition>,
        exons: Vec<GenomePosition>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            tx_id: tx_id.to_string(),
            gene_id: gene_id.to_string(),
            position: GenomePosition {
                chromosome,
                start,
                end,
            },
            cds,
            exons,
        }
    }
}
