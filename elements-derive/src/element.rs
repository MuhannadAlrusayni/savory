use darling::FromDeriveInput;
use darling::{
    ast,
    usage::{CollectLifetimes, CollectTypeParams, GenericsExt, Purpose},
    util::{self, Override},
    FromMeta,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::{collections::HashMap, default::Default};
use syn::{parse_quote, parse_str};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(element), supports(struct_any))]
pub(crate) struct Element {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<util::Ignored, Field>,
    #[darling(default)]
    style_map: Option<Style>,
    #[darling(default)]
    config_bound: Option<Vec<syn::WherePredicate>>,
    #[darling(default)]
    view: util::Flag,
}

#[derive(Debug, FromField)]
#[darling(attributes(element))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    config: Option<Override<Config>>,
    #[darling(default)]
    data_lens: Option<Override<Lens>>,
}

#[derive(FromMeta, Debug, Default, Clone)]
struct Lens {
    #[darling(default)]
    nested: util::Flag,
    #[darling(default)]
    copy: util::Flag,
}

#[derive(Debug, Default, Clone)]
struct Style {
    fields: HashMap<String, Option<syn::Path>>,
}

#[derive(FromMeta, Debug, Default, Clone)]
struct Config {
    #[darling(default)]
    default: Option<Override<String>>,
    #[darling(default)]
    required: util::Flag,
    #[darling(default)]
    nested: util::Flag,
    #[darling(default)]
    no_pub: util::Flag,
}

uses_type_params!(Field, ty);
uses_lifetimes!(Field, ty);

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lens_impl = self.get_data_lens_impl();
        let config_impl = self.get_config_impl();
        let style_map_impl = self.get_style_map_impl();

        tokens.extend(quote! {
            #config_impl
            #style_map_impl
            #lens_impl
        })
    }
}

impl Element {
    fn get_style_map_impl(&self) -> TokenStream {
        if let Some(ref style) = self.style_map {
            let fields = style
                .fields
                .iter()
                .map(|(name, ty)| {
                    let name = syn::Ident::new(name, proc_macro2::Span::call_site());
                    let ty = ty
                        .as_ref()
                        .map(|ty| quote! { #ty })
                        .unwrap_or_else(|| quote! { savory_style::Style });
                    quote! { pub #name: #ty, }
                })
                .collect::<Vec<_>>();
            quote! {
                #[derive(Clone, Debug, Default, PartialEq, Rich)]
                pub struct StyleMap {
                    #(
                        #[rich(write(style = compose), write)]
                        #fields
                    )*
                }
            }
        } else {
            quote! {}
        }
    }
    fn get_config_impl(&self) -> TokenStream {
        let lifetimes = self.generics.declared_lifetimes();
        let params = self.generics.declared_type_params();

        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("`Element` doesn't work with enum yet")
            .fields;

        let config_fields = fields
            .iter()
            .filter(|item| item.config.is_some())
            .collect::<Vec<&&Field>>();

        let mut new_args = vec![];
        let mut new_fill = vec![];
        let mut pass_new_args = vec![];
        let mut struct_fields = vec![];
        for field in config_fields.iter() {
            if let Some(config) = field.config.clone().map(|val| val.unwrap_or_default()) {
                let ty = &field.ty;
                let field = &field.ident;

                let nested_config_or_type = |ty| {
                    if config.nested.is_some() {
                        quote! { <#ty as Element>::Config }
                    } else {
                        quote! { #ty }
                    }
                };

                let ty = ty.get_option_ty().unwrap_or(ty);
                let ty = nested_config_or_type(ty);
                if config.required.is_some() {
                    if config.default.is_some() {
                        panic!("`default` attribute cannot be used with `required` attribute")
                    }
                    if config.no_pub.is_some() {
                        panic!("`no_pub` attribute cannot be used with `required` attribute")
                    }

                    pass_new_args.push(quote! { #field, });
                    struct_fields.push(quote! {
                        #[rich(write, write(style = compose))]
                        pub #field: #ty,
                    });
                    new_args.push(quote! { #field: impl Into<#ty>, });
                    new_fill.push(quote! { #field: #field.into(), });
                } else {
                    if let Some(ref expr) = config.default {
                        let def_expr = match expr.as_ref() {
                            Override::Inherit => quote! { ::std::default::Default::default() },
                            Override::Explicit(expr_str) => {
                                let expr = parse_str::<syn::Expr>(&expr_str)
                                    .expect("Expect expr in as value for `default` attribute");
                                quote! { (#expr).into() }
                            }
                        };
                        new_fill.push(quote! { #field: #def_expr, });
                        if config.no_pub.is_none() {
                            struct_fields.push(quote! {
                                #[rich(write, write(style = compose))]
                                pub #field: #ty,
                            })
                        } else {
                            struct_fields.push(quote! {
                                #field: #ty,
                            })
                        };
                    } else {
                        if config.no_pub.is_none() {
                            struct_fields.push(quote! {
                                #[rich(write, write(option), write(option, style = compose))]
                                pub #field: Option<#ty>,
                            });
                        } else {
                            struct_fields.push(quote! {
                                #field: Option<#ty>,
                            })
                        };
                        new_fill.push(quote! { #field: None, });
                    }
                }
            }
        }

        let mut needed_gen = self
            .generics
            .params
            .clone()
            .into_iter()
            .map(|param| (param, false))
            .collect::<Vec<_>>();

        for lifetime in config_fields
            .clone()
            .into_iter()
            .cloned()
            .collect_lifetimes_cloned(&Purpose::Declare.into(), &lifetimes)
        {
            let lifetime: syn::GenericParam = parse_quote! { #lifetime };
            for (param, needed) in needed_gen.iter_mut() {
                if *param == lifetime {
                    *needed = true;
                    break;
                }
            }
        }
        for param in config_fields
            .clone()
            .into_iter()
            .cloned()
            .collect_type_params_cloned(&Purpose::Declare.into(), &params)
        {
            let got_param: syn::GenericParam = parse_quote! { #param };
            for (param, needed) in needed_gen.iter_mut() {
                if *param == got_param {
                    *needed = true;
                    break;
                }
            }
        }

        let needed_gen = needed_gen
            .into_iter()
            .filter_map(|(param, needed)| if needed { Some(param) } else { None })
            .collect::<syn::punctuated::Punctuated<_, syn::Token![,]>>();

        let config_ty_gen = if !needed_gen.is_empty() {
            quote! { < #needed_gen > }
        } else {
            quote! {}
        };

        let config_where_clause = self
            .config_bound
            .as_ref()
            .map(|b| quote! { where #( #b, )* });

        let (ty_impl, ty_gen, where_clause) = self.generics.split_for_impl();
        let element_name = &self.ident;

        // FIXME: I think view shoudn't have Config struct, therefor we shoudn't
        // generate any code realted to Config or even the Config struct it self
        let config_init_method = if self.view.is_none() {
            quote! {
                pub fn init(self, orders: &mut impl Orders<<#element_name #ty_gen as Element>::Message>) -> #element_name #ty_gen #where_clause {
                    #element_name::init(self, orders)
                }
            }
        } else {
            quote! {}
        };

        if !config_fields.is_empty() {
            quote! {
                impl #ty_impl #element_name #ty_gen #where_clause {
                    pub fn config(#( #new_args )*) -> Config #config_ty_gen {
                        Config::new(#( #pass_new_args )*)
                    }
                }

                #[derive(Rich)]
                pub struct Config #config_ty_gen #config_where_clause {
                    #( #struct_fields )*
                }

                impl #config_ty_gen Config #config_ty_gen #config_where_clause {
                    pub fn new(#( #new_args )*) -> Self {
                        Self {
                            #( #new_fill )*
                        }
                    }

                    #config_init_method
                }
            }
        } else {
            quote! {
                impl #ty_impl #element_name #ty_gen #where_clause {
                    pub fn config() -> Config {
                        Config
                    }
                }

                pub struct Config;

                impl Config {
                    pub fn new() -> Self {
                        Config
                    }

                    #config_init_method
                }
            }
        }
    }
    fn get_data_lens_impl(&self) -> TokenStream {
        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("`Element` doesn't work with enum yet")
            .fields;

        let lens_fields = fields
            .iter()
            .filter(|item| item.data_lens.is_some())
            .collect::<Vec<_>>();

        if lens_fields.is_empty() {
            return quote! {};
        }

        let (fields_def, fields_in_new_fn) = lens_fields
                .iter()
                .filter_map(|field| {
                    field
                        .data_lens
                        .clone()
                        .map(|data_lens| data_lens.unwrap_or_default())
                        .map(|data_lens| {
                            let name = &field.ident;
                            let ty = &field.ty;
                            match ty.get_option_ty() {
                                Some(ty) => {
                                    let struct_lens_name = format_ident!("{}Lens", ty.name());
                                    if data_lens.nested.is_some() {
                                        if data_lens.copy.is_some() {
                                            panic!("nested lens cannot be copyable");
                                        }
                                        let field_def = quote! { pub #name: Option<#struct_lens_name<'lens>>, };
                                        let field_in_new_fn = quote! { #name: self.#name.as_ref().map(|val| val.data_lens()), };
                                        (field_def, field_in_new_fn)
                                    } else {
                                        if data_lens.copy.is_some() {
                                            let field_def = quote! { pub #name: Option<#ty>, };
                                            let field_in_new_fn = quote! { #name: self.#name, };
                                            (field_def, field_in_new_fn)
                                        } else {
                                            let field_def = quote! { pub #name: Option<&'lens #ty>, };
                                            let field_in_new_fn = quote! { #name: self.#name.as_ref(), };
                                            (field_def, field_in_new_fn)
                                        }
                                    }
                                }
                                None => {
                                    if data_lens.nested.is_some() {
                                        if data_lens.copy.is_some() {
                                            panic!("nested lens cannot be copyable");
                                        }
                                        let struct_lens_name = format_ident!("{}Lens", ty.name());
                                        let field_def =
                                            quote! { pub #name: #struct_lens_name<'lens>, };
                                        let field_in_new_fn =
                                            quote! { #name: self.#name.data_lens(), };
                                        (field_def, field_in_new_fn)
                                    } else {
                                        if data_lens.copy.is_some() {
                                            let field_def = quote! { pub #name: #ty, };
                                            let field_in_new_fn = quote! { #name: self.#name, };
                                            (field_def, field_in_new_fn)
                                        } else {
                                            let field_def = quote! { pub #name: &'lens #ty, };
                                            let field_in_new_fn = quote! { #name: &self.#name, };
                                            (field_def, field_in_new_fn)
                                        }
                                    }
                                }
                            }                        })
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();

        let mut generics = self.generics.clone();
        let (lens_lifetime, plain_lens_lifetime) = if !lens_fields.iter().all(|f| {
            f.data_lens
                .as_ref()
                .map_or(false, |dl| dl.clone().unwrap_or_default().copy.is_some())
        }) {
            generics.params.push(
                syn::LifetimeDef::new(syn::Lifetime::new("'lens", proc_macro2::Span::call_site()))
                    .into(),
            );
            (quote! { <'lens> }, quote! { 'lens })
        } else {
            (quote! {}, quote! {})
        };
        let (impl_generics, ..) = generics.split_for_impl();
        let (.., ty_generics, where_clause) = self.generics.split_for_impl();

        let struct_name = &self.ident;
        let lens_struct_name = format_ident!("{}Lens", &self.ident);
        quote! {
            pub struct #lens_struct_name #lens_lifetime {
                #( #fields_def )*
            }

            impl #impl_generics #struct_name #ty_generics #where_clause {
                fn data_lens(& #plain_lens_lifetime self) -> #lens_struct_name #lens_lifetime {
                    #lens_struct_name {
                        #( #fields_in_new_fn )*
                    }
                }
            }

            // impl #impl_generics ThemeLens<'lens> for #struct_name #ty_generics #where_clause {
            //     type Lens = #lens_struct_name<'lens>;

            //     fn data_lens(&'lens self) -> #lens_struct_name<'lens> {
            //         #lens_struct_name {
            //             #( #fields_in_new_fn )*
            //         }
            //     }
            // }
        }
    }
}

impl FromMeta for Style {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut map = HashMap::default();

        for meta in items.iter() {
            match meta {
                syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                    if let Some(ident) = path.get_ident() {
                        let name = ident.to_string();
                        if map.insert(name.clone(), None).is_some() {
                            return Err(darling::Error::duplicate_field(&name));
                        }
                    } else {
                        return Err(darling::Error::unsupported_format("expect an ident"));
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList { path, nested, .. })) => {
                    if let Some(ident) = path.get_ident() {
                        let name = ident.to_string();
                        match nested.first() {
                            Some(syn::NestedMeta::Meta(syn::Meta::Path(path)))
                                if nested.len() == 1 =>
                            {
                                if map.insert(name.clone(), Some(path.clone())).is_some() {
                                    return Err(darling::Error::duplicate_field(&name));
                                }
                            }
                            _ => {
                                return Err(darling::Error::custom("unexpected style field format"))
                            }
                        }
                    } else {
                        return Err(darling::Error::unsupported_format("expect an ident"));
                    }
                }
                _ => return Err(darling::Error::unsupported_format("expect field name")),
            }
        }

        Ok(Style { fields: map })
    }
}
pub trait TypeExt {
    fn get_option_ty(&self) -> Option<&syn::Type>;
    fn path_only(&self) -> Option<syn::punctuated::Punctuated<&syn::Ident, syn::Token![,]>>;
    fn name(&self) -> &syn::Ident;
}

impl TypeExt for syn::Type {
    fn get_option_ty(&self) -> Option<&syn::Type> {
        match self {
            syn::Type::Path(syn::TypePath {
                path: syn::Path { ref segments, .. },
                ..
            }) => {
                let last_segment = segments.iter().last();
                match last_segment {
                    Some(syn::PathSegment {
                        ident,
                        arguments:
                            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                args,
                                ..
                            }),
                    }) => {
                        if ident != "Option" {
                            None
                        } else {
                            let first_arg = args.into_iter().next();
                            match first_arg {
                                Some(syn::GenericArgument::Type(ty)) => Some(ty),
                                _ => None,
                            }
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn path_only(&self) -> Option<syn::punctuated::Punctuated<&syn::Ident, syn::Token![,]>> {
        match self {
            Self::Path(syn::TypePath {
                path: syn::Path { segments, .. },
                ..
            }) => Some(
                segments
                    .iter()
                    .map(|seg| &seg.ident)
                    .collect::<syn::punctuated::Punctuated<_, _>>(),
            ),
            _ => None,
        }
    }

    fn name(&self) -> &syn::Ident {
        if let syn::Type::Path(syn::TypePath { path, .. }) = &self {
            if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
                return ident;
            }
        }
        panic!("Expected type to be a `Path` with a segment.",);
    }
}
