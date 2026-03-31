use std::error::Error;

use async_trait::async_trait;
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use oxrdf::Graph;
use rdfreak_query::GraphDatabase;

pub struct OxigraphGraphDatabase {
    store: oxigraph::store::Store,
}

#[async_trait]
impl GraphDatabase for OxigraphGraphDatabase {
    async fn query_graph(&self, query: &str) -> Result<Graph, Box<dyn Error>> {
        let results = SparqlEvaluator::new()
            .parse_query(query)
            .unwrap()
            .on_store(&self.store)
            .execute()?;

        let QueryResults::Graph(triples) = results else {
            return Err("Expected graph results".into());
        };

        let mut result_graph = Graph::new();

        for maybe_triple in triples {
            let triple = maybe_triple.unwrap();

            result_graph.insert(&triple);
        }

        Ok(result_graph)
    }
}
