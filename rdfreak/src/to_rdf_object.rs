use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term};

use crate::ToRdfLiteral;

/// Represents a type that can be converted to an RDF term that can be used as the object term in a triple.
pub trait ToRdfObject {
    /// Converts the value to an RDF term
    fn to_term(&self, graph: &mut Graph) -> Term;
}

impl ToRdfObject for BlankNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::BlankNode(self.clone())
    }
}

impl ToRdfObject for NamedNode {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::NamedNode(self.clone())
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

impl ToRdfObject for Literal {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        Term::Literal(self.clone())
    }
}

// feels unecessary, but may be needed for consistency reasons
impl ToRdfObject for Term {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        self.clone()
    }
}

macro_rules! impl_to_rdf_object_for_primitive {
    ($t:ty) => {
        impl ToRdfObject for $t {
            fn to_term(&self, _graph: &mut Graph) -> Term {
                Term::Literal(self.to_literal())
            }
        }
    };
}

impl_to_rdf_object_for_primitive!(bool);

impl_to_rdf_object_for_primitive!(i8);
impl_to_rdf_object_for_primitive!(i32);
impl_to_rdf_object_for_primitive!(i64);

impl_to_rdf_object_for_primitive!(u8);
impl_to_rdf_object_for_primitive!(u32);
impl_to_rdf_object_for_primitive!(u64);

impl_to_rdf_object_for_primitive!(f32);
impl_to_rdf_object_for_primitive!(f64);

impl_to_rdf_object_for_primitive!(String);
