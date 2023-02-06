use std::collections::HashMap;

use anyhow::Result;
use co_expression_domain::{CoExpressionType, Edge, FilterOp, ICoExpressionRepository, Network};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Document},
    Collection, Database,
};

/*
For mongodb
database name -> co-expression
collection name -> taxnomy_id
document -> [
    {
        meta: {},
        edges: [{
            ...
        }]
    },
    ...
]
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoExpressionEdge {
    #[serde(rename = "_id")]
    gene_id: String,
    target: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "_id")]
    gene_id: String,
    corr: f64,
    meta: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    version: String,
    correlation_type: CoExpressionType,
    created_at: DateTime,
    updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrealtionDocument {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    meta: Meta,
    edges: Vec<CoExpressionEdge>,
}

impl CorrealtionDocument {
    pub fn new(meta: Meta, edges: Vec<CoExpressionEdge>) -> Self {
        Self {
            id: None,
            meta,
            edges,
        }
    }
}

pub struct MongoCoExpressionRepository {
    // Database uses std::sync::Arc internally, so it can safely be shared across threads or async tasks.
    // ref: https://docs.rs/mongodb/latest/mongodb/struct.Database.html
    collection: Collection<CorrealtionDocument>,
    version: String,
    coexpression_type: CoExpressionType,
}

impl MongoCoExpressionRepository {
    pub fn new(
        db: &Database,
        taxonomy_id: &str,
        version: String,
        coexpression_type: CoExpressionType,
    ) -> Self {
        Self {
            collection: db.collection(taxonomy_id),
            version,
            coexpression_type,
        }
    }
}

#[async_trait::async_trait]
impl ICoExpressionRepository for MongoCoExpressionRepository {
    async fn dfs(&self, start: &str, depth: usize, filters: &[FilterOp]) -> Result<Network> {
        let mut cursor = self.collection.aggregate(vec![], None).await?;
        unimplemented!()
    }
    async fn retrive_coexpression(&self, id: &str, filters: &[FilterOp]) -> Result<Edge> {
        unimplemented!()
    }
    async fn get_coexpression_network(
        &self,
        ids: &[String],
        filters: &[FilterOp],
    ) -> Result<Network> {
        unimplemented!()
    }
    async fn upsert_network(&self, network: Network) -> Result<()> {
        unimplemented!()
    }
}
