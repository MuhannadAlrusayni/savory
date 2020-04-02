use crate::element::{
    attr::Attribute,
    field::{Field, FieldType},
};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, Token,
};

#[derive(Debug, Default)]
pub(crate) struct Lens {
    attributes: Vec<syn::Attribute>,
    rename: Option<Ident>,
    nested: bool,
}

impl Lens {
    pub(crate) fn expand_lens(
        input: &syn::DeriveInput,
        fields: &Vec<Field>,
    ) -> proc_macro2::TokenStream {
        let fields = fields
            .iter()
            .filter_map(|field| {
                field
                    .attrs
                    .iter()
                    .filter_map(|att| match att {
                        Attribute::Lens(lens) => Some(lens),
                    })
                    .nth(0)
                    .map(|lens| (field, lens))
            })
            .collect::<Vec<(&Field, &Lens)>>();

        let struct_name = &input.ident;

        if fields.is_empty() {
            return quote! {};
        }

        let (fields_def, fields_fill) = fields
            .iter()
            .map(|(field, lens)| {
                let field_name = &field.name;
                if lens.nested {
                    let struct_lens_name = match field.ty {
                        FieldType::Normal(ref ty) | FieldType::Option(ref ty) => {
                            if let syn::Type::Path(syn::TypePath{ ref path, .. }) = ty {
                                format_ident!(
                                    "{}Lens",
                                    path.segments
                                        .last()
                                        .map(|seg| seg.ident.clone())
                                        .expect("theme_lens(..) was not able to find nested type")
                                )
                            } else {
                                panic!("theme_lens(..) nested type is not supported")
                            }
                        }
                    };

                    match field.ty {
                        FieldType::Normal(_) => (
                            quote! { pub #field_name: #struct_lens_name<'lens>, },
                            quote! { #field_name: self.#field_name.theme_lens(), }
                        ),
                        FieldType::Option(_) => (
                            quote! { pub #field_name: Option<#struct_lens_name<'lens>>, },
                            quote! { #field_name: self.#field_name.as_ref().map(|val| val.theme_lens()), },
                        ),
                    }
                } else {
                    match field.ty {
                        FieldType::Normal(ref ty) => (
                            quote! { pub #field_name: &'lens #ty, },
                            quote! { #field_name: &self.#field_name, },
                        ),
                        FieldType::Option(ref ty) => (
                            quote! { pub #field_name: Option<&'lens #ty>, },
                            quote! { #field_name: self.#field_name.as_ref(), },
                        ),
                    }
                }
            })
            .fold(
                (vec![], vec![]),
                |(mut fields_def, mut fields_fill), (def, fill)| {
                    fields_def.push(def);
                    fields_fill.push(fill);
                    (fields_def, fields_fill)
                },
            );

        let mut generics = input.generics.clone();
        generics.params.push(
            syn::LifetimeDef::new(syn::Lifetime::new("'lens", proc_macro2::Span::call_site()))
                .into(),
        );
        let (impl_generics, ..) = generics.split_for_impl();
        let (.., ty_generics, where_clause) = input.generics.split_for_impl();

        let lens_struct_name = format_ident!("{}Lens", struct_name);
        quote! {
        pub struct #lens_struct_name<'lens> {
            #( #fields_def )*
        }

        impl #impl_generics ThemeLens<'lens> for #struct_name #ty_generics #where_clause {
            type Lens = #lens_struct_name<'lens>;

            fn theme_lens(&'lens self) -> #lens_struct_name<'lens> {
                    #lens_struct_name {
                        #( #fields_fill )*
                    }
                }
            }
        }
    }
}

impl Parse for Lens {
    fn parse(input: ParseStream) -> Result<Self> {
        let attr_name = "theme_lens";
        if input.parse::<Ident>()? != attr_name {
            return Err(input.error(format!("Expected `{}`", attr_name)));
        }

        // if there was no inner attrs or this the last attribute, reutrn default WriteFn
        if input.peek(Token![,]) || input.is_empty() {
            return Ok(Lens::default());
        }

        let mut lens = Lens::default();

        let inner;
        syn::parenthesized!(inner in input);

        lens.attributes = inner.call(syn::Attribute::parse_outer)?;

        while !inner.is_empty() {
            match inner.parse::<Ident>()?.to_string().as_ref() {
                "rename" => {
                    let _ = inner.parse::<Token![=]>()?;
                    match lens.rename {
                        Some(_) => {
                            return Err(inner
                                .error("`theme_lens` attribute have been renamed more than once"))
                        }
                        None => lens.rename = Some(inner.parse::<Ident>()?),
                    };
                }
                "nested" => {
                    if lens.nested {
                        return Err(
                            inner.error("`theme_lens` have received `nested` flag more than once")
                        );
                    }
                    lens.nested = true;
                }
                _ => Err(inner
                    .error("`theme_lens` attribute only accept `rename` and `copy` attributes"))?,
            }
            if inner.peek(Token![,]) {
                inner.parse::<Token![,]>()?;
            }
        }

        Ok(lens)
    }
}
