pub mod attributes;
pub mod directive;

use std::str::FromStr;

use attributes::{parse_attributes, Attributes};
use serde::{Deserialize, Serialize};

pub(crate) const MISSING_FIELD: &str = ".";
const FIELD_DELIMITER: char = '\t';
const MAX_FIELDS: usize = 9;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Chromosome {
    Char(char),
    Number(u8),
}

impl FromStr for Chromosome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            Ok(Self::Char(s.chars().next().unwrap()))
        } else {
            Ok(Self::Number(s.parse::<u8>().map_err(|e| e.to_string())?))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Strand {
    Forward,
    Reverse,
}

impl AsRef<str> for Strand {
    fn as_ref(&self) -> &str {
        match self {
            Self::Forward => "+",
            Self::Reverse => "-",
        }
    }
}

impl FromStr for Strand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Forward),
            "-" => Ok(Self::Reverse),
            _ => Err(format!("invalid strand: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Zero,
    One,
    Two,
}

impl AsRef<str> for Phase {
    fn as_ref(&self) -> &str {
        match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
        }
    }
}

impl FromStr for Phase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => Err(format!("invalid phase: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GffRecord {
    pub seqid: Chromosome,
    pub source: String,
    pub r#type: String,
    pub start: u32,
    pub end: u32,
    pub score: Option<f64>,
    pub strand: Option<Strand>,
    pub phase: Option<Phase>,
    pub attributes: Attributes,
}

pub fn parse_line(line: &str) -> Result<GffRecord, String> {
    let fields: Vec<&str> = line.split(FIELD_DELIMITER).collect();
    if fields.len() != MAX_FIELDS {
        return Err(format!(
            "expected {} fields, got {}",
            MAX_FIELDS,
            fields.len()
        ));
    }

    let seqid = fields[0].parse::<Chromosome>()?;
    let source = fields[1].to_string();
    let r#type = fields[2].to_string();
    let start = fields[3].parse::<u32>().map_err(|e| e.to_string())?;
    let end = fields[4].parse::<u32>().map_err(|e| e.to_string())?;
    let score = match fields[5] {
        MISSING_FIELD => None,
        _ => Some(fields[5].parse::<f64>().map_err(|e| e.to_string())?),
    };
    let strand = match fields[6] {
        MISSING_FIELD => None,
        _ => Some(fields[6].parse::<Strand>().map_err(|e| e.to_string())?),
    };
    let phase = match fields[7] {
        MISSING_FIELD => None,
        _ => Some(fields[7].parse::<Phase>().map_err(|e| e.to_string())?),
    };
    let attributes = parse_attributes(fields[8]).map_err(|e| e.to_string())?;

    Ok(GffRecord {
        seqid,
        source,
        r#type,
        start,
        end,
        score,
        strand,
        phase,
        attributes,
    })
}
