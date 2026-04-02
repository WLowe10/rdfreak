use darling::FromMeta;

/// gets the first attribute with the identifier "rdf" from a list of attributes
pub fn get_rdf_attribute(attributes: &[syn::Attribute]) -> Option<&syn::Attribute> {
    attributes.iter().find(|attr| attr.path().is_ident("rdf"))
}

// resource structs

#[derive(Debug, FromMeta)]
pub struct ResourceStructRdfAttributes {
    #[darling(rename = "type")]
    pub rdf_type: String,
}

/// parses the expected RDF attributes from a resource struct-level attribute
pub fn parse_resource_struct_rdf_attributes(
    input: &syn::DeriveInput,
) -> syn::Result<ResourceStructRdfAttributes> {
    let attr = get_rdf_attribute(&input.attrs).ok_or_else(|| {
        syn::Error::new_spanned(input, "Missing required attribute: #[rdf(type = \"...\")]")
    })?;

    ResourceStructRdfAttributes::from_meta(&attr.meta)
        .map_err(|err| syn::Error::new_spanned(attr, err))
}

#[derive(Debug, FromMeta)]
pub struct ResourceStructFieldRdfAttributes {
    #[darling(default, rename = "subject")]
    pub is_subject: bool,

    pub predicate: Option<String>,
}

/// parses the expected RDF attributes from a struct field-level attribute
pub fn parse_resource_struct_field_rdf_attributes(
    field: &syn::Field,
) -> syn::Result<ResourceStructFieldRdfAttributes> {
    let attr = get_rdf_attribute(&field.attrs).ok_or_else(|| {
        syn::Error::new_spanned(
            field,
            "Missing required attribute: #[rdf(predicate = \"...\")]",
        )
    })?;

    ResourceStructFieldRdfAttributes::from_meta(&attr.meta)
        .map_err(|err| syn::Error::new_spanned(attr, err))
}

pub fn validate_resource_struct_field_rdf_attributes(
    field: &syn::Field,
    attributes: &ResourceStructFieldRdfAttributes,
) -> syn::Result<()> {
    if attributes.is_subject && attributes.predicate.is_some() {
        return Err(syn::Error::new_spanned(
            field,
            "A field cannot have both #[rdf(subject)] and #[rdf(predicate = \"...\")] attributes",
        ));
    }

    Ok(())
}

pub fn validate_all_resource_struct_field_rdf_attributes(
    struct_input: &syn::DeriveInput,
    struct_definition: &syn::DataStruct,
    field_atttributes: &[ResourceStructFieldRdfAttributes],
) -> syn::Result<()> {
    let mut encountered_subject_field = false;

    // validate the fields for each attribute
    for (field, attributes) in struct_definition.fields.iter().zip(field_atttributes) {
        validate_resource_struct_field_rdf_attributes(field, attributes)?;

        if attributes.is_subject {
            if encountered_subject_field {
                return Err(syn::Error::new_spanned(
                    field,
                    "Only one field can have the #[rdf(subject)] attribute",
                ));
            }

            encountered_subject_field = true;
        }
    }

    if !encountered_subject_field {
        return Err(syn::Error::new_spanned(
            struct_input,
            "Missing required attribute: #[rdf(subject)] on a field",
        ));
    }

    Ok(())
}

// literal structs

#[derive(Debug, FromMeta)]
pub struct LiteralStructRdfAttributes {
    pub datatype: String,
}

/// parses the expected RDF attributes from a resource struct-level attribute
pub fn parse_literal_struct_rdf_attributes(
    input: &syn::DeriveInput,
) -> syn::Result<LiteralStructRdfAttributes> {
    let attr = get_rdf_attribute(&input.attrs).ok_or_else(|| {
        syn::Error::new_spanned(
            input,
            "Missing required attribute: #[rdf(datatype = \"...\")]",
        )
    })?;

    LiteralStructRdfAttributes::from_meta(&attr.meta)
        .map_err(|err| syn::Error::new_spanned(attr, err))
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote;

    #[test]
    fn test_validate_all_resource_struct_field_rdf_attributes_fails_with_no_subject() {
        let test_struct: syn::DeriveInput = parse_quote! {
            #[rdf(type = "http://example.com/Person")]
            struct Person {
                #[rdf(predicate = "http://example.com/predicate")]
                pub id: String
            }
        };

        let syn::Data::Struct(struct_data) = &test_struct.data else {
            panic!("Test struct should be a struct");
        };

        let field_rdf_attributes = struct_data
            .fields
            .iter()
            .map(parse_resource_struct_field_rdf_attributes)
            .collect::<Result<Vec<_>, syn::Error>>()
            .unwrap();

        let syn_result = validate_all_resource_struct_field_rdf_attributes(
            &test_struct,
            struct_data,
            &field_rdf_attributes,
        );

        let syn_err = syn_result.err().unwrap();

        pretty_assertions::assert_eq!(
            syn_err.to_string(),
            "Missing required attribute: #[rdf(subject)] on a field"
        )
    }

    #[test]
    fn test_validate_all_resource_struct_field_rdf_attributes_fails_with_multiple_subjects() {
        let test_struct: syn::DeriveInput = parse_quote! {
            #[rdf(type = "http://example.com/Person")]
            struct Person {
                #[rdf(subject)]
                pub subject1: ::oxrdf::NamedOrBlankNode,

                #[rdf(subject)]
                pub subject2: ::oxrdf::NamedOrBlankNode,

                #[rdf(predicate = "http://example.com/predicate")]
                pub id: String
            }
        };

        let syn::Data::Struct(struct_data) = &test_struct.data else {
            panic!("Test struct should be a struct");
        };

        let field_rdf_attributes = struct_data
            .fields
            .iter()
            .map(parse_resource_struct_field_rdf_attributes)
            .collect::<Result<Vec<_>, syn::Error>>()
            .unwrap();

        let syn_result = validate_all_resource_struct_field_rdf_attributes(
            &test_struct,
            struct_data,
            &field_rdf_attributes,
        );

        let syn_err = syn_result.err().unwrap();

        pretty_assertions::assert_eq!(
            syn_err.to_string(),
            "Only one field can have the #[rdf(subject)] attribute"
        );
    }

    #[test]
    fn test_validate_all_resource_struct_field_rdf_attributes_fails_with_invalid_field_attributes()
    {
        let test_struct: syn::DeriveInput = parse_quote! {
            #[rdf(type = "http://example.com/Person")]
            struct Person {
                #[rdf(subject, predicate = "http://example.com/predicate")]
                pub id: String
            }
        };

        let syn::Data::Struct(struct_data) = &test_struct.data else {
            panic!("Test struct should be a struct");
        };

        let field_rdf_attributes = struct_data
            .fields
            .iter()
            .map(parse_resource_struct_field_rdf_attributes)
            .collect::<Result<Vec<_>, syn::Error>>()
            .unwrap();

        let syn_result = validate_all_resource_struct_field_rdf_attributes(
            &test_struct,
            struct_data,
            &field_rdf_attributes,
        );

        let syn_err = syn_result.err().unwrap();

        pretty_assertions::assert_eq!(
            syn_err.to_string(),
            "A field cannot have both #[rdf(subject)] and #[rdf(predicate = \"...\")] attributes"
        );
    }
}
