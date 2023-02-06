use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceCode {
    EXP,
    IDA,
    IPI,
    IMP,
    IGI,
    IEP,
    ISS,
    ISO,
    ISA,
    ISM,
    IGC,
    IBA,
    IBD,
    IKR,
    IRD,
    IMR,
    RCA,
    HTP,
    HDA,
    HMP,
    HGI,
    HEP,
    TAS,
    NAS,
    IC,
    ND,
    IEA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCodeMeta {
    code: String,
    id: String,
    category: String,
    description: String,
}

impl EvidenceCodeMeta {
    pub fn new(code: &str, id: &str, category: &str, description: &str) -> Self {
        Self {
            code: code.to_string(),
            id: id.to_string(),
            category: category.to_string(),
            description: description.to_string(),
        }
    }
}

impl EvidenceCode {
    pub fn meta(&self) -> EvidenceCodeMeta {
        match self {
            // Experimental Evidence codes:
            EvidenceCode::EXP => EvidenceCodeMeta::new("EXP", "ECO:0000269", "Experimental", "Inferred from Experiment"),
            EvidenceCode::IDA => EvidenceCodeMeta::new("IDA", "ECO:0000314", "Experimental", "Inferred from Direct Assay"),
            EvidenceCode::IPI => EvidenceCodeMeta::new("IPI", "ECO:0000353", "Experimental", "Inferred from Physical Interaction"),
            EvidenceCode::IMP => EvidenceCodeMeta::new("IMP", "ECO:0000315", "Experimental", "Inferred from Mutant Phenotype"),
            EvidenceCode::IGI => EvidenceCodeMeta::new("IGI", "ECO:0000316", "Experimental", "Inferred from Genetic Interaction"),
            EvidenceCode::IEP => EvidenceCodeMeta::new("IEP", "ECO:0000270", "Experimental", "Inferred from Expression Pattern"),
            // Similarity evidence codes
            EvidenceCode::ISS => EvidenceCodeMeta::new("ISS", "ECO:0000250", "Similarity", "Inferred from Sequence or structural Similarity"),
            EvidenceCode::ISO => EvidenceCodeMeta::new("ISO", "ECO:0000266", "Similarity", "Inferred from Sequence Orthology"),
            EvidenceCode::ISA => EvidenceCodeMeta::new("ISA", "ECO:0000247", "Similarity", "Inferred from Sequence Alignment"),
            EvidenceCode::ISM => EvidenceCodeMeta::new("ISM", "ECO:0000255", "Similarity", "Inferred from Sequence Model used in manual assertion"),
            EvidenceCode::IGC => EvidenceCodeMeta::new("IGC", "ECO:0000317", "Similarity", "Inferred from Genomic Context"),
            EvidenceCode::IBA => EvidenceCodeMeta::new("IBA", "ECO:0000318", "Similarity", "Inferred from Biological aspect of Ancestor"),
            EvidenceCode::IBD => EvidenceCodeMeta::new("IBD", "ECO:0000319", "Similarity", "Inferred from Biological aspect of Descendant"),
            EvidenceCode::IKR => EvidenceCodeMeta::new("IKR", "ECO:0000320", "Similarity", "Inferred from phylogenetic determination of loss of key residues (manual assertion)"),
            EvidenceCode::IRD => EvidenceCodeMeta::new("IRD", "ECO:0000321", "Similarity", "Inferred from Rapid Divergence from ancestral sequence (manual assertion)"),
            EvidenceCode::IMR => EvidenceCodeMeta::new("IMR", "ECO:0000320", "Similarity", "Phylogenetic determination of loss of key residues in manual assertion"),
            // Combinatorial evidence codes
            EvidenceCode::RCA => EvidenceCodeMeta::new("RCA", "ECO:0000245", "Combinatorial", "Inferred from Reviewed Computational Analysis"),
            // High Throughput Experimental evidence codes
            EvidenceCode::HTP => EvidenceCodeMeta::new("HTP", "ECO:0006056", "High_Throughput", "Inferred from High Throughput Experimental"),
            EvidenceCode::HDA => EvidenceCodeMeta::new("HDA", "ECO:0007005", "High_Throughput", "Inferred from High Throughput Direct Assay"),
            EvidenceCode::HMP => EvidenceCodeMeta::new("HMP", "ECO:0007001", "High_Throughput", "Inferred from High Throughput Mutant Phenotype"),
            EvidenceCode::HGI => EvidenceCodeMeta::new("HGI", "ECO:0007003", "High_Throughput", "Inferred from High Throughput Genetic Interaction"),
            EvidenceCode::HEP => EvidenceCodeMeta::new("HEP", "ECO:0007007", "High_Throughput", "Inferred from High Throughput Expression Pattern"),
            // Author Statement evidence codes
            EvidenceCode::TAS => EvidenceCodeMeta::new("TAS", "ECO:0000304", "Author", "Traceable Author Statement used in manual assertion"),
            EvidenceCode::NAS => EvidenceCodeMeta::new("NAS", "ECO:0000303", "Author", "Non-traceable Author Statement used in manual assertion"),
            // Curator Inference
            EvidenceCode::IC => EvidenceCodeMeta::new("IC", "ECO:0000305", "Curatorial", "Inferred by Curator"),
            // No Biological Data
            EvidenceCode::ND => EvidenceCodeMeta::new("ND", "ECO:0000307", "No biological data", "No biological Data available"),
            // Automatic Assertion
            EvidenceCode::IEA => EvidenceCodeMeta::new("IEA", "ECO:0000501", "Automatic", "Inferred from Electronic Annotation")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoTermNamespace {
    BiologicalProcess,
    MolecularFunction,
    CellerComponent,
}

impl ToString for GoTermNamespace {
    fn to_string(&self) -> String {
        match self {
            GoTermNamespace::BiologicalProcess => "BP".to_string(),
            GoTermNamespace::CellerComponent => "CC".to_string(),
            GoTermNamespace::MolecularFunction => "MF".to_string(),
        }
    }
}

pub type GoTermID = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoTerm {
    id: GoTermID,
    name: String,
    namespace: GoTermNamespace,
    def: String,
    is_obsolate: bool,
    xrefs: Vec<String>,
    synonyms: Vec<String>,
    is_a: Vec<GoTermID>,
    relationships: HashMap<String, Vec<GoTermID>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoTermAnnotation {
    evidence_code: EvidenceCode,
    term: GoTerm,
    assinged_by: common::User,
}