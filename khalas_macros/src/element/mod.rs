use quote::quote;

pub mod attr;
pub mod field;

use attr::lens::Lens;
use field::Field;

pub fn expand_element(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let result = match input.data {
        syn::Data::Enum(_) => panic!("doesn't work with enums yet"),
        syn::Data::Struct(ref s) => element_for_struct(input, &s.fields),
        syn::Data::Union(_) => panic!("doesn't work with unions yet"),
    };
    result.into()
}

fn element_for_struct(
    input: &syn::DeriveInput,
    fields: &syn::Fields,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match *fields {
        syn::Fields::Named(ref fields) => element_impl(&input, &fields.named),
        syn::Fields::Unnamed(_) => panic!("doesn't work with tuple struct yet"),
        syn::Fields::Unit => panic!("doesn't work with unit struct yet"),
    }
}

fn element_impl(
    input: &syn::DeriveInput,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let fields = fields
        .iter()
        .map(|f| Field::from_syn_field(f))
        .collect::<Vec<Result<Field, syn::Error>>>()
        .into_iter()
        .collect::<Result<Vec<Field>, syn::Error>>()?;

    let lens_impl = Lens::expand_lens(input, &fields);

    Ok(quote! {
        #lens_impl
    })
}
