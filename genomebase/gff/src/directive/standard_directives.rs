use crate::Chromosome;
use std::fmt;
use serde::{Deserialize, Serialize};

const DIRECTIVE_PREFIX: &str = "##";

pub trait Directive {
    fn directive_format(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GenomeBuild {
    pub name: String,
    pub source: String,
}

impl fmt::Display for GenomeBuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.source)
    }
}

impl Directive for GenomeBuild {
    fn directive_format(&self) -> String {
        format!("{}genome-build {}", DIRECTIVE_PREFIX, self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SequenceRegion {
    pub seqid: Chromosome,
    pub start: u64,
    pub end: u64,
}

impl fmt::Display for SequenceRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.seqid.to_string(), self.start, self.end)
    }
}

impl Directive for SequenceRegion {
    fn directive_format(&self) -> String {
        format!("{}sequence-region {}", DIRECTIVE_PREFIX, self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GffVersion {
    major: u8,
    minor: Option<u8>,
    patch: Option<u8>,
}

impl GffVersion {
    pub fn new(major: u8, minor: Option<u8>, patch: Option<u8>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for GffVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.major, self.minor, self.patch) {
            (major, None, None) => format!("{}.0.0", major).fmt(f),
            (major, Some(minor), None) => format!("{}.{}.0", major, minor).fmt(f),
            (major, Some(minor), Some(patch)) => format!("{}.{}.{}", major, minor, patch).fmt(f),
            _ => panic!("invalid GFF version"),
        }
    }
}

impl Directive for GffVersion {
    fn directive_format(&self) -> String {
        format!("{}gff-version {}", DIRECTIVE_PREFIX, self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FeatureOntology {
    pub uri: String,
}

impl fmt::Display for FeatureOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uri)
    }
}

impl Directive for FeatureOntology {
    fn directive_format(&self) -> String {
        format!("{}feature-ontology {}", DIRECTIVE_PREFIX, self)
    }
}