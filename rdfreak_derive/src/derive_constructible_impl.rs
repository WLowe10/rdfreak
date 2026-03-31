use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::parse_struct_field_rdf_attributes;

pub fn derive_constructible_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(struct_data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "Constructible can only be derived for structs",
        ));
    };

    let struct_identifier = &input.ident;

    let property_attributes = struct_data
        .fields
        .iter()
        .map(parse_struct_field_rdf_attributes)
        .collect::<Result<Vec<_>, syn::Error>>()?;

    // generate code for building construct patterns for each property

    let build_property_patterns_statements = struct_data
        .fields
        .iter()
        .zip(&property_attributes)
        .filter(|(_, attr)| !attr.is_subject)
        .map(|(field, attr)| {
    		let field_type = &field.ty;
            let predicate = attr.predicate.as_ref().unwrap();

            let build_property_pattern_statement = quote! {
                <#field_type as ::rdfreak::ConstructibleRdfProperty>::build_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked(#predicate));
            };

            Ok(build_property_pattern_statement)
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let tokens = quote! {
        impl ::rdfreak::ConstructibleEntity for #struct_identifier {
            fn build_property_patterns(
                construct_query_patterns: &mut ::rdfreak::SparqlConstructQueryPatterns,
                variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                subject_variable: &str,
            ) {
                #(#build_property_patterns_statements)*
            }
        }

        impl ::rdfreak::ConstructibleRdfProperty for #struct_identifier {
            fn build_patterns(
                construct_query_patterns: &mut ::rdfreak::SparqlConstructQueryPatterns,
                variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                subject_variable: &str,
                predicate: &::oxrdf::NamedNode,
            ) {
                let object_variable = variable_generator.next().unwrap();

                let triple_pattern = format!(
                    "\t{} {} {} .\n",
                    subject_variable, predicate, object_variable
                );

                construct_query_patterns.patterns.push_str(&triple_pattern);

                construct_query_patterns
                    .where_patterns
                    .push_str(&triple_pattern);

                <Self as ::rdfreak::ConstructibleEntity>::build_patterns(construct_query_patterns, variable_generator, &object_variable);
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
        let input_tokens: syn::DeriveInput = syn::parse2(quote! {
            struct Person {
                #[rdf(subject)]
                iri: oxrdf::NamedNode,

                #[rdf(predicate = "http://example.org/name")]
                name: String,

                #[rdf(predicate = "http://example.org/age")]
                age: u32,
            }
        })
        .unwrap();

        let expected = quote! {
            impl ::rdfreak::ConstructibleEntity for Person {
                fn build_property_patterns(
                    construct_query_patterns: &mut ::rdfreak::SparqlConstructQueryPatterns,
                    variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                    subject_variable: &str,
                ) {
                    <String as ::rdfreak::ConstructibleRdfProperty>::build_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked("http://example.org/name"));
                    <u32 as ::rdfreak::ConstructibleRdfProperty>::build_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked("http://example.org/age"));
                }
            }

            impl ::rdfreak::ConstructibleRdfProperty for Person {
                fn build_patterns(
                    construct_query_patterns: &mut ::rdfreak::SparqlConstructQueryPatterns,
                    variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                    subject_variable: &str,
                    predicate: &::oxrdf::NamedNode,
                ) {
                    let object_variable = variable_generator.next().unwrap();

                    let triple_pattern = format!(
                        "\t{} {} {} .\n",
                        subject_variable, predicate, object_variable
                    );

                    construct_query_patterns.patterns.push_str(&triple_pattern);

                    construct_query_patterns
                        .where_patterns
                        .push_str(&triple_pattern);

                    <Self as ::rdfreak::ConstructibleEntity>::build_patterns(construct_query_patterns, variable_generator, &object_variable);
                }
            }
        };

        let generated = derive_constructible_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
