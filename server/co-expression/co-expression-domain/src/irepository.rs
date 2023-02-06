use anyhow::Result;

use async_trait::async_trait;

use crate::{Edge, FilterOp, Network};

#[async_trait]
pub trait ICoExpressionRepository {
    async fn dfs(&self, start: &str, depth: usize, filters: &[FilterOp]) -> Result<Network>;
    async fn retrive_coexpression(&self, id: &str, filters: &[FilterOp]) -> Result<Edge>;
    async fn get_coexpression_network(
        &self,
        ids: &[String],
        filters: &[FilterOp],
    ) -> Result<Network>;
    async fn upsert_network(&self, network: Network) -> Result<()>;
}
