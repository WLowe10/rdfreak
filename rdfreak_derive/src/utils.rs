/// gets the first attribute with the identifier "rdf" from a list of attributes
pub fn get_rdf_attribute(attributes: &[syn::Attribute]) -> Option<&syn::Attribute> {
    attributes.iter().find(|attr| attr.path().is_ident("rdf"))
}
