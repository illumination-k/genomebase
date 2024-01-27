use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

const DELIMITER: char = ',';

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Array(Vec<String>),
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(DELIMITER) {
            let array = s.split(DELIMITER).map(|s| s.to_string()).collect();
            Ok(Value::Array(array))
        } else {
            Ok(Value::String(s.to_string()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Id,
    Name,
    Alias,
    Parent,
    Target,
    Gap,
    DerivesFrom,
    Note,
    Dbxref,
    OntologyTerm,
    IsCircular,
    Other(String),
}

pub type Attributes = IndexMap<Tag, Value>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParseAttibuteError {
    MissingTag(String),
    MissingValue(String),
    InvalidValue(String),
}

impl Display for ParseAttibuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTag(tag) => write!(f, "missing tag: {}", tag),
            Self::MissingValue(tag) => write!(f, "missing value for tag: {}", tag),
            Self::InvalidValue(value) => write!(f, "invalid value: {}", value),
        }
    }
}

impl Error for ParseAttibuteError {}

pub fn parse_attributes(attributes: &str) -> Result<Attributes, ParseAttibuteError> {
    let mut map = IndexMap::new();

    for attribute in attributes.split(';') {
        let mut parts = attribute.split('=');
        let tag = parts
            .next()
            .ok_or_else(|| ParseAttibuteError::MissingTag(attribute.to_string()))?;
        let value = parts
            .next()
            .ok_or_else(|| ParseAttibuteError::MissingValue(attribute.to_string()))?;
        let tag = match tag {
            "ID" => Tag::Id,
            "Name" => Tag::Name,
            "Alias" => Tag::Alias,
            "Parent" => Tag::Parent,
            "Target" => Tag::Target,
            "Gap" => Tag::Gap,
            "Derives_from" => Tag::DerivesFrom,
            "Note" => Tag::Note,
            "Dbxref" => Tag::Dbxref,
            "Ontology_term" => Tag::OntologyTerm,
            "Is_circular" => Tag::IsCircular,
            _ => Tag::Other(tag.to_string()),
        };
        let value = value
            .parse::<Value>()
            .map_err(|_| ParseAttibuteError::InvalidValue(value.to_string()))?;
        map.insert(tag, value);
    }

    Ok(map)
}
