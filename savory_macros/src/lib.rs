#[macro_use]
extern crate darling;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use darling::FromDeriveInput;
use proc_macro::TokenStream;

pub(crate) mod element;

#[proc_macro_derive(Element, attributes(element))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let element_impl = match element::Element::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };
    (quote! {
        #element_impl
    })
    .into()
}
