pub(crate) mod element;

extern crate proc_macro;

use crate::element::expand_element;
use proc_macro::TokenStream;

#[proc_macro_derive(Element, attributes(element))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    expand_element(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
