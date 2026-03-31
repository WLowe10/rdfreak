use std::{error::Error, sync::Arc};

use oxrdf::{Graph, NamedOrBlankNode};
use oxttl::TurtleSerializer;
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
            "CONSTRUCT {{ {patterns} }} WHERE {{ VALUES {subject_var} {{ {subject_value} }} {where_patterns} }}",
            subject_var = subject_variable,
            subject_value = entity_subject,
            patterns = construct_query_patterns.patterns,
            where_patterns = construct_query_patterns.where_patterns,
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

    pub async fn insert<T: Entity + ConstructibleEntity>(
        &self,
        entity: &T,
    ) -> Result<(), QueryError> {
        let mut entity_graph = Graph::new();

        entity.serialize(&mut entity_graph);

        let mut serializer = TurtleSerializer::new().for_writer(Vec::new());

        for entity_triple in entity_graph.iter() {
            serializer.serialize_triple(entity_triple).unwrap();
        }

        let entity_ttl = String::from_utf8(serializer.finish().unwrap()).unwrap();

        let query = format!("INSERT DATA {{ {0} }}", entity_ttl);

        self.graph_db
            .update(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        Ok(())
    }
}
