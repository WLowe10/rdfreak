use std::ops::Deref;

use oxrdf::{Graph, Literal, NamedNode, NamedOrBlankNode, Term, Triple};
use rdfreak::{
    DeserializeLiteralError, DeserializeLiteralResult, DeserializeRdfObjectError,
    DeserializeRdfObjectResult, DeserializeRdfPropertyError, DeserializeRdfPropertyResult,
    RdfLiteral, RdfObject, RdfProperty,
};

pub struct DateTime(chrono::NaiveDateTime);

/// A wrapper around `chrono::NaiveDateTime` for representing xsd:dateTime literals in RDF.
impl DateTime {
    pub fn new(date: chrono::NaiveDateTime) -> Self {
        Self(date)
    }

    pub fn inner(&self) -> chrono::NaiveDateTime {
        self.0
    }
}

impl Deref for DateTime {
    type Target = chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(date: chrono::NaiveDateTime) -> Self {
        Self::new(date)
    }
}

impl From<DateTime> for chrono::NaiveDateTime {
    fn from(date: DateTime) -> Self {
        date.0
    }
}

impl RdfLiteral for DateTime {
    fn to_literal(&self) -> Literal {
        let formatted = self.0.format("%Y-%m-%dT%H:%M:%S").to_string();

        Literal::new_typed_literal(
            formatted,
            NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTime"),
        )
    }

    fn from_literal(literal: &Literal) -> DeserializeLiteralResult<Self> {
        if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#dateTime" {
            return Err(DeserializeLiteralError::InvalidDatatype {
                expected: "http://www.w3.org/2001/XMLSchema#dateTime".to_owned(),
                actual: literal.datatype().as_str().to_owned(),
            });
        }

        let chrono_date_time =
            chrono::NaiveDateTime::parse_from_str(literal.value(), "%Y-%m-%dT%H:%M:%S").map_err(
                |err| {
                    DeserializeLiteralError::FailedToParse(format!(
                        "Failed to parse '{}' as date: {}",
                        literal.value(),
                        err
                    ))
                },
            )?;

        Ok(Self::new(chrono_date_time))
    }
}

impl RdfObject for DateTime {
    fn to_term(&self) -> Term {
        Term::Literal(self.to_literal())
    }

    fn from_term(_graph: &Graph, term: &oxrdf::Term) -> DeserializeRdfObjectResult<Self> {
        let oxrdf::Term::Literal(lit) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        let value = Self::from_literal(lit)?;

        Ok(value)
    }
}

impl RdfProperty for DateTime {
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &NamedOrBlankNode,
        predicate: &NamedNode,
    ) {
        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            self.to_term(),
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
