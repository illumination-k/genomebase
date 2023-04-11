use crate::{impl_term_serde, term_id_deserializer, term_id_serializer, TermID};
use anyhow::Result;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KogID(String);

impl TermID for KogID {
    fn try_new(id: &str) -> Result<Self> {
        Ok(Self(id.to_string()))
    }

    fn id(&self) -> &String {
        &self.0
    }
}

impl_term_serde!(KogID);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum KogCategory {
    A,
    K,
    L,
    B,
    J,
    D,
    Y,
    V,
    T,
    M,
    N,
    Z,
    W,
    U,
    O,
    E,
    F,
    H,
    I,
    G,
    P,
    C,
    Q,
    R,
    S,
    X,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord, new)]
pub struct Kog {
    id: KogID,
    category: KogCategory,
    description: String,
}

impl Kog {
    pub fn id(&self) -> &KogID {
        &self.id
    }

    pub fn category(&self) -> &KogCategory {
        &self.category
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}