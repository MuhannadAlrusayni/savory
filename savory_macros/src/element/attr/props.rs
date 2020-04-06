use crate::element::{
    attr::Attribute,
    field::{Field, FieldType},
};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, Token,
};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum State {
    Optinal { default: Option<syn::Expr> },
    Reqired,
}

impl Default for State {
    fn default() -> Self {
        Self::Optinal { default: None }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Props {
    attributes: Vec<syn::Attribute>,
    state: State,
}

impl Props {
    pub(crate) fn expand_props(
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
                        Attribute::Props(props) => Some(props),
                        _ => None,
                    })
                    .nth(0)
                    .map(|props| (field, props))
            })
            .collect::<Vec<(&Field, &Props)>>();

        if fields.is_empty() {
            return quote! {};
        }

        let mut new_args = vec![];
        let mut new_fill = vec![];
        let mut props_fields = vec![];
        let mut props_fns = vec![];
        // let mut generics =
        //     syn::punctuated::Punctuated::<syn::GenericParam, syn::Token![,]>::new();
        let mut generics_params: syn::punctuated::Punctuated<
            (&syn::GenericParam, bool),
            syn::Token![,],
        > = input
            .generics
            .params
            .iter()
            .map(|param| (param, false))
            .collect();

        for (field, props) in fields.iter() {
            let field_name = &field.name;
            let mut get_generics = |ty: &syn::Type| {
                if let syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                }) = ty
                {
                    if let Some(seg) = segments.last() {
                        if let syn::PathArguments::AngleBracketed(ref gen) = seg.arguments {
                            for gen in gen.args.iter() {
                                match gen {
                                    syn::GenericArgument::Lifetime(lifetime) => {
                                        for (parama, ref mut needed) in generics_params.iter_mut() {
                                            if let syn::GenericParam::Lifetime(lifetime_def) =
                                                parama
                                            {
                                                if &lifetime_def.lifetime == lifetime {
                                                    *needed = true;
                                                }
                                            }
                                        }
                                    }
                                    syn::GenericArgument::Type(syn::Type::Path(
                                        syn::TypePath { path, .. },
                                    )) => {
                                        for (parama, ref mut needed) in generics_params.iter_mut() {
                                            if let syn::GenericParam::Type(type_param) = parama {
                                                if let Some(ident) = path.get_ident() {
                                                    if &type_param.ident == ident {
                                                        *needed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            };

            match (&field.ty, &props.state) {
                (FieldType::Normal(ref ty), State::Optinal { ref default })
                | (FieldType::Option(ref ty), State::Optinal { ref default }) => {
                    if let Some(expr) = default {
                        props_fields.push(quote! { pub #field_name: #ty, });
                        new_fill.push(quote! { #field_name: (#expr).into(), });
                        get_generics(&ty);
                        props_fns.push(quote! {
                            pub fn #field_name(mut self, val: impl Into<#ty>) -> Self {
                                self.#field_name = val.into();
                                self
                            }
                        })
                    } else {
                        props_fields.push(quote! { pub #field_name: Option<#ty>, });
                        new_fill.push(quote! { #field_name: None, });
                        get_generics(&ty);
                        props_fns.push(quote! {
                            pub fn #field_name(mut self, val: impl Into<#ty>) -> Self {
                                self.#field_name = Some(val.into());
                                self
                            }
                        })
                    }
                }
                (FieldType::Normal(ref ty), State::Reqired) => {
                    props_fields.push(quote! { pub #field_name: #ty, });
                    new_args.push(quote! { #field_name: impl Into<#ty>, });
                    new_fill.push(quote! { #field_name: #field_name.into(), });
                    get_generics(&ty);
                    props_fns.push(quote! {
                        pub fn #field_name(mut self, val: impl Into<#ty>) -> Self {
                            self.#field_name = val.into();
                            self
                        }
                    })
                }
                (FieldType::Option(ref ty), State::Reqired) => {
                    props_fields.push(quote! { pub #field_name: Option<#ty>, });
                    new_args.push(quote! { #field_name: impl Into<#ty>, });
                    new_fill.push(quote! { #field_name: Some(#field_name.into()), });
                    get_generics(&ty);
                    props_fns.push(quote! {
                        pub fn #field_name(mut self, val: impl Into<#ty>) -> Self {
                            self.#field_name = Some(val.into());
                            self
                        }
                    })
                }
            }
        }

        let generics_params: syn::punctuated::Punctuated<&syn::GenericParam, syn::Token![,]> =
            generics_params
                .iter()
                .filter_map(|(param, needed)| if *needed { Some(*param) } else { None })
                .collect();
        let generics = quote! { <#generics_params> };

        quote! {
            pub struct Props #generics {
                #( #props_fields )*
            }

            impl #generics Props #generics {
                pub fn new(#( #new_args )*) -> Self {
                    Self {
                        #( #new_fill )*
                    }
                }

                #( #props_fns )*
            }
        }
    }
}

impl Parse for Props {
    fn parse(input: ParseStream) -> Result<Self> {
        let attr_name = "props";
        if input.parse::<Ident>()? != attr_name {
            return Err(input.error(format!("Expected `{}`", attr_name)));
        }

        let mut props = Props::default();

        // if there was no inner attrs or this the last attribute, reutrn default props
        if input.peek(Token![,]) || input.is_empty() {
            return Ok(props);
        }

        let inner;
        syn::parenthesized!(inner in input);

        props.attributes = inner.call(syn::Attribute::parse_outer)?;

        while !inner.is_empty() {
            match inner.parse::<Ident>()?.to_string().as_ref() {
                "default" => {
                    let _ = inner.parse::<Token![=]>()?;
                    match props.state {
                        State::Reqired => {
                            return Err(inner
                                .error("`props` cannot have `default` and `reqired` at same time"))
                        }
                        State::Optinal { default: Some(_) } => {
                            return Err(inner
                                .error("`props` have received `default` attribute more than once"))
                        }
                        State::Optinal { default: None } => {
                            props.state = State::Optinal {
                                default: Some(inner.parse::<syn::Expr>()?),
                            };
                        }
                    };
                }
                "reqired" => {
                    match props.state {
                        State::Reqired => {
                            return Err(inner
                                .error("`props` have received `reqired` attribute more than once"))
                        }
                        State::Optinal { default: Some(_) } => {
                            return Err(inner
                                .error("`props` cannot have `default` and `reqired` at same time"))
                        }
                        State::Optinal { default: None } => props.state = State::Reqired,
                    }
                }
                _ => Err(inner.error(
                    "`props` attribute only accept `default = ..` and `reqired` attributes",
                ))?,
            }
            if inner.peek(Token![,]) {
                inner.parse::<Token![,]>()?;
            }
        }

        Ok(props)
    }
}
