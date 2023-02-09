use crate::{impl_term_serde, term_id_deserializer, term_id_serializer, TermID};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct KogID(String);

impl TermID for KogID {
    fn try_new(id: &str) -> Result<Self> {
        Ok(Self(id.to_string()))
    }

    fn id(&self) -> String {
        self.0.to_owned()
    }
}

impl_term_serde!(KogID);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kog {
    id: KogID,
    category: KogCategory,
    description: String,
}