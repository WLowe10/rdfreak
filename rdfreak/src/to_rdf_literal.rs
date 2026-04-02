use oxrdf::{Literal, NamedNode};

pub trait ToRdfLiteral {
    /// Converts the value to an RDF literal
    fn to_literal(&self) -> Literal;
}

impl ToRdfLiteral for i32 {
    fn to_literal(&self) -> Literal {
        Literal::new_typed_literal(
            self.to_string(),
            NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
        )
    }
}

impl ToRdfLiteral for String {
    fn to_literal(&self) -> Literal {
        Literal::new_typed_literal(
            self,
            NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#string"),
        )
    }
}
