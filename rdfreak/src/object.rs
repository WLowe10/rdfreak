use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term};

use crate::{DeserializeLiteralError, DeserializeResourceError, RdfLiteral};

/// Represents a type that can be converted to an RDF term that can be used as the object term in a triple.
pub trait ToRdfObject {
    /// Converts the value to an RDF term
    fn to_term(&self, graph: &mut Graph) -> Term;
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeRdfObjectError {
    #[error("Unexpected term type: {0:?}")]
    UnexpectedTermType(Term),

    #[error(transparent)]
    FailedToDeserializeLiteral(#[from] DeserializeLiteralError),

    #[error(transparent)]
    FailedToDeserializeResource(#[from] DeserializeResourceError),
}

pub type DeserializeRdfObjectResult<T> = Result<T, DeserializeRdfObjectError>;

/// Represents a type that can be converted to and from an RDF term that can be used as the object term in a triple.
pub trait FromRdfObject: Sized {
    /// Converts an RDF term to the value type, if possible.
    fn from_term(graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self>;
}

impl ToRdfObject for BlankNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::BlankNode(self.clone())
    }
}

impl FromRdfObject for BlankNode {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::BlankNode(blank_node) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(blank_node.clone())
    }
}

impl ToRdfObject for NamedNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::NamedNode(self.clone())
    }
}

impl FromRdfObject for NamedNode {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::NamedNode(named_node) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(named_node.clone())
    }
}

impl ToRdfObject for NamedOrBlankNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        match self {
            NamedOrBlankNode::NamedNode(named_node) => Term::NamedNode(named_node.clone()),
            NamedOrBlankNode::BlankNode(blank_node) => Term::BlankNode(blank_node.clone()),
        }
    }
}

impl FromRdfObject for NamedOrBlankNode {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        match term {
            Term::NamedNode(named_node) => Ok(NamedOrBlankNode::NamedNode(named_node.clone())),
            Term::BlankNode(blank_node) => Ok(NamedOrBlankNode::BlankNode(blank_node.clone())),
            _ => Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone())),
        }
    }
}

impl ToRdfObject for Literal {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.clone())
    }
}

impl FromRdfObject for Literal {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(literal.clone())
    }
}

// feels unecessary, but may be needed for consistency reasons
impl ToRdfObject for Term {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        self.clone()
    }
}

impl FromRdfObject for Term {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        Ok(term.clone())
    }
}

impl ToRdfObject for i32 {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.to_literal())
    }
}

impl FromRdfObject for i32 {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        let value = Self::from_literal(literal)?;

        Ok(value)
    }
}

impl ToRdfObject for String {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.to_literal())
    }
}

impl FromRdfObject for String {
    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        let value = Self::from_literal(literal)?;

        Ok(value)
    }
}
