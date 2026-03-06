use std::num::ParseIntError;

use oxrdf::{Literal, NamedNode};

#[derive(Debug, thiserror::Error)]
pub enum DeserializeLiteralError {
    #[error("Expected a typed literal with datatype {expected}, but got {actual}")]
    InvalidDatatype { expected: String, actual: String },

    #[error("Failed to parse literal value: {0}")]
    FailedToParse(String),
}

pub type DeserializeLiteralResult<T> = Result<T, DeserializeLiteralError>;

// we will be able to make a derive for RdfLiteral as long as the type implements Display + FromStr

/// Represents a type that can be converted to and from an RDF literal.
pub trait RdfLiteral: Sized {
    /// Converts the value to an RDF literal
    fn to_literal(&self) -> Literal;

    /// Converts an RDF literal to the value type, if possible.
    fn from_literal(literal: &Literal) -> DeserializeLiteralResult<Self>;
}

impl RdfLiteral for i32 {
    fn to_literal(&self) -> Literal {
        Literal::new_typed_literal(
            self.to_string(),
            NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
        )
    }

    fn from_literal(literal: &Literal) -> DeserializeLiteralResult<Self> {
        if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#integer" {
            return Err(DeserializeLiteralError::InvalidDatatype {
                expected: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
                actual: literal.datatype().as_str().to_string(),
            });
        }

        // why is the err param required to be explicitly typed here?
        let parsed_value: i32 = literal.value().parse().map_err(|err: ParseIntError| {
            DeserializeLiteralError::FailedToParse(err.to_string())
        })?;

        Ok(parsed_value)
    }
}

impl RdfLiteral for String {
    fn to_literal(&self) -> Literal {
        Literal::new_typed_literal(
            self,
            NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#string"),
        )
    }

    fn from_literal(literal: &Literal) -> DeserializeLiteralResult<Self> {
        if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#string" {
            return Err(DeserializeLiteralError::InvalidDatatype {
                expected: "http://www.w3.org/2001/XMLSchema#string".to_string(),
                actual: literal.datatype().as_str().to_string(),
            });
        }

        // let parsed_value: String = literal
        //     .value()
        //     .parse()
        //     .map_err(DeserializeLiteralError::FailedToParse)?;

        Ok(literal.value().to_owned())
    }
}
