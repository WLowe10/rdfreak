use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term};

use crate::{DeserializeRdfObjectError, FromRdfObject};

#[derive(Debug, thiserror::Error)]
pub enum DeserializeRdfPropertyError {
    #[error("Missing object value for property {0}")]
    MissingObjectValue(NamedNode),

    #[error(transparent)]
    FailedToDeserializeObject(#[from] DeserializeRdfObjectError),
}

pub type DeserializeRdfPropertyResult<T> = Result<T, DeserializeRdfPropertyError>;

/// A trait for deserializing a property value from an RDF graph, given a subject and predicate.
pub trait DeserializeRdfProperty: Sized {
    /// Deserializes the property value from the given graph, using the provided subject and predicate.
    fn deserialize_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self>;
}

impl DeserializeRdfProperty for BlankNode {
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

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl DeserializeRdfProperty for NamedNode {
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

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl DeserializeRdfProperty for NamedOrBlankNode {
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

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl DeserializeRdfProperty for Literal {
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

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl DeserializeRdfProperty for Term {
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

        Ok(object_term.into())
    }
}

impl DeserializeRdfProperty for String {
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

        let object_value = Self::from_term(graph, &object_term.into())?;

        Ok(object_value)
    }
}

impl<T: FromRdfObject> DeserializeRdfProperty for Option<T> {
    fn deserialize_property(
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

impl<T: FromRdfObject> DeserializeRdfProperty for Vec<T> {
    fn deserialize_property(
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
