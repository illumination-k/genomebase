use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoExpressionType {
    PCC,
    SCC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOp {
    op: Op,
    key: String,
    value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    corr: f64,
    pvalue: f64,
    meta: HashMap<String, f64>,
    targets: Vec<String>,
}

pub type Network = HashMap<String, Edge>;
