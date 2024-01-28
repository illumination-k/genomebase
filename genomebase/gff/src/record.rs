use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::attributes::{parse_attributes, Attributes};
use serde::{Deserialize, Serialize};

pub(crate) const MISSING_FIELD: &str = ".";
const FIELD_DELIMITER: char = '\t';
const MAX_FIELDS: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Serialize for Strand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Strand {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "+" => Ok(Self::Forward),
            "-" => Ok(Self::Reverse),
            _ => Err(serde::de::Error::custom(format!("invalid strand: {}", s))),
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Serialize for Phase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Phase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => Err(serde::de::Error::custom(format!("invalid phase: {}", s))),
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
    pub seqid: String,
    pub source: String,
    pub r#type: String,
    pub start: u32,
    pub end: u32,
    pub score: Option<f64>,
    pub strand: Option<Strand>,
    pub phase: Option<Phase>,
    pub attributes: Attributes,
}

impl Display for GffRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            self.seqid,
            FIELD_DELIMITER,
            self.source,
            FIELD_DELIMITER,
            self.r#type,
            FIELD_DELIMITER,
            self.start,
            FIELD_DELIMITER,
            self.end,
        )?;

        match self.score {
            Some(score) => write!(f, "{}", score)?,
            None => write!(f, "{}", MISSING_FIELD)?,
        }

        write!(f, "{}", FIELD_DELIMITER)?;

        match self.strand {
            Some(strand) => write!(f, "{}", strand.as_ref())?,
            None => write!(f, "{}", MISSING_FIELD)?,
        }

        write!(f, "{}", FIELD_DELIMITER)?;

        match self.phase {
            Some(phase) => write!(f, "{}", phase.as_ref())?,
            None => write!(f, "{}", MISSING_FIELD)?,
        }

        write!(f, "{}", FIELD_DELIMITER)?;

        write!(f, "{:?}", self.attributes)?;

        Ok(())
    }
}

impl FromStr for GffRecord {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        parse_line(line)
    }
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

    let seqid = fields[0].to_string();
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
