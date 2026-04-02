use std::{fmt::Display, ops::Deref, str::FromStr};

use rdfreak_derive::RdfLiteral;

/// A wrapper around `chrono::NaiveDateTime` for representing xsd:dateTime literals in RDF.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, RdfLiteral)]
#[rdf(datatype = "http://www.w3.org/2001/XMLSchema#dateTime")]
pub struct NaiveDateTime(chrono::NaiveDateTime);

impl NaiveDateTime {
    pub fn new(date: chrono::NaiveDateTime) -> Self {
        Self(date)
    }

    pub fn inner(&self) -> chrono::NaiveDateTime {
        self.0
    }
}

impl Display for NaiveDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S"))
    }
}

impl FromStr for NaiveDateTime {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_time = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")?;

        Ok(Self::new(date_time))
    }
}

impl Deref for NaiveDateTime {
    type Target = chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<chrono::NaiveDateTime> for NaiveDateTime {
    fn from(date: chrono::NaiveDateTime) -> Self {
        Self::new(date)
    }
}

impl From<NaiveDateTime> for chrono::NaiveDateTime {
    fn from(date: NaiveDateTime) -> Self {
        date.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use oxrdf::{Literal, NamedNode};
    use rdfreak::{FromRdfLiteral, ToRdfLiteral};

    #[test]
    fn test_to_literal() {
        let date_time = NaiveDateTime::from_str("2024-01-01T12:00:00").unwrap();
        let date_time_literal = date_time.to_literal();

        let expected_literal = Literal::new_typed_literal(
            "2024-01-01T12:00:00",
            NamedNode::new("http://www.w3.org/2001/XMLSchema#dateTime").unwrap(),
        );

        assert_eq!(date_time_literal, expected_literal);
    }

    #[test]
    fn test_from_literal() {
        let date_time_literal = Literal::new_typed_literal(
            "2024-01-01T12:00:00".to_owned(),
            NamedNode::new("http://www.w3.org/2001/XMLSchema#dateTime").unwrap(),
        );

        let date_time = NaiveDateTime::from_literal(&date_time_literal).unwrap();
        let expected_date_time = NaiveDateTime::from_str("2024-01-01T12:00:00").unwrap();

        assert_eq!(date_time, expected_date_time);
    }
}
