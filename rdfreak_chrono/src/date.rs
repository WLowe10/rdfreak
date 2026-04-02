use std::{fmt::Display, ops::Deref, str::FromStr};

use rdfreak_derive::RdfLiteral;

/// A wrapper around `chrono::NaiveDate` for representing xsd:date literals in RDF.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, RdfLiteral)]
#[rdf(datatype = "http://www.w3.org/2001/XMLSchema#date")]
pub struct Date(chrono::NaiveDate);

impl Date {
    pub fn new(date: chrono::NaiveDate) -> Self {
        Self(date)
    }

    pub fn inner(&self) -> chrono::NaiveDate {
        self.0
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl FromStr for Date {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")?;

        Ok(Self::new(date))
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

#[cfg(test)]
mod tests {
    use super::*;

    use oxrdf::{Literal, NamedNode};
    use rdfreak::{FromRdfLiteral, ToRdfLiteral};

    #[test]
    fn test_to_literal() {
        let date = Date::from_str("2024-01-01").unwrap();
        let date_literal = date.to_literal();

        let expected_literal = Literal::new_typed_literal(
            "2024-01-01".to_owned(),
            NamedNode::new("http://www.w3.org/2001/XMLSchema#date").unwrap(),
        );

        assert_eq!(date_literal, expected_literal);
    }

    #[test]
    fn test_from_literal() {
        let date_literal = Literal::new_typed_literal(
            "2024-01-01".to_owned(),
            NamedNode::new("http://www.w3.org/2001/XMLSchema#date").unwrap(),
        );

        let date = Date::from_literal(&date_literal).unwrap();
        let expected_date = Date::from_str("2024-01-01").unwrap();

        assert_eq!(date, expected_date);
    }
}
