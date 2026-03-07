use oxrdf::{Graph, NamedNode, NamedOrBlankNode, Term, Triple};

use rdfreak::{
    DeserializeRdfObjectResult, DeserializeRdfPropertyError, DeserializeRdfPropertyResult,
    RdfObject, RdfProperty,
};

/// Represents a reference to an RDF object term that can be deserialized into a value of type T.
#[derive(Debug, Clone)]
pub struct ObjectRef<T: RdfObject> {
    object_term: Term,
    _marker: std::marker::PhantomData<T>,
}

impl<T: RdfObject> ObjectRef<T> {
    pub fn new(object_term: Term) -> Self {
        Self {
            object_term,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn deserialize(&self, graph: &Graph) -> DeserializeRdfObjectResult<T> {
        T::from_term(graph, &self.object_term)
    }
}

impl<T: RdfObject> RdfObject for ObjectRef<T> {
    fn to_term(&self) -> Term {
        // ObjectRef serializes to just a term. Could be a named node, blank node, or literal term.
        self.object_term.clone()
    }

    fn from_term(_graph: &Graph, term: &Term) -> DeserializeRdfObjectResult<Self> {
        // object ref deserialization just wraps the term in an ObjectRef struct. The actual deserialization to type T happens when the deserialize method is called on the ObjectRef instance.
        Ok(ObjectRef::new(term.clone()))
    }
}

impl<T: RdfObject> RdfProperty for ObjectRef<T> {
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) {
        graph.insert(&Triple::new(
            subject.clone(),
            predicate.clone(),
            self.object_term.clone(),
        ));
    }

    fn deserialize_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(DeserializeRdfPropertyError::MissingObjectValue(
                predicate.clone(),
            ));
        };

        Ok(ObjectRef::new(object_term.into()))
    }
}
