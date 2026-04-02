/// Implements all of the traits a type should implement to be used as a literal in rdfreak.
///
/// The type must implement `ToString` and `FromStr` for this to work, and the literal will be represented as a typed literal with the provided datatype.
///
/// # Example
/// ```
/// use rdfreak::impl_traits_for_literal;
///
/// impl_traits_for_literal!(i32, "http://www.w3.org/2001/XMLSchema#integer");
/// ```
#[macro_export]
macro_rules! impl_traits_for_literal {
    ($t:ty, $datatype:expr) => {
        impl $crate::ToRdfLiteral for $t {
            fn to_literal(&self) -> ::oxrdf::Literal {
                ::oxrdf::Literal::new_typed_literal(
                    <Self as ::std::string::ToString>::to_string(self),
                    ::oxrdf::NamedNode::new_unchecked($datatype),
                )
            }
        }

        impl $crate::FromRdfLiteral for $t {
            fn from_literal(literal: &::oxrdf::Literal) -> $crate::FromRdfLiteralResult<Self> {
                if literal.datatype().as_str() != $datatype {
                    return Err($crate::RdfLiteralError::InvalidDatatype {
                        expected: $datatype.to_string(),
                        actual: literal.datatype().as_str().to_string(),
                    });
                }

                let parsed_value = <$t as ::std::str::FromStr>::from_str(literal.value())
                    .map_err(|err| $crate::RdfLiteralError::Parse(err.to_string()))?;

                Ok(parsed_value)
            }
        }

        impl $crate::ToRdfObject for $t {
            fn to_term(&self, _graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                ::oxrdf::Term::Literal(<Self as $crate::ToRdfLiteral>::to_literal(self))
            }
        }

        impl $crate::FromRdfObject for $t {
            fn from_term(
                _graph: &::oxrdf::Graph,
                term: &::oxrdf::Term,
            ) -> $crate::FromRdfObjectResult<Self> {
                let ::oxrdf::Term::Literal(literal) = term else {
                    return Err($crate::RdfObjectError::UnexpectedTermType(term.clone()));
                };

                let value = <Self as $crate::FromRdfLiteral>::from_literal(literal)?;

                Ok(value)
            }
        }

        impl $crate::ToRdfProperty for $t {
            fn to_property(
                &self,
                graph: &mut ::oxrdf::Graph,
                subject: &::oxrdf::NamedOrBlankNode,
                predicate: &::oxrdf::NamedNode,
            ) {
                let object_term = <Self as $crate::ToRdfObject>::to_term(self, graph);

                graph.insert(&::oxrdf::Triple::new(
                    subject.as_ref(),
                    predicate.as_ref(),
                    object_term,
                ));
            }
        }

        impl $crate::FromRdfProperty for $t {
            fn from_property(
                graph: &::oxrdf::Graph,
                subject: &::oxrdf::NamedOrBlankNode,
                predicate: &::oxrdf::NamedNode,
            ) -> $crate::DeserializeRdfPropertyResult<Self> {
                let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

                let Some(object_term) = maybe_object_term else {
                    return Err($crate::RdfPropertyError::MissingObjectValue(
                        predicate.clone(),
                    ));
                };

                let object_value =
                    <Self as $crate::FromRdfObject>::from_term(graph, &object_term.into())?;

                Ok(object_value)
            }
        }
    };
}
