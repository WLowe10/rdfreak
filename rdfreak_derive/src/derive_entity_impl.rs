use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::{
    parse_struct_field_rdf_attributes, parse_struct_rdf_attributes,
    validate_all_struct_field_rdf_attributes,
};

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

    let tokens = quote! {
        impl ::rdfreak::Entity for #struct_identifier {
            fn get_rdf_type() -> ::oxrdf::NamedNode {
                ::oxrdf::NamedNode::new_unchecked(#struct_rdf_type)
            }

            fn get_subject(&self) -> &::oxrdf::NamedOrBlankNode {
                &self.#subject_identifier
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
        let input_tokens: syn::DeriveInput = syn::parse_quote! {
            enum StopLight {
                Green,
                Yellow,
                Red,
            }
        };

        let derive_error = derive_entity_impl(input_tokens).err().unwrap();

        assert!(
            derive_error
                .to_string()
                .contains("Entity can only be derived for structs")
        );
    }

    #[test]
    fn test_fails_missing_rdf_attributes() {
        let input_tokens: syn::DeriveInput = syn::parse_quote! {
            struct Person {
                #[rdf(subject)]
                subject: oxrdf::NamedOrBlankNode,

                #[rdf(predicate = "http://example.org/name")]
                name: String,

                #[rdf(predicate = "http://example.org/age")]
                age: u32,
            }
        };

        let derive_error = derive_entity_impl(input_tokens).err().unwrap();

        pretty_assertions::assert_eq!(
            derive_error.to_string(),
            "Missing required attribute: #[rdf(type = \"...\")]"
        );
    }

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
            impl ::rdfreak::Entity for Person {
                fn get_rdf_type() -> ::oxrdf::NamedNode {
                    ::oxrdf::NamedNode::new_unchecked("http://example.org/Person")
                }

                fn get_subject(&self) -> &::oxrdf::NamedOrBlankNode {
                    &self.subject
                }
            }
        };

        let generated = derive_entity_impl(input_tokens).unwrap();

        pretty_assertions::assert_eq!(generated.to_string(), expected.to_string());
    }
}
