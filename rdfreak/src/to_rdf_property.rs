use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term, Triple};

use crate::ToRdfObject;

/// A trait for serializing a property value into an RDF graph, given a subject and predicate.
pub trait ToRdfProperty: Sized {
    /// Serializes the property value into the given graph, using the provided subject and predicate.
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode);
}

impl<T: ToRdfProperty> ToRdfProperty for Option<T> {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        if let Some(value) = self {
            value.to_property(graph, subject, predicate);
        }
    }
}

impl<T: ToRdfProperty> ToRdfProperty for Vec<T> {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        for item in self {
            item.to_property(graph, subject, predicate);
        }
    }
}

// the name of this is because the types this macro will be used for implement ToRdfObject
macro_rules! impl_to_rdf_property_for_object {
    ($t:ty) => {
        impl ToRdfProperty for $t {
            fn to_property(
                &self,
                graph: &mut Graph,
                subject: &NamedOrBlankNode,
                predicate: &NamedNode,
            ) {
                let object_term = self.to_term(graph);

                graph.insert(&Triple::new(
                    subject.as_ref(),
                    predicate.as_ref(),
                    object_term,
                ));
            }
        }
    };
}

impl_to_rdf_property_for_object!(BlankNode);
impl_to_rdf_property_for_object!(NamedNode);
impl_to_rdf_property_for_object!(NamedOrBlankNode);
impl_to_rdf_property_for_object!(Literal);
impl_to_rdf_property_for_object!(Term);

impl_to_rdf_property_for_object!(bool);

impl_to_rdf_property_for_object!(i8);
impl_to_rdf_property_for_object!(i32);
impl_to_rdf_property_for_object!(i64);

impl_to_rdf_property_for_object!(u8);
impl_to_rdf_property_for_object!(u32);
impl_to_rdf_property_for_object!(u64);

impl_to_rdf_property_for_object!(f32);
impl_to_rdf_property_for_object!(f64);

impl_to_rdf_property_for_object!(String);
