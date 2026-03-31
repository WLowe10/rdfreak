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

    /// Queries the graph for a single entity of type T with the given subject.
    pub async fn query_single<E: Entity + ConstructibleEntity>(
        &self,
        entity_subject: &NamedOrBlankNode,
    ) -> Result<E, QueryError> {
        let mut construct_query_patterns = SparqlConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        E::build_patterns(
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

        let entity = E::deserialize(&result_graph, entity_subject)
            .map_err(QueryError::FailedToDeserializeResult)?;

        Ok(entity)
    }

    /// Inserts the given entity into the graph.
    pub async fn insert<E: Entity + ConstructibleEntity>(
        &self,
        entity: &E,
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

    /// Saves the given entity to the graph.
    ///
    /// This is basically both an insert and a delete (upsert).
    pub async fn save<E: Entity + ConstructibleEntity>(
        &self,
        entity: &E,
    ) -> Result<(), QueryError> {
        // this is basically both an insert and a delete.

        // 1) build the query patterns

        let mut construct_query_patterns = SparqlConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        E::build_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        // 2) serialize the entity to ttl

        let mut entity_graph = Graph::new();

        entity.serialize(&mut entity_graph);

        let mut serializer = TurtleSerializer::new().for_writer(Vec::new());

        for entity_triple in entity_graph.iter() {
            serializer.serialize_triple(entity_triple).unwrap();
        }

        let entity_ttl = String::from_utf8(serializer.finish().unwrap()).unwrap();

        // 3) build the query

        let query = format!(
            "DELETE {{ {patterns} }} INSERT {{ {entity_ttl} }} WHERE {{ VALUES {subject_var} {{ {subject_value} }} {where_patterns} }}",
            subject_var = subject_variable,
            subject_value = entity.get_subject(),
            patterns = construct_query_patterns.patterns,
            where_patterns = construct_query_patterns.where_patterns,
            entity_ttl = entity_ttl,
        );

        // 4) execute the query

        self.graph_db
            .update(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        Ok(())
    }

    /// Deletes the entity with the given subject from the graph.
    pub async fn delete<E: Entity + ConstructibleEntity>(
        &self,
        entity_subject: &NamedOrBlankNode,
    ) -> Result<(), QueryError> {
        let mut construct_query_patterns = SparqlConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        E::build_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        let query = format!(
            "DELETE {{ {patterns} }} WHERE {{ VALUES {subject_var} {{ {subject_value} }} {where_patterns} }}",
            subject_var = subject_variable,
            subject_value = entity_subject,
            patterns = construct_query_patterns.patterns,
            where_patterns = construct_query_patterns.where_patterns,
        );

        self.graph_db
            .update(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        Ok(())
    }
}
