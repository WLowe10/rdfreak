use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::{
    parse_struct_field_rdf_attributes, parse_struct_rdf_attributes,
    validate_all_struct_field_rdf_attributes,
};

pub fn derive_from_rdf_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(struct_data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "FromRdf can only be derived for structs",
        ));
    };

    let struct_identifier = &input.ident;

    parse_struct_rdf_attributes(&input)?;

    let field_rdf_attributes = struct_data
        .fields
        .iter()
        .map(parse_struct_field_rdf_attributes)
        .collect::<Result<Vec<_>, syn::Error>>()?;

    validate_all_struct_field_rdf_attributes(&input, struct_data, &field_rdf_attributes)?;

    let subject_field = struct_data
        .fields
        .iter()
        .zip(&field_rdf_attributes)
        .find(|(_, attr)| attr.is_subject)
        .unwrap()
        .0;

    let subject_identifier = subject_field.ident.as_ref().unwrap();

    // generate code for deserializing each property

    let deserialize_struct_field_inits = struct_data
        .fields
        .iter()
        .zip(&field_rdf_attributes)
        .filter(|(_, attr)| !attr.is_subject)
        .map(|(field, attr)| {
            let field_ident = field.ident.as_ref().unwrap();
            let predicate = attr.predicate.as_ref().unwrap();
            let field_name_str = syn::LitStr::new(&field_ident.to_string(), field_ident.span());

            quote! {
                #field_ident: ::rdfreak::FromRdfProperty::from_property(
                    graph,
                    subject,
                    &::oxrdf::NamedNode::new_unchecked(#predicate),
                ).map_err(|err| ::rdfreak::DeserializeResourceError::FailedToDeserializeProperty {
                    property: #field_name_str.to_owned(),
                    subject: subject.clone(),
                    source: Box::new(err),
                })?,
            }
        })
        .collect::<Vec<_>>();

    let tokens = quote! {
        impl ::rdfreak::FromRdf for #struct_identifier {
            fn from_rdf(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode) -> ::rdfreak::DeserializeResourceResult<Self> {
                use ::rdfreak::FromRdfProperty;

                let expected_rdf_type = <Self as ::rdfreak::Resource>::get_rdf_type();

                let rdf_types = Vec::<::rdfreak::RdfType>::from_property(
                    graph,
                    subject,
                    &NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                )
                .map_err(|err| ::rdfreak::DeserializeResourceError::FailedToDeserializeProperty {
                    subject: subject.clone(),
                    property: "rdf:type".to_string(),
                    source: Box::new(err),
                })?;

                let has_expected_rdf_type = rdf_types
                    .iter()
                    .any(|rdf_type| rdf_type.get_named_node() == &expected_rdf_type);

                if !has_expected_rdf_type {
                    return Err(::rdfreak::DeserializeResourceError::InvalidRdfType {
                        expected: expected_rdf_type,
                        found: rdf_types
                            .into_iter()
                            .map(|rdf_type| rdf_type.get_named_node().clone())
                            .collect(),
                    });
                }

                Ok(Self {
                    #subject_identifier: subject.clone(),
                    #(#deserialize_struct_field_inits)*
                })
            }
        }

        impl ::rdfreak::FromRdfObject for #struct_identifier {
            fn from_term(graph: &::oxrdf::Graph, term: &::oxrdf::Term) -> ::rdfreak::DeserializeRdfObjectResult<Self> {
                let ::oxrdf::Term::NamedNode(named_node) = term else {
                    return Err(::rdfreak::DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
                };

                let value = <Self as ::rdfreak::FromRdf>::from_rdf(graph, &::oxrdf::NamedOrBlankNode::NamedNode(named_node.clone()))?;

                Ok(value)
            }
        }

        impl ::rdfreak::FromRdfProperty for #struct_identifier {
            fn from_property(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) -> ::rdfreak::DeserializeRdfPropertyResult<Self> {
                let object_term = graph
                    .object_for_subject_predicate(subject, predicate)
                    .ok_or_else(|| ::rdfreak::DeserializeRdfPropertyError::MissingObjectValue(predicate.clone()))?;

                let value = <Self as ::rdfreak::FromRdfObject>::from_term(graph, &object_term.into_owned())?;

                Ok(value)
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

                #[rdf(predicate = "http://example.org/occupation")]
                occupation: Option<String>,
            }
        };

        let expected = quote! {
            impl ::rdfreak::FromRdf for Person {
                fn from_rdf(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode) -> ::rdfreak::DeserializeResourceResult<Self> {
                    use ::rdfreak::FromRdfProperty;

                    let expected_rdf_type = <Self as ::rdfreak::Resource>::get_rdf_type();

                    let rdf_types = Vec::<::rdfreak::RdfType>::from_property(
                        graph,
                        subject,
                        &NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    )
                    .map_err(|err| ::rdfreak::DeserializeResourceError::FailedToDeserializeProperty {
                        subject: subject.clone(),
                        property: "rdf:type".to_string(),
                        source: Box::new(err),
                    })?;

                    let has_expected_rdf_type = rdf_types
                        .iter()
                        .any(|rdf_type| rdf_type.get_named_node() == &expected_rdf_type);

                    if !has_expected_rdf_type {
                        return Err(::rdfreak::DeserializeResourceError::InvalidRdfType {
                            expected: expected_rdf_type,
                            found: rdf_types
                                .into_iter()
                                .map(|rdf_type| rdf_type.get_named_node().clone())
                                .collect(),
                        });
                    }

                    Ok(Self {
                        subject: subject.clone(),
                        name: ::rdfreak::FromRdfProperty::from_property(
                            graph,
                            subject,
                            &::oxrdf::NamedNode::new_unchecked("http://example.org/name"),
                        )
                        .map_err(
                            |err| ::rdfreak::DeserializeResourceError::FailedToDeserializeProperty {
                                property: "name".to_owned(),
                                subject: subject.clone(),
                                source: Box::new(err),
                            }
                        )?,
                        age: ::rdfreak::FromRdfProperty::from_property(
                            graph,
                            subject,
                            &::oxrdf::NamedNode::new_unchecked("http://example.org/age"),
                        )
                        .map_err(
                            |err| ::rdfreak::DeserializeResourceError::FailedToDeserializeProperty {
                                property: "age".to_owned(),
                                subject: subject.clone(),
                                source: Box::new(err),
                            }
                        )?,
                        occupation: ::rdfreak::FromRdfProperty::from_property(
                            graph,
                            subject,
                            &::oxrdf::NamedNode::new_unchecked("http://example.org/occupation"),
                        )
                        .map_err(
                            |err| ::rdfreak::DeserializeResourceError::FailedToDeserializeProperty {
                                property: "occupation".to_owned(),
                                subject: subject.clone(),
                                source: Box::new(err),
                            }
                        )?,
                    })
                }
            }

            impl ::rdfreak::FromRdfObject for Person {
                fn from_term(graph: &::oxrdf::Graph, term: &::oxrdf::Term) -> ::rdfreak::DeserializeRdfObjectResult<Self> {
                    let ::oxrdf::Term::NamedNode(named_node) = term else {
                        return Err(::rdfreak::DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
                    };

                    let value = <Self as ::rdfreak::FromRdf>::from_rdf(graph, &::oxrdf::NamedOrBlankNode::NamedNode(named_node.clone()))?;

                    Ok(value)
                }
            }

            impl ::rdfreak::FromRdfProperty for Person {
                fn from_property(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) -> ::rdfreak::DeserializeRdfPropertyResult<Self> {
                    let object_term = graph
                        .object_for_subject_predicate(subject, predicate)
                        .ok_or_else(|| ::rdfreak::DeserializeRdfPropertyError::MissingObjectValue(predicate.clone()))?;

                    let value = <Self as ::rdfreak::FromRdfObject>::from_term(graph, &object_term.into_owned())?;

                    Ok(value)
                }
            }
        };

        let generated = derive_from_rdf_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
