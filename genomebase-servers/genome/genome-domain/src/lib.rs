mod go_term;
use anyhow::Result;
use async_trait::async_trait;
pub use go_term::*;

mod kog;
pub use kog::*;

pub mod kegg;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::collections::{HashMap, HashSet};

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
    genome_versions: HashSet<String>,
    annotation_model_versions: HashSet<String>,
}

impl Organism {
    pub fn new(
        taxonomy_id: String,
        name: String,
        genome_versions: HashSet<String>,
        annotation_model_versions: HashSet<String>,
    ) -> Self {
        Self {
            taxonomy_id,
            name,
            genome_versions,
            annotation_model_versions,
        }
    }

    pub fn add_genome_version(&mut self, version: String) {
        self.genome_versions.insert(version);
    }

    pub fn add_annotation_model_version(&mut self, version: String) {
        self.annotation_model_versions.insert(version);
    }
}

#[async_trait]
pub trait IOrganismRepository {
    async fn get_name(&self, taxonomy_id: &str) -> Result<String>;
    async fn retrive(&self, taxonomy_id: &str) -> Option<Organism>;
    async fn list(&self, taxonomy_ids: &[&str]) -> Vec<Organism>;
    async fn upsert(&self, organims: Organism) -> Result<()>;
}

#[async_trait]
pub trait IGenomeRepository {
    async fn upsert_genome(
        &self,
        taxonomy_id: &str,
        genome_version: &str,
        sequence: HashMap<String, String>,
    ) -> Result<()>;
    async fn fetch_genomic_sequence(
        &self,
        taxonomy_id: &str,
        genome_version: &str,
        seqname: &str,
        start: usize,
        end: usize,
    ) -> Result<String>;
}

#[async_trait]
pub trait IAnnotationModelRepository {
    async fn upsert_model(
        &self,
        taxonomy_id: &str,
        model_version: &str,
        genome_version: &str,
    ) -> Result<()>;
    async fn retrive_gene(
        &self,
        taxonomy_id: &str,
        model_version: &str,
        gene_id: &str,
    ) -> Option<Gene>;
    async fn list_genes(
        &self,
        taxonomy_id: &str,
        model_version: &str,
        gene_ids: &[&str],
    ) -> Result<Vec<Gene>>;
    async fn upsert_gene(&self, taxonomy_id: &str, model_version: &str, gene: &Gene) -> Result<()>;
    async fn bulk_upsert_genes(
        &self,
        taxonomy_id: &str,
        model_version: &str,
        genes: &[Gene],
    ) -> Result<()>;
}

pub struct GenomeService<O, A, G> {
    organism_repository: O,
    genome_repository: G,
    annotation_repository: A,
}

impl<O, A, G> GenomeService<O, A, G>
where
    O: IOrganismRepository,
    A: IAnnotationModelRepository,
    G: IGenomeRepository,
{
    pub fn new(organism_repository: O, genome_repository: G, annotation_repository: A) -> Self {
        Self {
            organism_repository,
            genome_repository,
            annotation_repository,
        }
    }

    async fn register_organims(
        &self,
        taxonomy_id: &str,
        genome_version: Option<&str>,
        annotation_version: Option<&str>,
    ) -> Result<()> {
        let mut organims = if let Some(o) = self.organism_repository.retrive(taxonomy_id).await {
            o
        } else {
            Organism::new(
                taxonomy_id.to_string(),
                self.organism_repository.get_name(taxonomy_id).await?,
                HashSet::new(),
                HashSet::new(),
            )
        };

        if let Some(genome_version) = genome_version {
            organims.add_genome_version(genome_version.to_string());
        }

        if let Some(annotation_version) = annotation_version {
            organims.add_annotation_model_version(annotation_version.to_string());
        }

        self.organism_repository.upsert(organims).await?;

        Ok(())
    }

    pub async fn register_full(
        &self,
        taxonomy_id: &str,
        genome_version: &str,
        annotation_version: &str,
        genome: HashMap<String, String>,
        genes: &[Gene],
    ) -> Result<()> {
        self.register_organims(taxonomy_id, Some(genome_version), Some(annotation_version))
            .await?;

        self.genome_repository
            .upsert_genome(taxonomy_id, genome_version, genome)
            .await?;

        self.annotation_repository
            .upsert_model(taxonomy_id, annotation_version, genome_version)
            .await?;
        self.annotation_repository
            .bulk_upsert_genes(taxonomy_id, annotation_version, genes)
            .await?;

        Ok(())
    }

    pub async fn register_genome(
        &self,
        taxonomy_id: &str,
        genome_version: &str,
        genome: HashMap<String, String>,
    ) -> Result<()> {
        self.register_organims(taxonomy_id, Some(genome_version), None)
            .await?;
        self.genome_repository
            .upsert_genome(taxonomy_id, genome_version, genome)
            .await?;

        Ok(())
    }

    pub async fn register_annotation(
        &self,
        taxonomy_id: &str,
        annotation_version: &str,
        genome_version: &str,
        genes: &[Gene],
    ) -> Result<()> {
        self.register_organims(taxonomy_id, None, Some(annotation_version))
            .await?;
        self.annotation_repository
            .upsert_model(taxonomy_id, annotation_version, genome_version)
            .await?;
        self.annotation_repository
            .bulk_upsert_genes(taxonomy_id, annotation_version, genes)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    id: String,
    transcripts: Vec<Transcript>,
    nomenclatures: Vec<Nomenclature>,
    paper: Vec<common::Paper>,
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
    kog: Option<Kog>,
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
