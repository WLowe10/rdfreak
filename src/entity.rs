use oxrdf::{Graph, NamedNode};

use crate::DeserializeLiteralError;

#[derive(Debug)]
pub enum EntityPropertyDefinitionKind {
    Literal,
    Entity(Vec<EntityPropertyDefinition>),
}

#[derive(Debug)]
pub struct EntityPropertyDefinition {
    kind: EntityPropertyDefinitionKind,
    name: NamedNode,
    predicate: NamedNode,
    is_optional: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeEntityError {
    #[error("Missing property: {0}")]
    MissingProperty(String),

    #[error(transparent)]
    FailedToDeserializeLiteral(#[from] DeserializeLiteralError),
}

pub type DeserializeEntityResult<T> = Result<T, DeserializeEntityError>;

/// A trait representing an entity that can be serialized and deserialized to and from RDF graphs.
pub trait Entity: Sized {
    // returns the rdf typ of the entity, which is used to identify the type of the entity in the graph.
    fn get_rdf_type() -> &'static NamedNode;

    /// returns a list of property definitions for the entity type.
    fn get_property_definitions() -> Vec<EntityPropertyDefinition>;

    /// serializes the entity into the given graph.
    fn serialize(&self, graph: &mut Graph);

    /// deserializes the entity from the given graph and IRI.
    fn deserialize(graph: &Graph, iri: &NamedNode) -> DeserializeEntityResult<Self>;

    /// returns the IRI of the entity.
    fn get_iri(&self) -> &NamedNode;
}
