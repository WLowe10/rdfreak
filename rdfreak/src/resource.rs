use oxrdf::{NamedNode, NamedOrBlankNode};

/// Represents a resource in RDF.
pub trait Resource: Sized {
    /// Returns the rdf type of the resource.
    fn get_rdf_type() -> NamedNode;

    /// Returns the subject of the resource.
    fn get_subject(&self) -> &NamedOrBlankNode;
}
