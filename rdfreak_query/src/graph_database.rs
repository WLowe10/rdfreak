use std::error::Error;

use async_trait::async_trait;
use oxrdf::Graph;

#[async_trait]
pub trait GraphDatabase {
    /// Executes a SPARQL construct query and returns the resulting graph.
    async fn query_graph(&self, query: &str) -> Result<Graph, Box<dyn Error>>;
}
