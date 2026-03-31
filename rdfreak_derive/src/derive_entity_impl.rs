use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::get_rdf_attribute;

#[derive(Debug, FromMeta)]
struct EntityStructRdfAttributes {
    #[darling(rename = "type")]
    rdf_type: String,
}

#[derive(Debug, FromMeta)]
struct StructFieldRdfAttributes {
    #[darling(default, rename = "subject")]
    is_subject: bool,

    predicate: Option<String>,
}

/// parses the expected RDF attributes from an entity struct-level attribute
fn parse_struct_rdf_attributes(input: &syn::DeriveInput) -> syn::Result<EntityStructRdfAttributes> {
    let attr = get_rdf_attribute(&input.attrs).ok_or_else(|| {
        syn::Error::new_spanned(input, "Missing required attribute: #[rdf(type = \"...\")]")
    })?;

    EntityStructRdfAttributes::from_meta(&attr.meta)
        .map_err(|err| syn::Error::new_spanned(attr, err))
}

/// parses the expected RDF attributes from a struct field-level attribute
fn parse_struct_field_rdf_attributes(field: &syn::Field) -> syn::Result<StructFieldRdfAttributes> {
    let attr = get_rdf_attribute(&field.attrs).ok_or_else(|| {
        syn::Error::new_spanned(
            field,
            "Missing required attribute: #[rdf(predicate = \"...\")]",
        )
    })?;

    StructFieldRdfAttributes::from_meta(&attr.meta)
        .map_err(|err| syn::Error::new_spanned(attr, err))
}

pub fn derive_entity_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(struct_data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "Entity can only be derived for structs",
        ));
    };

    let struct_identifier = &input.ident;
    let struct_rdf_attributes = parse_struct_rdf_attributes(&input)?;
    let struct_rdf_type = &struct_rdf_attributes.rdf_type;

    let property_attributes = struct_data
        .fields
        .iter()
        .map(parse_struct_field_rdf_attributes)
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let property_attributes_with_subject_count = property_attributes
        .iter()
        .filter(|attr| attr.is_subject)
        .count();

    // validate that at most one struct field has the #[rdf(subject)] attribute
    if property_attributes_with_subject_count > 1 {
        return Err(syn::Error::new_spanned(
            input,
            "Only one struct field can have the #[rdf(subject)] attribute",
        ));
    }

    let maybe_iri_field_idx = property_attributes.iter().position(|attr| attr.is_subject);

    let Some(iri_field_idx) = maybe_iri_field_idx else {
        return Err(syn::Error::new_spanned(
            input,
            "Missing required attribute: #[rdf(subject)] on one of the struct fields",
        ));
    };

    let subject_field = &struct_data.fields.iter().nth(iri_field_idx).unwrap();
    let subject_identifier = subject_field.ident.as_ref().unwrap();

    // validate that no struct field has both #[rdf(subject)] and #[rdf(predicate = "…")] attributes
    for property_attr in &property_attributes {
        if property_attr.is_subject && property_attr.predicate.is_some() {
            return Err(syn::Error::new_spanned(
                input,
                "A struct field cannot have both #[rdf(subject)] and #[rdf(predicate = \"...\")] attributes",
            ));
        }
    }

    // generate code for serializing each property

    let serialize_property_statements = struct_data
        .fields
        .iter()
        .zip(&property_attributes)
        .filter(|(_, attr)| !attr.is_subject)
        .map(|(field, attr)| {
            let field_ident = field.ident.as_ref().unwrap();
            let predicate = attr.predicate.as_ref().unwrap();

            let serialize_field_statement = quote! {
                ::rdfreak::RdfProperty::serialize_property(&self.#field_ident, graph, subject, &::oxrdf::NamedNode::new_unchecked(#predicate));
            };

            Ok(serialize_field_statement)
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    // generate code for deserializing each property

    let deserialize_struct_field_inits = struct_data
        .fields
        .iter()
        .zip(&property_attributes)
        .filter(|(_, attr)| !attr.is_subject)
        .map(|(field, attr)| {
            let field_ident = field.ident.as_ref().unwrap();
            let predicate = attr.predicate.as_ref().unwrap();
            let field_name_str = syn::LitStr::new(&field_ident.to_string(), field_ident.span());

            let deserialize_field = quote! {
                #field_ident: ::rdfreak::RdfProperty::deserialize_property(
                    graph,
                    subject,
                    &::oxrdf::NamedNode::new_unchecked(#predicate),
                ).map_err(|err| ::rdfreak::DeserializeEntityError::FailedToDeserializeProperty {
                    property: #field_name_str.to_owned(),
                    subject: subject.clone(),
                    source: Box::new(err),
                })?,
            };

            Ok(deserialize_field)
        })
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
                <#field_type as ::rdfreak::ConstructableRdfProperty>::build_patterns(construct_query_patterns, variable_generator, subject_variable, &::oxrdf::NamedNode::new_unchecked(#predicate));
            };

            Ok(build_property_pattern_statement)
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let tokens = quote! {
        impl ::rdfreak::Entity for #struct_identifier {
            fn get_rdf_type() -> ::oxrdf::NamedNode {
                ::oxrdf::NamedNode::new_unchecked(#struct_rdf_type)
            }

            fn get_subject(&self) -> &::oxrdf::NamedOrBlankNode {
                &self.#subject_identifier
            }

            fn serialize_properties(&self, graph: &mut ::oxrdf::Graph) {
                let subject = ::rdfreak::Entity::get_subject(self);

                #(#serialize_property_statements)*
            }

            fn deserialize_properties(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode) -> ::rdfreak::DeserializeEntityResult<Self> {
                Ok(Self {
                    #subject_identifier: subject.clone(),
                    #(#deserialize_struct_field_inits)*
                })
            }
        }

        impl ::rdfreak::RdfObject for #struct_identifier {
            fn to_term(&self, graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                ::rdfreak::Entity::serialize(self, graph);

                let subject = ::rdfreak::Entity::get_subject(self);

                match subject {
                    NamedOrBlankNode::NamedNode(named_node) => Term::NamedNode(named_node.clone()),
                    NamedOrBlankNode::BlankNode(blank_node) => Term::BlankNode(blank_node.clone()),
                }
            }

            fn from_term(graph: &::oxrdf::Graph, term: &::oxrdf::Term) -> ::rdfreak::DeserializeRdfObjectResult<Self> {
                let ::oxrdf::Term::NamedNode(named_node) = term else {
                    return Err(::rdfreak::DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
                };

                let value = <Self as ::rdfreak::Entity>::deserialize(graph, &::oxrdf::NamedOrBlankNode::NamedNode(named_node.clone()))?;

                Ok(value)
            }
        }

        impl ::rdfreak::RdfProperty for #struct_identifier {
            fn serialize_property(&self, graph: &mut ::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) {
                let object_term = ::rdfreak::RdfObject::to_term(self, graph);

                graph.insert(&::oxrdf::Triple::new(
                    subject.as_ref(),
                    predicate.as_ref(),
                    object_term,
                ));
            }

            fn deserialize_property(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) -> ::rdfreak::DeserializeRdfPropertyResult<Self> {
                let object_term = graph
                    .object_for_subject_predicate(subject, predicate)
                    .ok_or_else(|| ::rdfreak::DeserializeRdfPropertyError::MissingObjectValue(predicate.clone()))?;

                let value = <Self as ::rdfreak::RdfObject>::from_term(graph, &object_term.into_owned())?;

                Ok(value)
            }
        }

        impl ::rdfreak::ConstructableEntity for #struct_identifier {
            fn build_property_patterns(
                construct_query_patterns: &mut ::rdfreak::SparqlConstructQueryPatterns,
                variable_generator: &mut ::rdfreak::SparqlVariableGenerator,
                subject_variable: &str,
            ) {
                #(#build_property_patterns_statements)*
            }
        }

        impl ::rdfreak::ConstructableRdfProperty for #struct_identifier {
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

                <Self as ::rdfreak::ConstructableEntity>::build_patterns(construct_query_patterns, variable_generator, &object_variable);
            }
        }
    };

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fails_on_non_struct() {
        let input_tokens: syn::DeriveInput = syn::parse2(quote! {
            enum StopLight {
                Green,
                Yellow,
                Red,
            }

        })
        .unwrap();

        let derive_error = derive_entity_impl(input_tokens).err().unwrap();

        assert!(
            derive_error
                .to_string()
                .contains("Entity can only be derived for structs")
        );
    }

    #[test]
    fn test_fails_missing_rdf_attributes() {
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

        let derive_error = derive_entity_impl(input_tokens).err().unwrap();

        assert!(
            derive_error
                .to_string()
                .contains("Missing required attribute: #[rdf(type = \"...\")]")
        );
    }

    // #[test]
    // fn test_fails_invalid_rdf_attributes() {
    // }

    // #[test]
    // fn test_fails_missing_field_rdf_attributes() {
    // }

    // need test for missing #[rdf(subject)] attribute on struct fields

    // need test for more than one #[rdf(subject)] attribute on struct fields

    // need test ensuring subject and predicate attributes are mutually exclusive on struct fields

    #[test]
    fn test_works() {
        let input_tokens: syn::DeriveInput = syn::parse2(quote! {
            #[rdf(type = "http://example.org/Person")]
            struct Person {
                #[rdf(subject)]
                subject: oxrdf::NamedOrBlankNode,

                #[rdf(predicate = "http://example.org/name")]
                name: String,

                #[rdf(predicate = "http://example.org/age")]
                age: u32,

                #[rdf(predicate = "http://example.org/dateOfDeath")]
                date_of_death: Option<String>,
            }
        })
        .unwrap();

        let expected = quote! {
            impl ::rdfreak::Entity for Person {
                fn get_rdf_type() -> ::oxrdf::NamedNode {
                    ::oxrdf::NamedNode::new_unchecked("http://example.org/Person")
                }

                fn get_subject(&self) -> &::oxrdf::NamedOrBlankNode {
                    &self.subject
                }

                fn serialize_properties(&self, graph: &mut ::oxrdf::Graph) {
                    let subject = ::rdfreak::Entity::get_subject(self);

                    ::rdfreak::RdfProperty::serialize_property(&self.name, graph, subject, &::oxrdf::NamedNode::new_unchecked("http://example.org/name"));
                    ::rdfreak::RdfProperty::serialize_property(&self.age, graph, subject, &::oxrdf::NamedNode::new_unchecked("http://example.org/age"));
                    ::rdfreak::RdfProperty::serialize_property(&self.date_of_death, graph, subject, &::oxrdf::NamedNode::new_unchecked("http://example.org/dateOfDeath"));
                }

                fn deserialize_properties(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode) -> ::rdfreak::DeserializeEntityResult<Self> {
                    Ok(Self {
                        subject: subject.clone(),
                        name: ::rdfreak::RdfProperty::deserialize_property(
                            graph,
                            subject,
                            &::oxrdf::NamedNode::new_unchecked("http://example.org/name"),
                        )
                        .map_err(
                            |err| ::rdfreak::DeserializeEntityError::FailedToDeserializeProperty {
                                property: "name".to_owned(),
                                subject: subject.clone(),
                                source: Box::new(err),
                            }
                        )?,
                        age: ::rdfreak::RdfProperty::deserialize_property(
                            graph,
                            subject,
                            &::oxrdf::NamedNode::new_unchecked("http://example.org/age"),
                        )
                        .map_err(
                            |err| ::rdfreak::DeserializeEntityError::FailedToDeserializeProperty {
                                property: "age".to_owned(),
                                subject: subject.clone(),
                                source: Box::new(err),
                            }
                        )?,
                        date_of_death: ::rdfreak::RdfProperty::deserialize_property(
                            graph,
                            subject,
                            &::oxrdf::NamedNode::new_unchecked("http://example.org/dateOfDeath"),
                        )
                        .map_err(
                            |err| ::rdfreak::DeserializeEntityError::FailedToDeserializeProperty {
                                property: "date_of_death".to_owned(),
                                subject: subject.clone(),
                                source: Box::new(err),
                            }
                        )?,
                    })
                }
            }

            impl ::rdfreak::RdfObject for Person {
                fn to_term(&self, graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                    ::rdfreak::Entity::serialize(self, graph);

                    let subject = ::rdfreak::Entity::get_subject(self);

                    match subject {
                        NamedOrBlankNode::NamedNode(named_node) => Term::NamedNode(named_node.clone()),
                        NamedOrBlankNode::BlankNode(blank_node) => Term::BlankNode(blank_node.clone()),
                    }
                }

                fn from_term(graph: &::oxrdf::Graph, term: &::oxrdf::Term) -> ::rdfreak::DeserializeRdfObjectResult<Self> {
                    let ::oxrdf::Term::NamedNode(named_node) = term else {
                        return Err(::rdfreak::DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
                    };

                    let value = <Self as ::rdfreak::Entity>::deserialize(graph, &::oxrdf::NamedOrBlankNode::NamedNode(named_node.clone()))?;

                    Ok(value)
                }
            }

            impl ::rdfreak::RdfProperty for Person {
                fn serialize_property(&self, graph: &mut ::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) {
                    let object_term = ::rdfreak::RdfObject::to_term(self, graph);

                    graph.insert(&::oxrdf::Triple::new(
                        subject.as_ref(),
                        predicate.as_ref(),
                        object_term,
                    ));
                }

                fn deserialize_property(graph: &::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) -> ::rdfreak::DeserializeRdfPropertyResult<Self> {
                    let object_term = graph
                        .object_for_subject_predicate(subject, predicate)
                        .ok_or_else(|| ::rdfreak::DeserializeRdfPropertyError::MissingObjectValue(predicate.clone()))?;

                    let value = <Self as ::rdfreak::RdfObject>::from_term(graph, &object_term.into_owned())?;

                    Ok(value)
                }
            }
        };

        let generated = derive_entity_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
