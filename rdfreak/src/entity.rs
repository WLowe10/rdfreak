use oxrdf::{NamedNode, NamedOrBlankNode};

/// Represents an entity in RDF.
pub trait Entity: Sized {
    /// Returns the rdf type of the entity, which is used to identify the type of the entity in the graph.
    fn get_rdf_type() -> NamedNode;

    /// Returns the subject of the entity.
    fn get_subject(&self) -> &NamedOrBlankNode;
}
