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

macro_rules! impl_from_rdf_literal_for {
    ($t:ty, $datatype:expr) => {
        impl FromRdfLiteral for $t {
            fn from_literal(literal: &Literal) -> FromRdfLiteralResult<Self> {
                if literal.datatype().as_str() != $datatype {
                    return Err(RdfLiteralError::InvalidDatatype {
                        expected: $datatype.to_string(),
                        actual: literal.datatype().as_str().to_string(),
                    });
                }

                let parsed_value: $t = literal
                    .value()
                    .parse::<$t>()
                    .map_err(|err| RdfLiteralError::Parse(err.to_string()))?;

                Ok(parsed_value)
            }
        }
    };
}

impl_from_rdf_literal_for!(bool, "http://www.w3.org/2001/XMLSchema#boolean");

impl_from_rdf_literal_for!(i8, "http://www.w3.org/2001/XMLSchema#byte");
impl_from_rdf_literal_for!(i32, "http://www.w3.org/2001/XMLSchema#integer");
impl_from_rdf_literal_for!(i64, "http://www.w3.org/2001/XMLSchema#long");

impl_from_rdf_literal_for!(u8, "http://www.w3.org/2001/XMLSchema#unsignedByte");
impl_from_rdf_literal_for!(u32, "http://www.w3.org/2001/XMLSchema#unsignedInt");
impl_from_rdf_literal_for!(u64, "http://www.w3.org/2001/XMLSchema#unsignedLong");

// or, should we just use decimal for both of them?
impl_from_rdf_literal_for!(f32, "http://www.w3.org/2001/XMLSchema#float");
impl_from_rdf_literal_for!(f64, "http://www.w3.org/2001/XMLSchema#decimal");

impl_from_rdf_literal_for!(String, "http://www.w3.org/2001/XMLSchema#string");
