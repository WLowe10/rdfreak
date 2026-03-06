mod derive_entity_impl;
mod utils;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use derive_entity_impl::derive_entity_impl;

#[proc_macro_derive(Entity, attributes(rdf))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_entity_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
