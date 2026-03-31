use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term};

use crate::{DeserializeEntityError, DeserializeLiteralError, RdfLiteral};

#[derive(Debug, thiserror::Error)]
pub enum DeserializeRdfObjectError {
    #[error("Unexpected term type: {0:?}")]
    UnexpectedTermType(Term),

    #[error(transparent)]
    FailedToDeserializeLiteral(#[from] DeserializeLiteralError),

    #[error(transparent)]
    FailedToDeserializeEntity(#[from] DeserializeEntityError),
}

pub type DeserializeRdfObjectResult<T> = Result<T, DeserializeRdfObjectError>;

/// Represents a type that can be converted to and from an RDF term that can be used as the object term in a triple.
pub trait RdfObject: Sized {
    /// Converts the value to an RDF term that can be inserted into a triple as the object term.
    fn to_term(&self, graph: &mut Graph) -> Term;

    /// Converts an RDF term to the value type, if possible.
    fn from_term(graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self>;
}

impl RdfObject for BlankNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::BlankNode(self.clone())
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::BlankNode(blank_node) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(blank_node.clone())
    }
}

impl RdfObject for NamedNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::NamedNode(self.clone())
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::NamedNode(named_node) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(named_node.clone())
    }
}

impl RdfObject for NamedOrBlankNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        match self {
            NamedOrBlankNode::NamedNode(named_node) => Term::NamedNode(named_node.clone()),
            NamedOrBlankNode::BlankNode(blank_node) => Term::BlankNode(blank_node.clone()),
        }
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        match term {
            Term::NamedNode(named_node) => Ok(NamedOrBlankNode::NamedNode(named_node.clone())),
            Term::BlankNode(blank_node) => Ok(NamedOrBlankNode::BlankNode(blank_node.clone())),
            _ => Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone())),
        }
    }
}

impl RdfObject for Literal {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.clone())
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(literal.clone())
    }
}

// feels unecessary, but may be needed for consistency reasons
impl RdfObject for Term {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        self.clone()
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        Ok(term.clone())
    }
}

impl RdfObject for i32 {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.to_literal())
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        let value = Self::from_literal(literal)?;

        Ok(value)
    }
}

impl RdfObject for String {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.to_literal())
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        let Term::Literal(literal) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        let value = Self::from_literal(literal)?;

        Ok(value)
    }
}
