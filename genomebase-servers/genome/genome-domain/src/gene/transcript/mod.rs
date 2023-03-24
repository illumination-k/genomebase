pub mod annotation;

use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use self::annotation::FunctionalAnnotation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GffRecord {
    seqname: String,
    source: String,
    #[serde(rename = "type")]
    _type: String,
    start: i32,
    end: i32,
    score: f32,
    strand: Strand,
    phase: char,
    attributes: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum Strand {
    Plus,
    Minus,
    NoInformation,
}

impl Serialize for Strand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Plus => serializer.serialize_char('+'),
            Self::Minus => serializer.serialize_char('-'),
            Self::NoInformation => serializer.serialize_char('.'),
        }
    }
}

impl<'de> Deserialize<'de> for Strand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let result = match s.as_str() {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "." => Ok(Self::NoInformation),
            _ => Err(anyhow!("Strand must be +, - or ., but get {}", s)),
        };

        result.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trasncript {
    id: String,
    gene_id: String,
    transcript_type: String,
    start: i32,
    end: i32,
    strand: Strand,
    structure: Vec<GffRecord>,
    annotation: FunctionalAnnotation,
}

impl Trasncript {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn gene_id(&self) -> &String {
        &self.gene_id
    }

    pub fn transcript_type(&self) -> &String {
        &self.transcript_type
    }

    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn end(&self) -> i32 {
        self.end
    }

    pub fn strand(&self) -> &Strand {
        &self.strand
    }

    pub fn structure(&self) -> &Vec<GffRecord> {
        &self.structure
    }

    pub fn annotation(&self) -> &FunctionalAnnotation {
        &self.annotation
    }
}
