mod go_term;
use anyhow::Result;
use async_trait::async_trait;
pub use go_term::*;

mod kog;
pub use kog::*;

pub mod kegg;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::collections::HashMap;

pub trait TermID {
    fn try_new(id: &str) -> Result<Self>
    where
        Self: Sized;
    fn id(&self) -> String;
}

pub fn term_id_serializer<S: Serializer, T: TermID>(term: &T, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&term.id())
}

pub fn term_id_deserializer<'de, D: Deserializer<'de>, T: TermID>(d: D) -> Result<T, D::Error> {
    let id = String::deserialize(d)?;
    T::try_new(&id).map_err(serde::de::Error::custom)
}

#[macro_export]
macro_rules! impl_term_serde {
    ($term_name: ident) => {
        impl Serialize for $term_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                term_id_serializer(self, serializer)
            }
        }

        impl<'de> Deserialize<'de> for $term_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                term_id_deserializer(deserializer)
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organism {
    taxonomy_id: String,
    name: String,
    genome_versions: Vec<String>,
    annotation_versions: Vec<String>,
}

pub struct GenomeService {
    organism: Organism,
}

impl GenomeService {
    pub fn new(organism: Organism) -> Self {
        Self { organism }
    }

    pub fn fetch_sequence(genome_version: &str) {}
    pub fn retrive_gene(annotaion_version: &str) {}
    pub fn list_genes(annotaion_version: &str) {}
}

#[async_trait]
pub trait IGenomeRepository {
    async fn upsert_genome(
        taxonomy_id: &str,
        genome_version: &str,
        sequence: HashMap<String, String>,
    ) -> Result<()>;
    async fn fetch_genomic_sequence(
        taxonomy_id: &str,
        genome_version: &str,
        seqname: &str,
        start: usize,
        end: usize,
    ) -> Result<String>;
}

#[async_trait]
pub trait IAnnotationModelCRUDRepository {
    async fn retrive_gene(taxonomy_id: &str, model_version: &str, gene_id: &str) -> Result<Gene>;
    async fn list_genes(
        taxonomy_id: &str,
        model_version: &str,
        gene_ids: &[&str],
    ) -> Result<Vec<Gene>>;
    async fn upsert_gene(taxonomy_id: &str, model_version: &str, gene: Gene) -> Result<()>;
    async fn bulk_upsert_genes(
        taxonomy_id: &str,
        model_version: &str,
        genes: Vec<Gene>,
    ) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    id: String,
    transcripts: Vec<Transcript>,
    nomenclatures: Vec<Nomenclature>,
    other_model_ids: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Strand {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptType {
    MRna,
    RRna,
    MiRNA,
    Transposon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    id: String,
    gene_id: String,
    type_: TranscriptType,

    // Annotation
    kog: Option<KOG>,
    kegg: Option<kegg::Annotation>,
    go_terms: Vec<GoTermAnnotation>,
    domains: Vec<DomainAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nomenclature {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAnnotation {
    start: usize,
    end: usize,
    accession: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KOG {
    accession: String,
    category: String,
    description: String,
}
