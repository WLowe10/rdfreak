use std::ops::Deref;

use oxrdf::{Graph, Literal, NamedNode, NamedOrBlankNode, Term, Triple};
use rdfreak::{
    DeserializeLiteralError, DeserializeLiteralResult, DeserializeRdfObjectError,
    DeserializeRdfObjectResult, DeserializeRdfPropertyError, DeserializeRdfPropertyResult,
    RdfLiteral, RdfObject, RdfProperty,
};

pub struct Date(chrono::NaiveDate);

/// A wrapper around `chrono::NaiveDate` for representing xsd:date literals in RDF.
impl Date {
    pub fn new(date: chrono::NaiveDate) -> Self {
        Self(date)
    }

    pub fn inner(&self) -> chrono::NaiveDate {
        self.0
    }
}

impl Deref for Date {
    type Target = chrono::NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<chrono::NaiveDate> for Date {
    fn from(date: chrono::NaiveDate) -> Self {
        Self::new(date)
    }
}

impl From<Date> for chrono::NaiveDate {
    fn from(date: Date) -> Self {
        date.0
    }
}

impl RdfLiteral for Date {
    fn to_literal(&self) -> Literal {
        let formatted = self.0.format("%Y-%m-%d").to_string();

        Literal::new_typed_literal(
            formatted,
            NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#date"),
        )
    }

    fn from_literal(literal: &Literal) -> DeserializeLiteralResult<Self> {
        if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#date" {
            return Err(DeserializeLiteralError::InvalidDatatype {
                expected: "http://www.w3.org/2001/XMLSchema#date".to_owned(),
                actual: literal.datatype().as_str().to_owned(),
            });
        }

        let chrono_date =
            chrono::NaiveDate::parse_from_str(literal.value(), "%Y-%m-%d").map_err(|err| {
                DeserializeLiteralError::FailedToParse(format!(
                    "Failed to parse '{}' as date: {}",
                    literal.value(),
                    err
                ))
            })?;

        Ok(Self::new(chrono_date))
    }
}

impl RdfObject for Date {
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

impl RdfProperty for Date {
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
