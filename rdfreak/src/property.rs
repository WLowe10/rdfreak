use oxrdf::{BlankNode, Graph, Literal, NamedNode, NamedOrBlankNode, Term, Triple};

use crate::{
    DeserializeRdfObjectError, RdfObject, SparqlConstructQueryPatterns, SparqlVariableGenerator,
};

#[derive(Debug, thiserror::Error)]
pub enum DeserializeRdfPropertyError {
    #[error("Missing object value for property {0}")]
    MissingObjectValue(NamedNode),

    #[error(transparent)]
    FailedToDeserializeObject(#[from] DeserializeRdfObjectError),
}

pub type DeserializeRdfPropertyResult<T> = Result<T, DeserializeRdfPropertyError>;

/// Represents a predicate-bound relation from an entity's subject to zero or more RDF object terms.
pub trait RdfProperty: Sized {
    /// Serializes the property value into the given graph, using the provided subject and predicate.
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    );

    /// Deserializes the property value from the given graph, using the provided subject and predicate.
    fn deserialize_property(
        graph: &Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self>;
}

pub trait ConstructableRdfProperty {
    fn build_patterns(
        construct_query_patterns: &mut SparqlConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    );
}

// note: lot of repetition here. consider using a macro to generate some of these

impl RdfProperty for BlankNode {
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

impl RdfProperty for NamedNode {
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

impl RdfProperty for NamedOrBlankNode {
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

impl RdfProperty for Literal {
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

impl RdfProperty for Term {
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

impl RdfProperty for String {
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

impl ConstructableRdfProperty for String {
    fn build_patterns(
        construct_query_patterns: &mut SparqlConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    ) {
        let object_variable = variable_generator.next().unwrap();

        let triple_pattern = format!(
            "\t{} {} {} .\n",
            subject_variable, predicate, object_variable
        );

        construct_query_patterns.patterns.push_str(&triple_pattern);

        construct_query_patterns
            .where_patterns
            .push_str(&triple_pattern);
    }
}

impl<T: RdfProperty + RdfObject> RdfProperty for Option<T> {
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

impl<T: ConstructableRdfProperty> ConstructableRdfProperty for Option<T> {
    fn build_patterns(
        construct_query_patterns: &mut SparqlConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    ) {
        construct_query_patterns
            .where_patterns
            .push_str("OPTIONAL {\n");

        T::build_patterns(
            construct_query_patterns,
            variable_generator,
            subject_variable,
            predicate,
        );

        construct_query_patterns.where_patterns.push_str("}\n");
    }
}

impl<T: RdfProperty + RdfObject> RdfProperty for Vec<T> {
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

impl<T: ConstructableRdfProperty> ConstructableRdfProperty for Vec<T> {
    fn build_patterns(
        construct_query_patterns: &mut SparqlConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    ) {
        construct_query_patterns
            .where_patterns
            .push_str("OPTIONAL {\n");

        T::build_patterns(
            construct_query_patterns,
            variable_generator,
            subject_variable,
            predicate,
        );

        construct_query_patterns.where_patterns.push_str("}\n");
    }
}
