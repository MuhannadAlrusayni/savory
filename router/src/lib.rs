#[macro_use]
extern crate darling;
extern crate syn;

use convert_case::{Case, Casing};
use darling::{
    ast::Data,
    util::{Flag, Ignored},
    Error, FromDeriveInput,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Ident, Type};

#[proc_macro_derive(Router, attributes(route))]
pub fn derive_router(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // darling::FromDeriveInput
    // match Router::from_derive_input(&syn::parse_macro_input!(input)) {
    //     Ok(router) => match router.expand() {
    //         Okk(token_stream) => token_stream.into(),
    //         Err(err) => err.
    //     }
    // }
    match Router::from_derive_input(&syn::parse_macro_input!(input)) {
        Ok(router) => quote! { #router }.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(route), supports(struct_named))]
struct Router {
    ident: Ident,
    // generics: Generics<GenericParam>,
    data: Data<Ignored, RouterField>,
    // attrs: Vec<Attribute>,
    #[darling(default)]
    root: Flag,
}

#[derive(FromField)]
#[darling(attributes(route))]
struct RouterField {
    ident: Option<Ident>,
    // vis: Visibility,
    ty: Type,

    // attrs: Vec<Attribute>,
    #[darling(default)]
    to: Option<String>,
    #[darling(default)]
    persist: Flag,
    #[darling(default)]
    param: Flag,
    #[darling(default)]
    subroute: Flag,
}

uses_type_params!(RouterField, ty);
uses_lifetimes!(RouterField, ty);

impl ToTokens for Router {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.expand() {
            Ok(router_dervie) => tokens.extend(router_dervie),
            Err(err) => tokens.extend(err.write_errors()),
        };
    }
}

impl Router {
    fn expand(&self) -> Result<TokenStream, darling::Error> {
        let router_ident = &self.ident;
        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("`Router` doesn't work with enum")
            .fields;

        let route_fields = fields
            .iter()
            // `to = ..` form used
            .filter(|f| f.to.is_some())
            .collect::<Vec<_>>();

        // return if there are no routes
        if route_fields.is_empty() {
            return Err(Error::custom(
                "Router doesn't contain any route, use `#[route(to = \"..\")]`",
            )
            .with_span(&self.ident.span()));
        }

        // return if no error page used
        let error_page_ty = fields
            .iter()
            .find(|f| f.ident == Some(format_ident!("error")))
            .map(|f| get_inner_ty_from_option(f))
            .ok_or(
                Error::custom("Router must have error page (e.g `error: Option<ErrorPage>`)")
                    .with_span(&self.ident.span()),
            )??;

        // return if internal field doesn't exist or use wrong type
        if !fields.iter().any(|f| {
            let ty: Type = parse_quote! { Internal };
            f.ident.as_ref().unwrap() == "internal" && f.ty == ty
        }) {
            return Err(
                Error::custom("Router must have internal field with type Internal")
                    .with_span(&self.ident.span()),
            );
        }

        // return if url field doesn't exist or use wrong type
        if !fields.iter().any(|f| {
            let ty: Type = parse_quote! { Url };
            f.ident.as_ref().unwrap() == "url" && f.ty == ty
        }) {
            return Err(Error::custom("Router must have url field with type Url")
                .with_span(&self.ident.span()));
        }

        let routes_ident = route_fields
            .iter()
            .map(|f| f.ident.clone().expect("named field"))
            .collect::<Vec<_>>();

        let routes_ty = route_fields
            .iter()
            .map(|f| get_inner_ty_from_option(f))
            .collect::<Result<Vec<_>, _>>()?;

        let routes_ident_pascal = route_fields
            .iter()
            .map(|f| {
                format_ident!(
                    "{}",
                    f.ident.as_ref().unwrap().to_string().to_case(Case::Pascal)
                )
            })
            .collect::<Vec<_>>();

        // let routes_ident_snake = routes_ident_pascal
        //     .iter()
        //     .map(|ident| format_ident!("{}", ident.to_string().to_case(Case::Snake)))
        //     .collect::<Vec<_>>();

        let slugs = route_fields
            .iter()
            .filter(|f| f.persist.is_some())
            .filter(|f| {
                f.to.as_ref()
                    .expect("route must have `to = \"..\"` attribute")
                    .split("/")
                    .into_iter()
                    .any(|s| s.starts_with("{") && s.ends_with("}"))
            })
            .map(|f| {
                format_ident!(
                    "{}_slugs",
                    f.ident
                        .as_ref()
                        .expect("expect named field")
                        .to_string()
                        .to_case(Case::Snake)
                )
            });

        let param_fields = fields
            .iter()
            .filter(|f| f.param.is_some())
            .collect::<Vec<_>>();

        let params_ident = param_fields
            .clone()
            .into_iter()
            .map(|f| f.ident.clone().unwrap())
            .collect::<Vec<_>>();

        let index_type = quote! {
            pub enum Index {
                #( #routes_ident_pascal, )*
                Error,
            }
        };

        let slugs_clone = slugs.clone();
        let internal = quote! {
            pub struct Internal {
                env: Env,
                index: Index,
                #(
                    #slugs_clone: Vec<String>,
                )*
            }
        };

        let message_type = quote! {
            pub enum Msg {
                Switch(Config),
                #( #routes_ident_pascal(<#routes_ty as Element>::Message), )*
                Error(<#error_page_ty as Element>::Message),
            }
        };

        let switch_method = quote! {
            fn switch(&mut self, orders: &mut impl Orders<<Self as Element>::Message>) {
                match self.checked_switch(orders) {
                    Ok(_) => {},
                    Err(msg) => {
                        type Config = <#error_page_ty as Element>::Config;
                        let config = Config {
                            error: msg,
                            #(
                                #params_ident: self.#params_ident.clone(),
                            )*
                        };
                        self.error = Some(config.init(&mut orders.proxy(Msg::Error), &self.internal.env));
                        self.internal.index = Index::Error;
                    }
                }
            }
        };

        let match_arm = route_fields
            .iter()
            // build match arms
            .map(|f| {
                let route_ident = f.ident.as_ref().expect("expect named field");
                let route_ident_pascal = format_ident!("{}", route_ident.to_string().to_case(Case::Pascal));
                let route_ty = get_inner_ty_from_option(&f)?;
                let slugs_field = format_ident!("{}_slugs", route_ident);
                let pattern =
                    f.to.as_ref()
                        .expect("route must have `to = \"..\"` attribute")
                        .split("/")
                        .into_iter()
                        .filter(|slug| !slug.is_empty())
                        .map(|slug| match slug.chars().collect::<Vec<_>>().as_slice() {
                            ['{', rest @ .., '}'] => {
                                let param = rest.iter().collect::<String>();
                                if param.is_empty() {
                                    panic!("{}", "path param must have name (e.g. `{id}`) in value of `to` attribute")
                                }
                                let param = format_ident!("{}", param);
                                quote! { #param }
                            }
                            rest => {
                                if rest.is_empty() {
                                    panic!("empty path part");
                                }
                                let static_path = syn::LitStr::new(
                                    &rest.to_vec().into_iter().collect::<String>(),
                                    Span::call_site(),
                                );
                                quote! { #static_path }
                            }
                        })
                        .collect::<Vec<_>>();
                let params_in_path =
                    f.to.as_ref()
                        .expect("route must have `to = \"..\"` attribute")
                        .split("/")
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .filter_map(|slug| match slug.chars().collect::<Vec<_>>().as_slice() {
                            ['{', rest @ .., '}'] => {
                                let param = format_ident!("{}", rest.iter().collect::<String>());
                                Some(quote! { #param })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                let pattern = if f.subroute.is_some() {
                    quote! { [ #( #pattern, )* url @ .. ] }
                } else {
                    quote! { [ #( #pattern, )* ] }
                };
                let subroute_param = if f.subroute.is_some() {
                    quote! { url: Url::new().set_path(url), }
                } else {
                    quote! {}
                };
                let else_subroute_use_switch_msg = if f.subroute.is_some() {
                    quote! {
                        else {
                            type Config = <#route_ty as Element>::Config;
                            type Message = <#route_ty as Element>::Message;
                            let config = Config {
                                url: Url::new().set_path(url),
                                #( #params_ident: self.#params_ident.clone(), )*
                                #( #params_in_path: #params_in_path.parse().map_err(|_| "failed to parse param-in-path to switch to route-name route".to_string())?, )*
                            };
                            orders.send(Msg::#route_ident_pascal(Message::Switch(config)));
                        }
                    }
                } else {
                    quote! {}
                };

                let block = match (params_in_path.is_empty(), f.persist.is_some()) {
                    (true, true) => quote! {
                        if self.#route_ident.is_none() {
                            type Config = <#route_ty as Element>::Config;
                            let config = Config {
                                #( #params_ident: self.#params_ident.clone(), )*
                                #subroute_param
                            };
                            self.#route_ident = Some(config.init(&mut orders.proxy(Msg::#route_ident_pascal), &self.internal.env));
                        } #else_subroute_use_switch_msg
                        self.internal.index = Index::#route_ident_pascal;
                    },
                    (true, false) => quote! {
                        type Config = <#route_ty as Element>::Config;
                        let config = Config {
                            #( #params_ident: self.#params_ident.clone(), )*
                            #subroute_param
                        };
                        self.#route_ident = Some(config.init(&mut orders.proxy(Msg::#route_ident_pascal), &self.internal.env));
                        self.internal.index = Index::#route_ident_pascal;
                    },
                    (false, true) => quote! {
                        let new_slugs = vec![ #( #params_in_path.to_string(), )* ];
                        if self.internal.#slugs_field != new_slugs {
                            type Config = <#route_ty as Element>::Config;
                            let config = Config {
                                #( #params_ident: self.#params_ident.clone(), )*
                                #( #params_in_path: #params_in_path.parse().map_err(|_| "failed to parse param-in-path to switch to route-name route".to_string())?, )*
                                #subroute_param
                            };
                            self.#route_ident = Some(config.init(&mut orders.proxy(Msg::#route_ident_pascal), &self.internal.env));
                            self.internal.#slugs_field = new_slugs;
                        } #else_subroute_use_switch_msg
                        self.internal.index = Index::#route_ident_pascal;
                    },
                    (false, false) => quote! {
                        type Config = <#route_ty as Element>::Config;
                        let config = Config {
                            #( #params_ident: self.#params_ident.clone(), )*
                            #( #params_in_path: #params_in_path.parse().map_err(|_| "failed to parse param-in-path to switch to route-name route".to_string())?, )*
                            #subroute_param
                        };
                        self.#route_ident = Some(config.init(&mut orders.proxy(Msg::#route_ident_pascal), &self.internal.env));
                        self.internal.index = Index::#route_ident_pascal;
                    },
                };

                Ok(quote! {
                    #pattern => { #block }
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let checked_switch_method = quote! {
            fn checked_switch(&mut self, orders: &mut impl Orders<<Self as Element>::Message>) -> Result<(), String> {
                let path = self.url.path().into_iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
                match path.as_slice() {
                    #(
                        #match_arm
                    )*
                    _ => return Err("Not Found".to_string()),
                };
                Ok(())
            }
        };

        let def_index_val = routes_ident_pascal
            .iter()
            .nth(0)
            .map(|i| quote! { Index::#i })
            .expect("router must have one route at less");
        let subscribe_url_changes_if_root = if self.root.is_some() {
            quote! {
                {
                    #( let #params_ident = config.#params_ident.clone(); )*
                    orders.subscribe(move |url_changed: subs::UrlChanged| Msg::Switch(Config {
                        url: url_changed.0,
                        #( #params_ident, )*
                    }));
                }
            }
        } else {
            quote! {}
        };
        let element_impl = quote! {
            impl Element for #router_ident {
                type Message = Msg;
                type Config = Config;

                fn init(config: Self::Config, orders: &mut impl Orders<Self::Message>, env: &Env) -> Self {
                    #subscribe_url_changes_if_root

                    let mut router = #router_ident {
                        url: config.url,
                        #(
                            #params_ident: config.#params_ident,
                        )*
                        #(
                            #routes_ident: None,
                        )*
                        error: None,
                        internal: Internal {
                            env: env.branch(),
                            index: #def_index_val,
                            #(
                                #slugs: vec![],
                            )*
                        }
                    };

                    router.switch(orders);
                    router
                }

                fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<Self::Message>) {
                    match msg {
                        Msg::Switch(config) => {
                            self.url = config.url;
                            #(
                                self.#params_ident = config.#params_ident;
                            )*
                            self.switch(orders);
                        }
                        #(
                            Msg::#routes_ident_pascal(msg) => if let Some(ref mut #routes_ident) = self.#routes_ident {
                                #routes_ident.update(msg, &mut orders.proxy(Msg::#routes_ident_pascal));
                            }
                        )*
                        Msg::Error(msg) => if let Some(ref mut error) = self.error {
                            error.update(msg, &mut orders.proxy(Msg::Error));
                        }
                    };
                }
            }
        };

        let view_impl = quote! {
            impl View<Node<Msg>> for #router_ident {
                fn view(&self) -> Node<Msg> {
                    match self.internal.index {
                        #(
                            Index::#routes_ident_pascal => if let Some(ref #routes_ident) = self.#routes_ident {
                                return #routes_ident.view().map_msg(Msg::#routes_ident_pascal);
                            }
                        )*
                        Index::Error => if let Some(ref error) = self.error {
                            return error.view().map_msg(Msg::Error);
                        }
                    };
                    // if none of the above match then we got invalid router state (fatle error!!)
                    // we use plain text error message
                    savory::html::h1().push("Error - Invalid route state!")
                }
            }
        };

        Ok(quote! {
            #index_type
            #internal
            #message_type

            impl #router_ident {
                #switch_method
                #checked_switch_method
            }

            #element_impl
            #view_impl
        })
    }
}

// fn ty_ident_from_field(f: &RouterField) -> Result<Ident, Error> {
//     let path_ty = path_type_from_field(f)?;
//     match path_ty.path.segments.iter().last() {
//         Some(seg) => Ok(seg.ident.clone()),
//         None => Err(Error::custom(format!(
//             "Was not able to get ident from {:?}",
//             path_ty
//         ))),
//     }
// }

// fn path_type_from_field(f: &RouterField) -> Result<syn::TypePath, Error> {
//     match f.ty {
//         Type::Path(ref ty) => Ok(ty.clone()),
//         _ => Err(Error::custom(format!("{:?} is not type path", f.ty))),
//     }
// }

fn get_inner_ty_from_option(f: &RouterField) -> Result<Type, Error> {
    fn error(ty: &Type) -> Error {
        Error::custom(format!("{:?} is not an Option<_> type", ty))
    }

    let typepath = match f.ty {
        Type::Path(ref ty) => ty.clone(),
        _ => return Err(error(&f.ty)),
    };

    let type_params = match typepath.path.segments.iter().last() {
        Some(seg) if seg.ident == "Option" => seg.arguments.clone(),
        _ => return Err(error(&f.ty)),
    };

    let type_params = match type_params {
        syn::PathArguments::AngleBracketed(ref arg) => {
            arg.args.iter().last().ok_or(error(&f.ty))?
        }
        _ => return Err(error(&f.ty)),
    };

    match type_params {
        syn::GenericArgument::Type(ty) => Ok(ty.clone()),
        _ => return Err(error(&f.ty)),
    }
}
