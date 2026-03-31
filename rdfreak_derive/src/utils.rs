use darling::FromMeta;

/// gets the first attribute with the identifier "rdf" from a list of attributes
pub fn get_rdf_attribute(attributes: &[syn::Attribute]) -> Option<&syn::Attribute> {
    attributes.iter().find(|attr| attr.path().is_ident("rdf"))
}

#[derive(Debug, FromMeta)]
pub struct StructFieldRdfAttributes {
    #[darling(default, rename = "subject")]
    pub is_subject: bool,

    pub predicate: Option<String>,
}

/// parses the expected RDF attributes from a struct field-level attribute
pub fn parse_struct_field_rdf_attributes(
    field: &syn::Field,
) -> syn::Result<StructFieldRdfAttributes> {
    let attr = get_rdf_attribute(&field.attrs).ok_or_else(|| {
        syn::Error::new_spanned(
            field,
            "Missing required attribute: #[rdf(predicate = \"...\")]",
        )
    })?;

    StructFieldRdfAttributes::from_meta(&attr.meta)
        .map_err(|err| syn::Error::new_spanned(attr, err))
}
