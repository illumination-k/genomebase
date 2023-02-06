mod go_term;

pub use go_term::*;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organism {
    taxonomy_id: String,
    name: String,
    genome: Option<Genome>,
    genes: Vec<Gene>,
}

pub type Genome = BTreeMap<String, Strand>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    id: String,
    transcripts: Vec<Transcript>,
    nomenclatures: Vec<Nomenclature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Strand {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenomicPosition {
    seqname: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TranscriptStructure {
    five_prime_utr: Vec<GenomicPosition>,
    exon: Vec<GenomicPosition>,
    cds: Vec<GenomicPosition>,
    three_prime_utr: Vec<GenomicPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptType {
    MRna,
    RRna,
    MiRNA,
    Transposon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TranscriptGenomeInformation {
    position: GenomicPosition,
    strand: Strand,
    structure: TranscriptStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    id: String,
    gene_id: String,
    type_: TranscriptType,
    // Genomic feature
    genomic_information: Option<TranscriptGenomeInformation>,

    // Annotation
    kog: Option<KOG>,
    kegg_orthology: Option<KeggOrthology>,
    go_terms: Vec<GoTerm>,
    domains: Vec<Domain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nomenclature {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    start: usize,
    end: usize,
    accession: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoTerm {
    accession: String,
    namespace: GoTermNamespace,
    description: String,
    evidence_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KOG {
    accession: String,
    category: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeggOrthology {
    id: String,
    description: String,
}
