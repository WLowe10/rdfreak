use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term, Triple};

use crate::ToRdfObject;

/// A trait for serializing a property value into an RDF graph, given a subject and predicate.
pub trait SerializeRdfProperty: Sized {
    /// Serializes the property value into the given graph, using the provided subject and predicate.
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    );
}

// note: lot of repetition here. consider using a macro to generate some of these

impl SerializeRdfProperty for BlankNode {
    fn serialize_property(
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

impl SerializeRdfProperty for NamedNode {
    fn serialize_property(
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

impl SerializeRdfProperty for NamedOrBlankNode {
    fn serialize_property(
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

impl SerializeRdfProperty for Literal {
    fn serialize_property(
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

impl SerializeRdfProperty for Term {
    fn serialize_property(
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

impl SerializeRdfProperty for String {
    fn serialize_property(
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

impl<T: SerializeRdfProperty> SerializeRdfProperty for Option<T> {
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) {
        if let Some(value) = self {
            value.serialize_property(graph, subject, predicate);
        }
    }
}

impl<T: SerializeRdfProperty> SerializeRdfProperty for Vec<T> {
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) {
        for item in self {
            item.serialize_property(graph, subject, predicate);
        }
    }
}
