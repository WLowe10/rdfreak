mod utils;

mod derive_constructible_impl;
mod derive_entity_impl;
mod derive_from_rdf_impl;
mod derive_to_rdf_impl;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use derive_constructible_impl::derive_constructible_impl;
use derive_entity_impl::derive_entity_impl;
use derive_from_rdf_impl::derive_from_rdf_impl;
use derive_to_rdf_impl::derive_to_rdf_impl;

#[proc_macro_derive(Entity, attributes(rdf))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_entity_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ToRdf, attributes(rdf))]
pub fn derive_to_rdf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_to_rdf_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(FromRdf, attributes(rdf))]
pub fn derive_from_rdf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_from_rdf_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Constructible, attributes(rdf))]
pub fn derive_constructible(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_constructible_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
