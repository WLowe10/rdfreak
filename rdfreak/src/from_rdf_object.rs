use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term};

use crate::{FromRdfLiteral, RdfLiteralError, ResourceError};

#[derive(Debug, thiserror::Error)]
pub enum RdfObjectError {
    #[error("Unexpected term type: {0:?}")]
    UnexpectedTermType(Term),

    #[error(transparent)]
    Literal(#[from] RdfLiteralError),

    #[error(transparent)]
    Resource(#[from] ResourceError),
}

pub type FromRdfObjectResult<T> = Result<T, RdfObjectError>;

/// Represents a type that can be converted to and from an RDF term that can be used as the object term in a triple.
pub trait FromRdfObject: Sized {
    /// Converts an RDF term to the value type, if possible.
    fn from_term(graph: &Graph, term: &Term) -> FromRdfObjectResult<Self>;
}

impl FromRdfObject for BlankNode {
    fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        let Term::BlankNode(blank_node) = term else {
            return Err(RdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(blank_node.clone())
    }
}

impl FromRdfObject for NamedNode {
    fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        let Term::NamedNode(named_node) = term else {
            return Err(RdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(named_node.clone())
    }
}

impl FromRdfObject for NamedOrBlankNode {
    fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        match term {
            Term::NamedNode(named_node) => Ok(NamedOrBlankNode::NamedNode(named_node.clone())),
            Term::BlankNode(blank_node) => Ok(NamedOrBlankNode::BlankNode(blank_node.clone())),
            _ => Err(RdfObjectError::UnexpectedTermType(term.clone())),
        }
    }
}

impl FromRdfObject for Literal {
    fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(RdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(literal.clone())
    }
}

impl FromRdfObject for Term {
    fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        Ok(term.clone())
    }
}

impl<T: FromRdfObject> FromRdfObject for Box<T> {
    fn from_term(graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        let value = T::from_term(graph, term)?;

        Ok(Box::new(value))
    }
}

macro_rules! impl_from_rdf_object_for_primitive {
    ($t:ty) => {
        impl FromRdfObject for $t {
            fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
                let Term::Literal(literal) = term else {
                    return Err(RdfObjectError::UnexpectedTermType(term.clone()));
                };

                let value = Self::from_literal(literal)?;

                Ok(value)
            }
        }
    };
}

impl_from_rdf_object_for_primitive!(bool);

impl_from_rdf_object_for_primitive!(i8);
impl_from_rdf_object_for_primitive!(i32);
impl_from_rdf_object_for_primitive!(i64);

impl_from_rdf_object_for_primitive!(u8);
impl_from_rdf_object_for_primitive!(u32);
impl_from_rdf_object_for_primitive!(u64);

impl_from_rdf_object_for_primitive!(f32);
impl_from_rdf_object_for_primitive!(f64);

impl_from_rdf_object_for_primitive!(String);
