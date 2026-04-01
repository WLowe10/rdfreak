use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::{
    parse_struct_field_rdf_attributes, parse_struct_rdf_attributes,
    validate_all_struct_field_rdf_attributes,
};

pub fn derive_to_rdf_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(struct_data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "ToRdf can only be derived for structs",
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

    // generate code for serializing each property

    let serialize_property_statements = struct_data
        .fields
        .iter()
        .zip(&field_rdf_attributes)
        .filter(|(_, attr)| !attr.is_subject)
        .map(|(field, attr)| {
            let field_ident = field.ident.as_ref().unwrap();
            let predicate = attr.predicate.as_ref().unwrap();

            quote! {
                ::rdfreak::SerializeRdfProperty::serialize_property(&self.#field_ident, graph, subject, &::oxrdf::NamedNode::new_unchecked(#predicate));
            }
        })
        .collect::<Vec<_>>();

    let tokens = quote! {
        impl ::rdfreak::ToRdf for #struct_identifier {
            fn to_rdf(&self, graph: &mut ::oxrdf::Graph) {
                let subject = ::rdfreak::Entity::get_subject(self);

                graph.insert(&::oxrdf::Triple::new(
                    <Self as ::rdfreak::Entity>::get_subject(self).as_ref(),
                    ::oxrdf::NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    <Self as ::rdfreak::Entity>::get_rdf_type().as_ref(),
                ));

                #(#serialize_property_statements)*
            }
        }

        impl ::rdfreak::ToRdfObject for #struct_identifier {
            fn to_term(&self, graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                ::rdfreak::ToRdf::to_rdf(self, graph);

                let subject = ::rdfreak::Entity::get_subject(self);

                match subject {
                    NamedOrBlankNode::NamedNode(named_node) => Term::NamedNode(named_node.clone()),
                    NamedOrBlankNode::BlankNode(blank_node) => Term::BlankNode(blank_node.clone()),
                }
            }
        }

        impl ::rdfreak::SerializeRdfProperty for #struct_identifier {
            fn serialize_property(&self, graph: &mut ::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) {
                let object_term = ::rdfreak::ToRdfObject::to_term(self, graph);

                graph.insert(&::oxrdf::Triple::new(
                    subject.as_ref(),
                    predicate.as_ref(),
                    object_term,
                ));
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
            impl ::rdfreak::ToRdf for Person {
                fn to_rdf(&self, graph: &mut ::oxrdf::Graph) {
                    let subject = ::rdfreak::Entity::get_subject(self);

                    graph.insert(&::oxrdf::Triple::new(
                        <Self as ::rdfreak::Entity>::get_subject(self).as_ref(),
                        ::oxrdf::NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                        <Self as ::rdfreak::Entity>::get_rdf_type().as_ref(),
                    ));

                    ::rdfreak::SerializeRdfProperty::serialize_property(&self.name, graph, subject, &::oxrdf::NamedNode::new_unchecked("http://example.org/name"));
                    ::rdfreak::SerializeRdfProperty::serialize_property(&self.age, graph, subject, &::oxrdf::NamedNode::new_unchecked("http://example.org/age"));
                    ::rdfreak::SerializeRdfProperty::serialize_property(&self.occupation, graph, subject, &::oxrdf::NamedNode::new_unchecked("http://example.org/occupation"));
                }
            }

            impl ::rdfreak::ToRdfObject for Person {
                fn to_term(&self, graph: &mut ::oxrdf::Graph) -> ::oxrdf::Term {
                    ::rdfreak::ToRdf::to_rdf(self, graph);

                    let subject = ::rdfreak::Entity::get_subject(self);

                    match subject {
                        NamedOrBlankNode::NamedNode(named_node) => Term::NamedNode(named_node.clone()),
                        NamedOrBlankNode::BlankNode(blank_node) => Term::BlankNode(blank_node.clone()),
                    }
                }
            }

            impl ::rdfreak::SerializeRdfProperty for Person {
                fn serialize_property(&self, graph: &mut ::oxrdf::Graph, subject: &::oxrdf::NamedOrBlankNode, predicate: &::oxrdf::NamedNode) {
                    let object_term = ::rdfreak::ToRdfObject::to_term(self, graph);

                    graph.insert(&::oxrdf::Triple::new(
                        subject.as_ref(),
                        predicate.as_ref(),
                        object_term,
                    ));
                }
            }
        };

        let generated = derive_to_rdf_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
