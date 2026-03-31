use oxrdf::{Graph, NamedNode, NamedOrBlankNode, Triple};

use crate::{
    DeserializeRdfPropertyError, RdfProperty, RdfType, SparqlConstructQueryPatterns,
    SparqlVariableGenerator,
};

#[derive(Debug, thiserror::Error)]
pub enum DeserializeEntityError {
    #[error("Missing rdf:type")]
    MissingRdfType,

    // note: consider whether the values in the error message should be formatted with debug (current) or display
    #[error("Invalid rdf:type: expected {expected:?}, found {found:?}")]
    InvalidRdfType {
        expected: NamedNode,
        found: Vec<NamedNode>,
    },

    #[error("Failed to deserialize property '{property}' for subject {subject:?}: {source}")]
    FailedToDeserializeProperty {
        subject: NamedOrBlankNode,
        property: String,
        #[source]
        source: Box<DeserializeRdfPropertyError>,
    },
}

pub type DeserializeEntityResult<T> = Result<T, DeserializeEntityError>;

/// Represents an entity in RDF.
pub trait Entity: Sized {
    // question: should this return Vec<NamedNode>?
    /// Returns the rdf type of the entity, which is used to identify the type of the entity in the graph.
    fn get_rdf_type() -> NamedNode;

    /// Serializes the properties of the entity into the given graph.
    fn serialize_properties(&self, graph: &mut Graph);

    /// Serializes the entity into the given graph.
    fn serialize(&self, graph: &mut Graph) {
        self.serialize_properties(graph);

        graph.insert(&Triple::new(
            self.get_subject().as_ref(),
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Self::get_rdf_type().as_ref(),
        ));
    }

    /// Deserializes the entity from the given graph and subject.
    fn deserialize_properties(
        graph: &Graph,
        subject: &NamedOrBlankNode,
    ) -> DeserializeEntityResult<Self>;

    /// Deserializes the entity from the given graph and subject.
    fn deserialize(graph: &Graph, subject: &NamedOrBlankNode) -> DeserializeEntityResult<Self> {
        let expected_rdf_type = Self::get_rdf_type();

        let rdf_types = Vec::<RdfType>::deserialize_property(
            graph,
            subject,
            &NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
        )
        .map_err(|err| DeserializeEntityError::FailedToDeserializeProperty {
            subject: subject.clone(),
            property: "rdf:type".to_string(),
            source: Box::new(err),
        })?;

        let has_expected_rdf_type = rdf_types
            .iter()
            .any(|rdf_type| rdf_type.get_named_node() == &expected_rdf_type);

        if !has_expected_rdf_type {
            return Err(DeserializeEntityError::InvalidRdfType {
                expected: expected_rdf_type,
                found: rdf_types
                    .into_iter()
                    .map(|rdf_type| rdf_type.get_named_node().clone())
                    .collect(),
            });
        }

        Self::deserialize_properties(graph, subject)
    }

    fn deserialize_all(graph: &Graph) -> DeserializeEntityResult<Vec<Self>> {
        let mut entities = Vec::new();

        let entity_subjects = graph.subjects_for_predicate_object(
            &NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Self::get_rdf_type().as_ref(),
        );

        for entity_subject in entity_subjects {
            let entity = Self::deserialize(graph, &entity_subject.into_owned())?;

            entities.push(entity);
        }

        Ok(entities)
    }

    /// Returns the subject of the entity.
    fn get_subject(&self) -> &NamedOrBlankNode;
}

pub trait ConstructibleEntity: Entity {
    fn build_property_patterns(
        construct_query_patterns: &mut SparqlConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
    );

    fn build_patterns(
        construct_query_patterns: &mut SparqlConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
    ) {
        let rdf_type_triple_pattern = format!(
            "\t{} {} {} .\n",
            subject_variable,
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Self::get_rdf_type()
        );

        construct_query_patterns
            .patterns
            .push_str(&rdf_type_triple_pattern);

        construct_query_patterns
            .where_patterns
            .push_str(&rdf_type_triple_pattern);

        Self::build_property_patterns(
            construct_query_patterns,
            variable_generator,
            subject_variable,
        );
    }
}
