use crate::Chromosome;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt;

const DIRECTIVE_PREFIX: &str = "##";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct GenomeBuild {
    pub name: String,
    pub source: String,
}

impl fmt::Display for GenomeBuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}genome-build {} {}",
            DIRECTIVE_PREFIX, self.name, self.source
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct SequenceRegion {
    pub seqid: Chromosome,
    pub start: u64,
    pub end: u64,
}

impl fmt::Display for SequenceRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}sequence-region{} {} {}",
            DIRECTIVE_PREFIX,
            self.seqid.to_string(),
            self.start,
            self.end
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct GffVersion {
    major: u8,
    minor: Option<u8>,
    patch: Option<u8>,
}

impl fmt::Display for GffVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.major, self.minor, self.patch) {
            (major, None, None) => format!("{}gff-version {}.0.0", DIRECTIVE_PREFIX, major).fmt(f),
            (major, Some(minor), None) => {
                format!("{}gff-version{}.{}.0", DIRECTIVE_PREFIX, major, minor).fmt(f)
            }
            (major, Some(minor), Some(patch)) => format!(
                "{}gff-version{}.{}.{}",
                DIRECTIVE_PREFIX, major, minor, patch
            )
            .fmt(f),
            _ => panic!("invalid GFF version"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct FeatureOntology {
    pub uri: String,
}

impl fmt::Display for FeatureOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}feature-ontology {}", DIRECTIVE_PREFIX, self.uri)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct AttributeOntology {
    pub uri: String,
}

impl fmt::Display for AttributeOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}attribute-ontology {}", DIRECTIVE_PREFIX, self.uri)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct SourceOntology {
    pub uri: String,
}

impl fmt::Display for SourceOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}source-ontology {}", DIRECTIVE_PREFIX, self.uri)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct Species {
    pub taxon_id: u32,
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}species http://www.ncbi.nlm.nih.gov/Taxonomy/Browser/wwwtax.cgi?id={}",
            DIRECTIVE_PREFIX, self.taxon_id
        )
    }
}
