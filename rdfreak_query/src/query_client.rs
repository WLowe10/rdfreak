use std::{error::Error, sync::Arc};

use oxrdf::{Graph, NamedOrBlankNode};
use oxttl::TurtleSerializer;
use rdfreak::{
    ConstructQueryPatterns, Constructible, DeserializeResourceError, DeserializeResourceResult,
    FromRdf, Resource, SparqlVariableGenerator, ToRdf, TriplePattern, deserialize_all,
};

use crate::GraphDatabase;

pub struct QueryClient {
    graph_db: Arc<dyn GraphDatabase>,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("Failed to query graph: {0}")]
    FailedToQueryGraph(Box<dyn Error>),

    #[error("Failed to deserialize resource: {0}")]
    FailedToDeserializeResource(DeserializeResourceError),
}

fn format_triple_patterns(triples: &[TriplePattern]) -> String {
    triples
        .iter()
        .map(|triple_pattern| format!("{}", triple_pattern,))
        .collect::<Vec<String>>()
        .join(" ")
}

impl QueryClient {
    pub fn new(graph_db: Arc<dyn GraphDatabase>) -> Self {
        Self { graph_db }
    }

    /// Queries the graph for a single resource of type R with the given subject.
    pub async fn query_single<R: Resource + FromRdf + Constructible>(
        &self,
        resource_subject: &NamedOrBlankNode,
    ) -> Result<Option<R>, QueryError> {
        let mut construct_query_patterns = ConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        R::insert_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        let query = format!(
            "CONSTRUCT {{ {template_patterns} }} WHERE {{ VALUES ?{subject_var} {{ {subject_value} }} {where_patterns} }}",
            subject_var = subject_variable,
            subject_value = resource_subject,
            template_patterns = format_triple_patterns(&construct_query_patterns.template_patterns),
            where_patterns = construct_query_patterns.where_pattern
        );

        let result_graph = self
            .graph_db
            .query_graph(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        let resource_result = R::from_rdf(&result_graph, resource_subject);

        match resource_result {
            Ok(resource) => Ok(Some(resource)),
            Err(DeserializeResourceError::InvalidRdfType { .. }) => Ok(None),
            Err(err) => Err(QueryError::FailedToDeserializeResource(err)),
        }
    }

    /// Queries the graph for all resources of type R.
    pub async fn query_all<R: Resource + FromRdf + Constructible>(
        &self,
    ) -> Result<Vec<R>, QueryError> {
        let mut construct_query_patterns = ConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        R::insert_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        let query = format!(
            "CONSTRUCT {{ {template_patterns} }} WHERE {{ {where_patterns} }}",
            template_patterns = format_triple_patterns(&construct_query_patterns.template_patterns),
            where_patterns = construct_query_patterns.where_pattern,
        );

        let result_graph = self
            .graph_db
            .query_graph(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        let entities = rdfreak::deserialize_all::<R>(&result_graph)
            .collect::<DeserializeResourceResult<_>>()
            .map_err(QueryError::FailedToDeserializeResource)?;

        Ok(entities)
    }

    /// Inserts the given resource into the graph.
    pub async fn insert<R: Resource + ToRdf + Constructible>(
        &self,
        resource: &R,
    ) -> Result<(), QueryError> {
        let mut resource_graph = Graph::new();

        resource.to_rdf(&mut resource_graph);

        let mut serializer = TurtleSerializer::new().for_writer(Vec::new());

        for resource_triple in resource_graph.iter() {
            serializer.serialize_triple(resource_triple).unwrap();
        }

        let resource_ttl = String::from_utf8(serializer.finish().unwrap()).unwrap();

        let query = format!("INSERT DATA {{ {0} }}", resource_ttl);

        self.graph_db
            .update(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        Ok(())
    }

    /// Saves the given resource to the graph.
    ///
    /// This is basically both an insert and a delete (upsert).
    pub async fn save<R: Resource + ToRdf + Constructible>(
        &self,
        resource: &R,
    ) -> Result<(), QueryError> {
        // this is basically both an insert and a delete.

        // 1) build the query patterns

        let mut construct_query_patterns = ConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        R::insert_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        // 2) serialize the resource to ttl

        let mut resource_graph = Graph::new();

        resource.to_rdf(&mut resource_graph);

        let mut serializer = TurtleSerializer::new().for_writer(Vec::new());

        for resource_triple in resource_graph.iter() {
            serializer.serialize_triple(resource_triple).unwrap();
        }

        let resource_ttl = String::from_utf8(serializer.finish().unwrap()).unwrap();

        // 3) build the query

        let query = format!(
            "DELETE {{ {template_patterns} }} INSERT {{ {resource_ttl} }} WHERE {{ VALUES ?{subject_var} {{ {subject_value} }} {where_patterns} }}",
            subject_var = subject_variable,
            subject_value = resource.get_subject(),
            template_patterns = format_triple_patterns(&construct_query_patterns.template_patterns),
            where_patterns = construct_query_patterns.where_pattern,
            resource_ttl = resource_ttl,
        );

        // 4) execute the query

        self.graph_db
            .update(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        Ok(())
    }

    /// Deletes the resource with the given subject from the graph.
    pub async fn delete<R: Resource + Constructible>(
        &self,
        resource_subject: &NamedOrBlankNode,
    ) -> Result<(), QueryError> {
        let mut construct_query_patterns = ConstructQueryPatterns::new();
        let mut variable_generator = SparqlVariableGenerator::new();

        let subject_variable = variable_generator.next().unwrap();

        R::insert_patterns(
            &mut construct_query_patterns,
            &mut variable_generator,
            &subject_variable,
        );

        let query = format!(
            "DELETE {{ {template_patterns} }} WHERE {{ VALUES ?{subject_var} {{ {subject_value} }} {where_patterns} }}",
            subject_var = subject_variable,
            subject_value = resource_subject,
            template_patterns = format_triple_patterns(&construct_query_patterns.template_patterns),
            where_patterns = construct_query_patterns.where_pattern,
        );

        self.graph_db
            .update(&query)
            .await
            .map_err(QueryError::FailedToQueryGraph)?;

        Ok(())
    }
}
