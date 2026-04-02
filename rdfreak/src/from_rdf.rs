use oxrdf::{Graph, NamedNode, NamedOrBlankNode};

use crate::RdfPropertyError;

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("Missing rdf:type")]
    MissingRdfType,

    // note: consider whether the values in the error message should be formatted with debug (current) or display
    #[error("Invalid rdf:type: expected {expected:?}, found {found:?}")]
    InvalidRdfType {
        expected: NamedNode,
        found: Vec<NamedNode>,
    },

    #[error("Failed to deserialize property '{property}' for subject {subject:?}: {source}")]
    Property {
        subject: NamedOrBlankNode,
        property: String,
        #[source]
        source: Box<RdfPropertyError>,
    },
}

pub type FromRdfResult<T> = Result<T, ResourceError>;

/// A trait for converting an RDF graph into an instance of a type.
pub trait FromRdf: Sized {
    /// Converts an RDF graph into an instance of the implementing type.
    fn from_rdf(graph: &Graph, subject: &NamedOrBlankNode) -> FromRdfResult<Self>;
}
