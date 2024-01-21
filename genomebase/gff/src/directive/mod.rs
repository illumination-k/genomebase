pub mod standard_directives;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum Standard {
    GffVersion,
    SequenceRegion,
    FeatureOntology,
    AttributeOntology,
    SourceOntology,
    Species,
    GenomeBuild,
    ForwardReferencesAreResolved,
    StartOfFasta,
}

impl AsRef<str> for Standard {
    fn as_ref(&self) -> &str {
        match self {
            Self::GffVersion => "gff-version",
            Self::SequenceRegion => "sequence-region",
            Self::FeatureOntology => "feature-ontology",
            Self::AttributeOntology => "attribute-ontology",
            Self::SourceOntology => "source-ontology",
            Self::Species => "species",
            Self::GenomeBuild => "genome-build",
            Self::ForwardReferencesAreResolved => "#",
            Self::StartOfFasta => "FASTA",
        }
    }
}

impl fmt::Display for Standard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct AttributeDef {
    pub tag_name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum Directive {
    Standard,
    AttributeDef,
    Other(String),
}
