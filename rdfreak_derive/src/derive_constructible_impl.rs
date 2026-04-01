use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::{
    parse_struct_field_rdf_attributes, parse_struct_rdf_attributes,
    validate_all_struct_field_rdf_attributes,
};

pub fn derive_constructible_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(struct_data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "Constructible can only be derived for structs",
        ));
    };

    let struct_identifier = &input.ident;

    parse_struct_rdf_attributes(&input)?;

    let field_attributes = struct_data
        .fields
        .iter()
        .map(parse_struct_field_rdf_attributes)
        .collect::<Result<Vec<_>, syn::Error>>()?;

    validate_all_struct_field_rdf_attributes(&input, struct_data, &field_attributes)?;

    // generate code for building construct patterns for each field

    let build_field_patterns_statements = struct_data
        .fields
        .iter()
        .zip(&field_attributes)
        .filter(|(_, attr)| !attr.is_subject)
        .map(|(field, attr)| {
    		let field_type = &field.ty;
            let predicate = attr.predicate.as_ref().unwrap();

            quote! {
                <#field_type as ::rdfreak::ConstructibleProperty>::insert_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked(#predicate));
            }
        })
        .collect::<Vec<_>>();

    let tokens = quote! {
        impl ::rdfreak::Constructible for #struct_identifier {
            fn insert_patterns(
                construct_query_patterns: &mut ::rdfreak::ConstructQueryPatterns,
                variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                subject_variable: &str,
            ) {
                let rdf_type_triple_pattern = ::rdfreak::TriplePattern::new(
                    ::rdfreak::TriplePatternNode::Variable(subject_variable.to_owned()),
                    ::rdfreak::TriplePatternNode::NamedNode(::oxrdf::NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")),
                    ::rdfreak::TriplePatternNode::NamedNode(<Self as ::rdfreak::Resource>::get_rdf_type()),
                );

                construct_query_patterns
                    .push_identical_triple_pattern(rdf_type_triple_pattern);

                #(#build_field_patterns_statements)*
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
                    ::rdfreak::TriplePatternNode::Variable(object_variable.clone()),
                );

                construct_query_patterns
                    .push_identical_triple_pattern(triple_pattern);

                <Self as ::rdfreak::Constructible>::insert_patterns(construct_query_patterns, variable_generator, &object_variable);
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
            #[rdf(type = "http://example.org/Person")]
            struct Person {
                #[rdf(subject)]
                subject: oxrdf::NamedOrBlankNode,

                #[rdf(predicate = "http://example.org/name")]
                name: String,

                #[rdf(predicate = "http://example.org/age")]
                age: u32,
            }
        };

        let expected = quote! {
            impl ::rdfreak::Constructible for Person {
                fn insert_patterns(
                    construct_query_patterns: &mut ::rdfreak::ConstructQueryPatterns,
                    variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                    subject_variable: &str,
                ) {
                    let rdf_type_triple_pattern = ::rdfreak::TriplePattern::new(
                        ::rdfreak::TriplePatternNode::Variable(subject_variable.to_owned()),
                        ::rdfreak::TriplePatternNode::NamedNode(::oxrdf::NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")),
                        ::rdfreak::TriplePatternNode::NamedNode(<Self as ::rdfreak::Resource>::get_rdf_type()),
                    );

                    construct_query_patterns
                        .push_identical_triple_pattern(rdf_type_triple_pattern);

                    <String as ::rdfreak::ConstructibleProperty>::insert_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked("http://example.org/name"));
                    <u32 as ::rdfreak::ConstructibleProperty>::insert_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked("http://example.org/age"));
                }
            }

            impl ::rdfreak::ConstructibleProperty for Person {
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
                        ::rdfreak::TriplePatternNode::Variable(object_variable.clone()),
                    );

                    construct_query_patterns
                        .push_identical_triple_pattern(triple_pattern);

                    <Self as ::rdfreak::Constructible>::insert_patterns(construct_query_patterns, variable_generator, &object_variable);
                }
            }
        };

        let generated = derive_constructible_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
