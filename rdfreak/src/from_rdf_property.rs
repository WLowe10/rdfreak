use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term};

use crate::{FromRdfObject, RdfObjectError};

#[derive(Debug, thiserror::Error)]
pub enum RdfPropertyError {
    #[error("Missing object value for property {0}")]
    MissingObjectValue(NamedNode),

    #[error(transparent)]
    Object(#[from] RdfObjectError),
}

pub type DeserializeRdfPropertyResult<T> = Result<T, RdfPropertyError>;

/// A trait for deserializing a property value from an RDF graph, given a subject and predicate.
pub trait FromRdfProperty: Sized {
    /// Deserializes the property value from the given graph, using the provided subject and predicate.
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self>;
}

impl FromRdfProperty for BlankNode {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl FromRdfProperty for NamedNode {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl FromRdfProperty for NamedOrBlankNode {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl FromRdfProperty for Literal {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl FromRdfProperty for Term {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        Ok(object_term.into())
    }
}

impl FromRdfProperty for String {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(RdfPropertyError::MissingObjectValue(predicate.clone()));
        };

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl<T: FromRdfObject> FromRdfProperty for Option<T> {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Ok(None);
        };

        let object_value = T::from_term(graph, &object_term.into())?;

        Ok(Some(object_value))
    }
}

impl<T: FromRdfObject> FromRdfProperty for Vec<T> {
    fn from_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let object_terms = graph.objects_for_subject_predicate(subject, predicate);

        let mut objects = Vec::new();

        for object_term in object_terms {
            let object = T::from_term(graph, &object_term.into())?;

            objects.push(object);
        }

        Ok(objects)
    }
}
