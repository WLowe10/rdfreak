use std::num::ParseIntError;

use oxrdf::Literal;

#[derive(Debug, thiserror::Error)]
pub enum RdfLiteralError {
    #[error("Expected a typed literal with datatype {expected}, but got {actual}")]
    InvalidDatatype { expected: String, actual: String },

    #[error("Failed to parse literal value: {0}")]
    Parse(String),
}

pub type FromRdfLiteralResult<T> = Result<T, RdfLiteralError>;

/// Represents a type that can be converted to and from an RDF literal.
pub trait FromRdfLiteral: Sized {
    /// Converts an RDF literal to the value type, if possible.
    fn from_literal(literal: &Literal) -> FromRdfLiteralResult<Self>;
}

impl FromRdfLiteral for i32 {
    fn from_literal(literal: &Literal) -> FromRdfLiteralResult<Self> {
        if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#integer" {
            return Err(RdfLiteralError::InvalidDatatype {
                expected: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
                actual: literal.datatype().as_str().to_string(),
            });
        }

        // why is the err param required to be explicitly typed here?
        let parsed_value: i32 = literal
            .value()
            .parse()
            .map_err(|err: ParseIntError| RdfLiteralError::Parse(err.to_string()))?;

        Ok(parsed_value)
    }
}

impl FromRdfLiteral for String {
    fn from_literal(literal: &Literal) -> FromRdfLiteralResult<Self> {
        if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#string" {
            return Err(RdfLiteralError::InvalidDatatype {
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
