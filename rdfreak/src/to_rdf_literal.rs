use oxrdf::{Literal, NamedNode};

pub trait ToRdfLiteral {
    /// Converts the value to an RDF literal
    fn to_literal(&self) -> Literal;
}

macro_rules! impl_to_rdf_literal_for {
    ($t:ty, $datatype:expr) => {
        impl ToRdfLiteral for $t {
            fn to_literal(&self) -> Literal {
                Literal::new_typed_literal(self.to_string(), NamedNode::new_unchecked($datatype))
            }
        }
    };
}

impl_to_rdf_literal_for!(bool, "http://www.w3.org/2001/XMLSchema#boolean");

impl_to_rdf_literal_for!(i8, "http://www.w3.org/2001/XMLSchema#byte");
impl_to_rdf_literal_for!(i32, "http://www.w3.org/2001/XMLSchema#integer");
impl_to_rdf_literal_for!(i64, "http://www.w3.org/2001/XMLSchema#long");

impl_to_rdf_literal_for!(u8, "http://www.w3.org/2001/XMLSchema#unsignedByte");
impl_to_rdf_literal_for!(u32, "http://www.w3.org/2001/XMLSchema#unsignedInt");
impl_to_rdf_literal_for!(u64, "http://www.w3.org/2001/XMLSchema#unsignedLong");

// or, should we just use decimal for both of them?
impl_to_rdf_literal_for!(f32, "http://www.w3.org/2001/XMLSchema#float");
impl_to_rdf_literal_for!(f64, "http://www.w3.org/2001/XMLSchema#decimal");

impl_to_rdf_literal_for!(String, "http://www.w3.org/2001/XMLSchema#string");
