use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term, Triple};

use crate::ToRdfObject;

/// A trait for serializing a property value into an RDF graph, given a subject and predicate.
pub trait ToRdfProperty: Sized {
    /// Serializes the property value into the given graph, using the provided subject and predicate.
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode);
}

// note: lot of repetition here. consider using a macro to generate some of these

impl ToRdfProperty for BlankNode {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        let object_term = self.to_term(graph);

        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            object_term,
        ));
    }
}

impl ToRdfProperty for NamedNode {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        let object_term = self.to_term(graph);

        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            object_term,
        ));
    }
}

impl ToRdfProperty for NamedOrBlankNode {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        let object_term = self.to_term(graph);

        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            object_term,
        ));
    }
}

impl ToRdfProperty for Literal {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        let object_term = self.to_term(graph);

        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            object_term,
        ));
    }
}

impl ToRdfProperty for Term {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        let object_term = self.to_term(graph);

        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            object_term,
        ));
    }
}

impl ToRdfProperty for String {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        let object_term = self.to_term(graph);

        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            object_term,
        ));
    }
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
