use std::{error::Error, sync::Arc};

use oxrdf::NamedOrBlankNode;
use rdfreak::{
    ConstructibleEntity, DeserializeEntityError, Entity, SparqlConstructQueryPatterns,
    SparqlVariableGenerator,
};

use crate::GraphDatabase;

pub struct QueryClient {
    graph_db: Arc<dyn GraphDatabase>,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("Failed to query graph: {0}")]
    FailedToQueryGraph(Box<dyn Error>),

    #[error("Failed to deserialize result: {0}")]
    FailedToDeserializeResult(DeserializeEntityError),
}

impl QueryClient {
    pub fn new(graph_db: Arc<dyn GraphDatabase>) -> Self {
        Self { graph_db }
    }

    pub async fn query_single<T: Entity + ConstructibleEntity>(
        &self,
        entity_subject: &NamedOrBlankNode,
    ) -> Result<T, QueryError> {
        let mut construct_query_patterns = SparqlConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        T::build_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        let query = format!(
            "CONSTRUCT {{ {0} }} WHERE {{ {1} }}",
            construct_query_patterns.patterns, construct_query_patterns.where_patterns
        );

        let result_graph = self
            .graph_db
            .query_graph(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        let entity = T::deserialize(&result_graph, entity_subject)
            .map_err(QueryError::FailedToDeserializeResult)?;

        Ok(entity)
    }
}
