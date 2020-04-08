use darling::FromDeriveInput;
use darling::{
    ast,
    usage::{CollectLifetimes, CollectTypeParams, GenericsExt, Purpose},
    util::{self, Override},
    FromMeta,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::default::Default;
use syn::{parse_quote, parse_str};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(element), supports(struct_any))]
pub(crate) struct Element {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<util::Ignored, Field>,
    #[darling(default)]
    style: Option<StringSet>,
    #[darling(default)]
    events: Option<StringSet>,
}

#[derive(Debug, Default, Clone)]
struct StringSet(Vec<String>);

#[derive(Debug, FromField)]
#[darling(attributes(element))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    theme_lens: Option<Override<Lens>>,
    #[darling(default)]
    props: Option<Override<Props>>,
}

#[derive(FromMeta, Debug, Default, Clone)]
struct Lens {
    #[darling(default)]
    nested: util::Flag,
}

#[derive(FromMeta, Debug, Default, Clone)]
struct Props {
    #[darling(default)]
    default: Option<String>,
    #[darling(default)]
    required: util::Flag,
}

uses_type_params!(Field, ty);
uses_lifetimes!(Field, ty);

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lens_impl = self.get_lens_impl();
        let props_impl = self.get_props_impl();
        let style_impl = self.get_style_impl();
        let events_impl = self.get_events_impl();

        tokens.extend(quote! {
            #lens_impl
            #props_impl
            #style_impl
            #events_impl
        })
    }
}

impl Element {
    fn get_events_impl(&self) -> TokenStream {
        if let Some(ref events) = self.events {
            let events = events
                .0
                .iter()
                .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                .collect::<Vec<_>>();
            quote! {
                #[derive(Rich)]
                pub struct Events<Msg> {
                    #(
                        #[rich(write(style = compose), write)]
                        pub #events: savory_html::events::Events<Msg>,
                    )*
                }

                impl<Msg> Clone for Events<Msg> {
                    fn clone(&self) -> Self {
                        Self {
                            #(
                                #events: self.#events.clone(),
                            )*
                        }
                    }
                }

                impl<Msg> Default for Events<Msg> {
                    fn default() -> Self {
                        Self {
                            #(
                                #events: savory_html::events::Events::default(),
                            )*
                        }
                    }
                }
            }
        } else {
            quote! {}
        }
    }

    fn get_style_impl(&self) -> TokenStream {
        if let Some(ref style) = self.style {
            let styles = style
                .0
                .iter()
                .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                .collect::<Vec<_>>();
            quote! {
                #[derive(Clone, Default, PartialEq, Rich)]
                pub struct Style {
                    #(
                        #[rich(write(style = compose), write)]
                        pub #styles: savory_html::css::Style,
                    )*
                }
            }
        } else {
            quote! {}
        }
    }

    fn get_lens_impl(&self) -> TokenStream {
        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("`Element` doesn't work with enum yet")
            .fields;

        let lens_fields = fields
            .iter()
            .filter(|item| item.theme_lens.is_some())
            .collect::<Vec<_>>();

        if lens_fields.is_empty() {
            return quote! {};
        }

        let (fields_def, fields_in_new_fn) = lens_fields
                .iter()
                .filter_map(|field| {
                    field
                        .theme_lens
                        .clone()
                        .map(|theme_lens| theme_lens.unwrap_or_default())
                        .map(|theme_lens| {
                            let name = &field.ident;
                            let ty = &field.ty;
                            let struct_lens_name =
                                format_ident!("{}theme_lens", field.ty.name());
                            match ty.get_option_ty() {
                                Some(ty) => {
                                    if theme_lens.nested.is_some() {
                                        let field_def = quote! { pub #name: Option<#struct_lens_name<'lens>>, };
                                        let field_in_new_fn = quote! { #name: self.#name.as_ref().map(|val| val.theme_lens()), };
                                        (field_def, field_in_new_fn)
                                    } else {
                                        let field_def = quote! { pub #name: Option<&'lens #ty>, };
                                        let field_in_new_fn = quote! { #name: self.#name.as_ref(), };
                                        (field_def, field_in_new_fn)
                                    }
                                }
                                None => {
                                    if theme_lens.nested.is_some() {
                                        let field_def =
                                            quote! { pub #name: #struct_lens_name<'lens>, };
                                        let field_in_new_fn =
                                            quote! { #name: self.#name.theme_lens(), };
                                        (field_def, field_in_new_fn)
                                    } else {
                                        let field_def = quote! { pub #name: &'lens #ty, };
                                        let field_in_new_fn = quote! { #name: &self.#name, };
                                        (field_def, field_in_new_fn)
                                    }
                                }
                            }
                        })
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();

        let mut generics = self.generics.clone();
        generics.params.push(
            syn::LifetimeDef::new(syn::Lifetime::new("'lens", proc_macro2::Span::call_site()))
                .into(),
        );
        let (impl_generics, ..) = generics.split_for_impl();
        let (.., ty_generics, where_clause) = self.generics.split_for_impl();

        let struct_name = &self.ident;
        let lens_struct_name = format_ident!("{}Lens", &self.ident);
        quote! {
            pub struct #lens_struct_name<'lens> {
                #( #fields_def )*
            }

            impl #impl_generics ThemeLens<'lens> for #struct_name #ty_generics #where_clause {
                type Lens = #lens_struct_name<'lens>;

                fn theme_lens(&'lens self) -> #lens_struct_name<'lens> {
                    #lens_struct_name {
                        #( #fields_in_new_fn )*
                    }
                }
            }
        }
    }

    fn get_props_impl(&self) -> TokenStream {
        let lifetimes = self.generics.declared_lifetimes();
        let params = self.generics.declared_type_params();

        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("`Element` doesn't work with enum yet")
            .fields;

        let props_fields = fields
            .iter()
            .filter(|item| item.props.is_some())
            .collect::<Vec<&&Field>>();

        if props_fields.is_empty() {
            return quote! {};
        }

        let mut new_args = vec![];
        let mut new_fill = vec![];
        let mut struct_fields = vec![];
        let mut struct_fns = vec![];
        for field in props_fields.iter() {
            if let Some(props) = field.props.clone().map(|val| val.unwrap_or_default()) {
                let ty = &field.ty;
                let field = &field.ident;
                if props.required.is_some() {
                    if props.default.is_some() {
                        panic!("`default` attribute cannot be used with `required` attribute")
                    }
                    match ty.get_option_ty() {
                        Some(ty) => {
                            struct_fields.push(quote! { pub #field: Option<#ty>, });
                            new_args.push(quote! { #field: impl Into<#ty>, });
                            new_fill.push(quote! { #field: Some(#field.into()), });
                            struct_fns.push(quote! {
                                pub fn #field(mut self, val: impl Into<#ty>) -> Self {
                                    self.#field = Some(val.into());
                                    self
                                }
                            })
                        }
                        None => {
                            struct_fields.push(quote! { pub #field: #ty, });
                            new_args.push(quote! { #field: impl Into<#ty>, });
                            new_fill.push(quote! { #field: #field.into(), });
                            struct_fns.push(quote! {
                                pub fn #field(mut self, val: impl Into<#ty>) -> Self {
                                    self.#field = val.into();
                                    self
                                }
                            })
                        }
                    }
                } else {
                    let ty = ty.get_option_ty().unwrap_or(ty);
                    if let Some(expr) = props.default {
                        let expr = parse_str::<syn::Expr>(&expr)
                            .expect("Expect expr in as value for `default` attribute");
                        struct_fields.push(quote! { pub #field: #ty, });
                        new_fill.push(quote! { #field: (#expr).into(), });
                        struct_fns.push(quote! {
                            pub fn #field(mut self, val: impl Into<#ty>) -> Self {
                                self.#field = val.into();
                                self
                            }
                        })
                    } else {
                        struct_fields.push(quote! { pub #field: Option<#ty>, });
                        new_fill.push(quote! { #field: None, });
                        struct_fns.push(quote! {
                            pub fn #field(mut self, val: impl Into<#ty>) -> Self {
                                self.#field = Some(val.into());
                                self
                            }
                        })
                    }
                }
            }
        }

        // let mut needed_gen: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> =
        //     Default::default();
        let mut needed_gen = self
            .generics
            .params
            .clone()
            .into_iter()
            .map(|param| (param, false))
            .collect::<Vec<_>>();

        for lifetime in props_fields
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
        for param in props_fields
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

        let gen_params = if !needed_gen.is_empty() {
            quote! { < #needed_gen > }
        } else {
            quote! {}
        };

        quote! {
            pub struct Props #gen_params {
                #( #struct_fields )*
            }

            impl #gen_params Props #gen_params {
                pub fn new(#( #new_args )*) -> Self {
                    Self {
                        #( #new_fill )*
                    }
                }

                #( #struct_fns )*
            }
        }
    }
}

impl FromMeta for StringSet {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut vec = vec![];

        for meta in items.iter() {
            match meta {
                syn::NestedMeta::Lit(syn::Lit::Str(lit_str)) => {
                    let string = lit_str.value();
                    if vec.iter().any(|item| item == &string) {
                        return Err(darling::Error::duplicate_field(&string));
                    } else {
                        vec.push(string);
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::Path(syn::Path { segments, .. })) => {
                    if segments.len() == 1 {
                        if let Some(path_seg) = segments.last() {
                            if !path_seg.arguments.is_empty() {
                                return Err(darling::Error::unsupported_format("expect an ident"));
                            }
                            vec.push(path_seg.ident.to_string());
                        } else {
                            return Err(darling::Error::unsupported_format("expect an ident"));
                        }
                    } else {
                        return Err(darling::Error::unsupported_format("expected signle ident"));
                    }
                }
                _ => {
                    return Err(darling::Error::unsupported_format(
                        "Expected format like this \"val\"",
                    ))
                }
            }
        }

        Ok(StringSet(vec))
    }
}

pub trait TypeExt {
    fn get_option_ty(&self) -> Option<&syn::Type>;
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
                            let first_arg = args.into_iter().nth(0);
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

    fn name(&self) -> &syn::Ident {
        if let syn::Type::Path(syn::TypePath { path, .. }) = &self {
            if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
                return ident;
            }
        }
        panic!("Expected type to be a `Path` with a segment.",);
    }
}
