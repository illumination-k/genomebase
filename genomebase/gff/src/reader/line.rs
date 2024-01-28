use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::{directive::Directive, record::GffRecord};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Line {
    Comment(String),
    Directive(Directive),
    Record(GffRecord),
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            if s.starts_with("##") {
                Ok(Self::Directive(s.parse::<Directive>()?))
            } else {
                Ok(Self::Comment(s.to_string()))
            }
        } else {
            Ok(Self::Record(s.parse::<GffRecord>()?))
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comment(comment) => write!(f, "{}", comment),
            Self::Directive(directive) => write!(f, "{}", directive),
            Self::Record(record) => write!(f, "{}", record),
        }
    }
}
