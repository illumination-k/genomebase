use std::collections::BTreeSet;
use uuid::Uuid;

use anyhow::{Result, anyhow};
use derive_new::new;
use serde::{Deserialize, Serialize};

use self::transcript::{
    annotation::kog::{Kog, KogID},
    Transcript,
};

pub mod transcript;

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

impl ToString for Strand {
    fn to_string(&self) -> String {
        match self {
            Self::Plus => "+".to_owned(),
            Self::Minus => "-".to_owned(),
            Self::NoInformation => ".".to_owned(),
        }
    }
}

impl TryFrom<String> for Strand {
    type Error = anyhow::Error;
    fn try_from(strand: String) -> Result<Self> {
        match strand.as_str() {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "." => Ok(Self::NoInformation),
            _ => Err(anyhow!("Strand must be +, - or ., but get {}", strand)),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct Gene {
    id: Uuid,
    gene_id: String,
    start: i32,
    end: i32,
    strand: Strand,
    #[serde(default)]
    transcripts: Vec<Transcript>,
}

impl Gene {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn cloned_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn gene_id(&self) -> &String {
        &self.gene_id
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

    pub fn transcripts(&self) -> &Vec<Transcript> {
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

    pub fn push_transcript(&mut self, transcript: Transcript) {
        self.transcripts.push(transcript);
    }

    pub fn extend_transcripts(&mut self, transcripts: impl IntoIterator<Item = Transcript>) {
        self.transcripts.extend(transcripts.into_iter());
    }
}

#[async_trait::async_trait]
pub trait IGeneRepository {
    async fn retrive_gene(&self, id: &str) -> Result<Gene>;
    async fn list_genes(&self, ids: Option<&[&str]>) -> Result<Vec<Gene>>;
    async fn save_gene(&self, gene: Gene) -> Result<()>;
    async fn delete_gene(&self, id: &str) -> Result<()>;
    async fn count(&self) -> Result<usize>;

    async fn get_kog_annotated_genes(&self, kog_id: &KogID) -> Result<Vec<Gene>>;
    async fn count_kog_annotated_genes(&self, kog_id: &KogID) -> Result<usize>;
}
