use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::parse_literal_struct_rdf_attributes;

pub fn derive_rdf_literal_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(_) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "RdfLiteral can only be derived for structs",
        ));
    };

    let struct_identifier = &input.ident;

    let literal_attributes = parse_literal_struct_rdf_attributes(&input)?;
    let datatype = literal_attributes.datatype;

    let tokens = quote! {
        impl ::rdfreak::ToRdfLiteral for #struct_identifier {
            fn to_literal(&self) -> ::oxrdf::Literal {
                ::oxrdf::Literal::new_typed_literal(
                    <Self as ::std::string::ToString>::to_string(self),
                    ::oxrdf::NamedNode::new_unchecked(#datatype),
                )
            }
        }

        impl ::rdfreak::FromRdfLiteral for #struct_identifier {
            fn from_literal(literal: &::oxrdf::Literal) -> ::rdfreak::FromRdfLiteralResult<Self> {
                if literal.datatype().as_str() != #datatype {
                    return Err(::rdfreak::RdfLiteralError::InvalidDatatype {
                        expected: #datatype.to_owned(),
                        actual: literal.datatype().as_str().to_owned(),
                    });
                }

                let parsed_value = <Self as ::std::str::FromStr>::from_str(literal.value())
                    .map_err(|err| ::rdfreak::RdfLiteralError::Parse(err.to_string()))?;

                Ok(parsed_value)
            }
        }

        impl ::rdfreak::ToRdfObject for #struct_identifier {
            fn to_term(&self, _graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                ::oxrdf::Term::Literal(<Self as ::rdfreak::ToRdfLiteral>::to_literal(self))
            }
        }

        impl ::rdfreak::FromRdfObject for #struct_identifier {
            fn from_term(
                _graph: &::oxrdf::Graph,
                term: &::oxrdf::Term,
            ) -> ::rdfreak::FromRdfObjectResult<Self> {
                let ::oxrdf::Term::Literal(literal) = term else {
                    return Err(::rdfreak::RdfObjectError::UnexpectedTermType(term.clone()));
                };

                let value = <Self as ::rdfreak::FromRdfLiteral>::from_literal(literal)?;

                Ok(value)
            }
        }

        impl ::rdfreak::ToRdfProperty for #struct_identifier {
            fn to_property(
                &self,
                graph: &mut ::oxrdf::Graph,
                subject: &::oxrdf::NamedOrBlankNode,
                predicate: &::oxrdf::NamedNode,
            ) {
                let object_term = <Self as ::rdfreak::ToRdfObject>::to_term(self, graph);

                graph.insert(&::oxrdf::Triple::new(
                    subject.as_ref(),
                    predicate.as_ref(),
                    object_term,
                ));
            }
        }

        impl ::rdfreak::FromRdfProperty for #struct_identifier {
            fn from_property(
                graph: &::oxrdf::Graph,
                subject: &::oxrdf::NamedOrBlankNode,
                predicate: &::oxrdf::NamedNode,
            ) -> ::rdfreak::FromRdfPropertyResult<Self> {
                let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

                let Some(object_term) = maybe_object_term else {
                    return Err(::rdfreak::RdfPropertyError::MissingObjectValue(
                        predicate.clone(),
                    ));
                };

                let object_value =
                    <Self as ::rdfreak::FromRdfObject>::from_term(graph, &object_term.into())?;

                Ok(object_value)
            }
        }

        impl ::rdfreak::ConstructibleProperty for #struct_identifier {
            fn insert_patterns(
                construct_query_patterns: &mut ::rdfreak::ConstructQueryPatterns,
                variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                subject_variable: &str,
                predicate: &::oxrdf::NamedNode,
            ) {
                let object_variable = variable_generator.next().unwrap();

                let triple_pattern = ::rdfreak::TriplePattern::new(
                    ::rdfreak::TriplePatternNode::Variable(subject_variable.to_owned()),
                    ::rdfreak::TriplePatternNode::NamedNode(predicate.clone()),
                    ::rdfreak::TriplePatternNode::Variable(object_variable),
                );

                construct_query_patterns.push_identical_triple_pattern(triple_pattern);
            }
        }
    };

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        let input_tokens: syn::DeriveInput = syn::parse_quote! {
            #[rdf(datatype = "http://www.w3.org/2001/XMLSchema#date")]
            struct Date(chrono::NaiveDate);
        };

        let expected = quote! {
            impl ::rdfreak::ToRdfLiteral for Date {
                fn to_literal(&self) -> ::oxrdf::Literal {
                    ::oxrdf::Literal::new_typed_literal(
                        <Self as ::std::string::ToString>::to_string(self),
                        ::oxrdf::NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#date"),
                    )
                }
            }

            impl ::rdfreak::FromRdfLiteral for Date {
                fn from_literal(literal: &::oxrdf::Literal) -> ::rdfreak::FromRdfLiteralResult<Self> {
                    if literal.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#date" {
                        return Err(::rdfreak::RdfLiteralError::InvalidDatatype {
                            expected: "http://www.w3.org/2001/XMLSchema#date".to_owned(),
                            actual: literal.datatype().as_str().to_owned(),
                        });
                    }

                    let parsed_value = <Self as ::std::str::FromStr>::from_str(literal.value())
                        .map_err(|err| ::rdfreak::RdfLiteralError::Parse(err.to_string()))?;

                    Ok(parsed_value)
                }
            }

            impl ::rdfreak::ToRdfObject for Date {
                fn to_term(&self, _graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                    ::oxrdf::Term::Literal(<Self as ::rdfreak::ToRdfLiteral>::to_literal(self))
                }
            }

            impl ::rdfreak::FromRdfObject for Date {
                fn from_term(
                    _graph: &::oxrdf::Graph,
                    term: &::oxrdf::Term,
                ) -> ::rdfreak::FromRdfObjectResult<Self> {
                    let ::oxrdf::Term::Literal(literal) = term else {
                        return Err(::rdfreak::RdfObjectError::UnexpectedTermType(term.clone()));
                    };

                    let value = <Self as ::rdfreak::FromRdfLiteral>::from_literal(literal)?;

                    Ok(value)
                }
            }

            impl ::rdfreak::ToRdfProperty for Date {
                fn to_property(
                    &self,
                    graph: &mut ::oxrdf::Graph,
                    subject: &::oxrdf::NamedOrBlankNode,
                    predicate: &::oxrdf::NamedNode,
                ) {
                    let object_term = <Self as ::rdfreak::ToRdfObject>::to_term(self, graph);

                    graph.insert(&::oxrdf::Triple::new(
                        subject.as_ref(),
                        predicate.as_ref(),
                        object_term,
                    ));
                }
            }

            impl ::rdfreak::FromRdfProperty for Date {
                fn from_property(
                    graph: &::oxrdf::Graph,
                    subject: &::oxrdf::NamedOrBlankNode,
                    predicate: &::oxrdf::NamedNode,
                ) -> ::rdfreak::FromRdfPropertyResult<Self> {
                    let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

                    let Some(object_term) = maybe_object_term else {
                        return Err(::rdfreak::RdfPropertyError::MissingObjectValue(
                            predicate.clone(),
                        ));
                    };

                    let object_value =
                        <Self as ::rdfreak::FromRdfObject>::from_term(graph, &object_term.into())?;

                    Ok(object_value)
                }
            }

            impl ::rdfreak::ConstructibleProperty for Date {
                fn insert_patterns(
                    construct_query_patterns: &mut ::rdfreak::ConstructQueryPatterns,
                    variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                    subject_variable: &str,
                    predicate: &::oxrdf::NamedNode,
                ) {
                    let object_variable = variable_generator.next().unwrap();

                    let triple_pattern = ::rdfreak::TriplePattern::new(
                        ::rdfreak::TriplePatternNode::Variable(subject_variable.to_owned()),
                        ::rdfreak::TriplePatternNode::NamedNode(predicate.clone()),
                        ::rdfreak::TriplePatternNode::Variable(object_variable),
                    );

                    construct_query_patterns.push_identical_triple_pattern(triple_pattern);
                }
            }
        };

        let generated = derive_rdf_literal_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
