use oxrdf::{Graph, NamedNode, NamedOrBlankNode, Term, Triple};

use rdfreak::{
    FromRdfObject, FromRdfObjectResult, FromRdfProperty, FromRdfPropertyResult, RdfPropertyError,
    ToRdfObject, ToRdfProperty,
};

/// Represents a reference to an RDF object term that can be deserialized into a value of type T.
#[derive(Debug, Clone)]
pub struct ObjectRef<T> {
    object_term: Term,
    _marker: std::marker::PhantomData<T>,
}

impl<T> ObjectRef<T> {
    pub fn new(object_term: Term) -> Self {
        Self {
            object_term,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: FromRdfObject> ObjectRef<T> {
    pub fn deserialize(&self, graph: &Graph) -> FromRdfObjectResult<T> {
        T::from_term(graph, &self.object_term)
    }
}

impl<T: ToRdfObject> ToRdfObject for ObjectRef<T> {
    fn to_term(&self, _graph: &mut Graph) -> Term {
        // ObjectRef serializes to just a term. Could be a named node, blank node, or literal term.
        self.object_term.clone()
    }
}

impl<T: FromRdfObject> FromRdfObject for ObjectRef<T> {
    fn from_term(_graph: &Graph, term: &Term) -> FromRdfObjectResult<Self> {
        // when deserializing an ObjectRef, we just wrap the term in an ObjectRef struct. The actual deserialization to type T happens when the deserialize method is called on the ObjectRef instance.
        Ok(ObjectRef::new(term.clone()))
    }
}

impl<T> ToRdfProperty for ObjectRef<T> {
    fn to_property(&self, graph: &mut Graph, subject: &NamedOrBlankNode, predicate: &NamedNode) {
        graph.insert(&Triple::new(
            subject.clone(),
            predicate.clone(),
            self.object_term.clone(),
        ));
    }
}

impl<T> FromRdfProperty for ObjectRef<T> {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> FromRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        Ok(ObjectRef::new(object_term.into()))
    }
}
