//! Traits and implementation that makes working with `Node` API declarative and
//! convenient.

use crate::prelude::*;
use paste::paste;
use seed::prelude::{At, AtValue};
use std::borrow::Cow;

pub use seed::prelude::{El, Node};

impl<Msg> PushOwned<Node<Msg>> for El<Msg> {
    fn push(self, val: Node<Msg>) -> Self {
        self.push(vec![val])
    }
}

impl<Msg> PushOwned<Node<Msg>> for Node<Msg> {
    fn push(self, val: Node<Msg>) -> Self {
        self.and_el(|el| el.push(val))
    }
}

impl<Msg> PushOwned<Vec<Node<Msg>>> for El<Msg> {
    fn push(mut self, val: Vec<Node<Msg>>) -> Self {
        self.children.extend(val);
        self
    }
}

impl<Msg> PushOwned<Vec<Node<Msg>>> for Node<Msg> {
    fn push(self, val: Vec<Node<Msg>>) -> Self {
        self.and_el(|el| el.push(val))
    }
}

impl<Msg> PushOwned<&'static str> for El<Msg> {
    fn push(self, val: &'static str) -> Self {
        self.push(html::text(val))
    }
}

impl<Msg> PushOwned<&'static str> for Node<Msg> {
    fn push(self, val: &'static str) -> Self {
        self.and_el(|el| el.push(val))
    }
}

impl<Msg> PushOwned<String> for El<Msg> {
    fn push(self, val: String) -> Self {
        self.push(html::text(val))
    }
}

impl<Msg> PushOwned<String> for Node<Msg> {
    fn push(self, val: String) -> Self {
        self.and_el(|el| el.push(val))
    }
}

impl<Msg> PushOwned<Cow<'static, str>> for El<Msg> {
    fn push(self, val: Cow<'static, str>) -> Self {
        self.push(html::text(val))
    }
}

impl<Msg> PushOwned<Cow<'static, str>> for Node<Msg> {
    fn push(self, val: Cow<'static, str>) -> Self {
        self.and_el(|el| el.push(val))
    }
}

// impl SetOwned
impl<Msg> SetOwned<Node<Msg>> for El<Msg> {
    fn set(self, val: Node<Msg>) -> Self {
        self.set(vec![val])
    }
}

impl<Msg> SetOwned<Node<Msg>> for Node<Msg> {
    fn set(self, val: Node<Msg>) -> Self {
        self.and_el(|el| el.set(val))
    }
}

impl<Msg> SetOwned<Vec<Node<Msg>>> for El<Msg> {
    fn set(mut self, val: Vec<Node<Msg>>) -> Self {
        self.children = val;
        self
    }
}

impl<Msg> SetOwned<Vec<Node<Msg>>> for Node<Msg> {
    fn set(self, val: Vec<Node<Msg>>) -> Self {
        self.and_el(|el| el.set(val))
    }
}

impl<Msg> SetOwned<&'static str> for El<Msg> {
    fn set(self, val: &'static str) -> Self {
        self.set(html::text(val))
    }
}

impl<Msg> SetOwned<&'static str> for Node<Msg> {
    fn set(self, val: &'static str) -> Self {
        self.and_el(|el| el.set(val))
    }
}

impl<Msg> SetOwned<String> for El<Msg> {
    fn set(self, val: String) -> Self {
        self.set(html::text(val))
    }
}

impl<Msg> SetOwned<String> for Node<Msg> {
    fn set(self, val: String) -> Self {
        self.and_el(|el| el.set(val))
    }
}

impl<Msg> SetOwned<Cow<'static, str>> for El<Msg> {
    fn set(self, val: Cow<'static, str>) -> Self {
        self.set(html::text(val))
    }
}

impl<Msg> SetOwned<Cow<'static, str>> for Node<Msg> {
    fn set(self, val: Cow<'static, str>) -> Self {
        self.and_el(|el| el.set(val))
    }
}

// TODO: use std::Pattern once stabilized
// TODO: support css like Selector
// TODO: add docs
pub trait LookupApi<Msg>: Sized {
    fn for_class<'a>(
        self,
        class: impl IntoIterator<Item = &'a str>,
        f: impl Fn(Self) -> Self + Copy,
    ) -> Self;
    fn for_id(self, id: impl ToString, f: impl Fn(Self) -> Self + Copy) -> Self;
    // fn for_selector(s: impl Into<Selector>, f: impl FnOnce(Node) -> Node<Msg>);
}

impl<Msg> LookupApi<Msg> for El<Msg> {
    fn for_class<'a>(
        mut self,
        class: impl IntoIterator<Item = &'a str>,
        f: impl Fn(Self) -> Self + Copy,
    ) -> Self {
        let class = class.into_iter().collect::<Vec<_>>();
        if self
            .get_class()
            .map_or(false, |c| c.split(" ").any(|c| class.contains(&c)))
        {
            self = f(self);
        }
        self.children = self
            .children
            .into_iter()
            .map(|c| {
                c.for_class(class.clone(), |node| {
                    node.and_el(|el| el.for_class(class.clone(), f))
                })
            })
            .collect();
        self
    }

    fn for_id(mut self, id: impl ToString, f: impl Fn(Self) -> Self + Copy) -> Self {
        let id = id.to_string();
        if self.get_id().map_or(false, |val| val == id) {
            self = f(self);
        }
        self.children = self
            .children
            .into_iter()
            .map(|c| {
                c.for_id(id.clone(), |node| {
                    node.and_el(|el| el.for_id(id.clone(), f))
                })
            })
            .collect();
        self
    }
}

impl<Msg> LookupApi<Msg> for Node<Msg> {
    fn for_class<'a>(
        mut self,
        class: impl IntoIterator<Item = &'a str>,
        f: impl Fn(Self) -> Self + Copy,
    ) -> Self {
        let class = class.into_iter().collect::<Vec<_>>();
        if !matches!(self, Self::Element(_)) {
            return self;
        }
        if self
            .get_class()
            .map_or(false, |c| c.split(" ").any(|c| class.contains(&c)))
        {
            self = f(self);
        }
        self.and_el(|mut el| {
            el.children = el
                .children
                .into_iter()
                .map(|c| c.for_class(class.clone(), f))
                .collect();
            el
        })
    }

    fn for_id(mut self, id: impl ToString, f: impl Fn(Self) -> Self + Copy) -> Self {
        let id = id.to_string();
        if !matches!(self, Self::Element(_)) {
            return self;
        }
        if self.get_id().map_or(false, |val| val == id) {
            self = f(self);
        }
        self.and_el(|mut el| {
            el.children = el
                .children
                .into_iter()
                .map(|c| c.for_id(id.clone(), f))
                .collect();
            el
        })
    }
}

macro_rules! attr_fns {
    (@match_like String $( $attr_name:literal $attr_ident:ident )*) => {
        $(
            fn $attr_ident(self, val: impl ToString) -> Self {
                self.set_attr($attr_name, val)
            }

            paste! {
                fn [<try_ $attr_ident>](self, val: Option<impl ToString>) -> Self {
                    match val {
                        Some(val) => self.$attr_ident(val),
                        None => self,
                    }
                }

                fn [<get_ $attr_ident>](&self) -> Option<&str> {
                    self.get_attr($attr_name)
                }
            }
        )*
    };
    (@match_like $ty:ident no_impl $( $attr_name:literal $attr_ident:ident )*) => {
        $(
            fn $attr_ident(self, val: $ty) -> Self;

            paste! {
                fn [<try_ $attr_ident>](self, val: Option<$ty>) -> Self {
                    match val {
                        Some(val) => self.$attr_ident(val),
                        None => self,
                    }
                }

                fn [<get_ $attr_ident>](&self) -> Option<$ty> {
                    match self.get_attr($attr_name) {
                        Some(val) => Some(val.parse::<$ty>().expect(&format!("expect value of type {} for attribute {}", stringify!($ty), $attr_name))),
                        None => None,
                    }
                }
            }
        )*
    };
    (@match_like $ty:ident $( $attr_name:literal $attr_ident:ident )* ) => {
        $(
            fn $attr_ident(self, val: $ty) -> Self {
                self.set_attr($attr_name, val.to_string())
            }

            paste! {
                fn [<try_ $attr_ident>](self, val: Option<$ty>) -> Self {
                    match val {
                        Some(val) => self.$attr_ident(val),
                        None => self,
                    }
                }

                fn [<get_ $attr_ident>](&self) -> Option<$ty> {
                    match self.get_attr($attr_name) {
                        Some(val) => Some(val.parse::<$ty>().expect(&format!("expect value of type {} for attribute {}", stringify!($ty), $attr_name))),
                        None => None,
                    }
                }
            }
        )*
    };
    ( $( $ty:ident $( ( $($flags:ident)+ ) )? {
        $( $attr_name:literal: $attr_ident:ident $(,)? )*
    } $(,)? )* ) => {
        $(
            attr_fns!(@match_like $ty $($( $flags )+)? $( $attr_name $attr_ident )* );
        )*
    };
}

pub trait AttributeApi: Sized {
    /// set an attribute with it's value
    fn set_attr(self, attr: impl ToString, val: impl ToString) -> Self;
    /// get an attribute value if it exist
    fn get_attr(&self, attr: impl ToString) -> Option<&str>;

    // TODO: add docs
    attr_fns!(
        String {
            "class": class,
            "id": id,
            "abbr": abbr,
            "accept": accept,
            "accept-charset": accept_charset,
            "action": action,
            "allow": allow,
            "alt": alt,
            "as": as_,
            "charset": charset,
            "cite": cite,
            "color": color,
            "content": content,
            "data": data,
            "date-time": datetime,
            "dir-name": dir_name,
            "download": download,
            "for": for_,
            "form": form,
            "form-action": form_action,
            "formtarget": form_target,
            "headers": headers,
            "href": href,
            "href-lang": href_lang,
            "integrity": integrity,
            "is": is,
            "itemid": item_id,
            "itemgroup": item_group,
            "itemref": item_ref,
            "itemtype": item_type,
            "label": label,
            "lang": lang,
            "list": list,
            "manifest": manifest,
            "media": media,
            "name": name,
            "nonce": nonce,
            "placeholder": placeholder,
            "poster": poster,
            "slot": slot,
            "src": src,
            "src-doc": src_doc,
            "src-lang": src_lang,
            "src-set": src_set,
            // this need custom implementation that is found in savory-style
            // "style": style,
            "title": title,
            "use-map": use_map,
            "value": value,
            // the following can be enum instead of strings
            // Note: some attributes for SVG nodes only, these might go to separate trait
            "step": step,
            "target": target,
            "translate": translate,
            "type": type_,
            "wrap": wrap,
            "cx": cx,
            "cy": cy,
            "r": r,
            "rx": rx,
            "ry": ry,
            "x": x,
            "y": y,
            "view-box": view_box,
            "max": max,
            "method": method,
            "min": min,
            "pattern": pattern,
            "preload": preload,
            "referrerpolicy": referrer_policy,
            "rel": rel,
            "sandbox": sandbox,
            "scope": scope,
            "shape": shape,
            "kind": kind,
            "draggable": draggable,
            "enc-type": enc_type,
            "entry_key_hint": enter_key_hint,
            "fromenctype": form_enc_type,
            "frommethod": form_method,
            "inputmode": input_mode,
            "decoding": decoding,
            "loading": loading,
            "dir": dir,
            "coords": coords,
            "crossorigin": cross_origin,
            "autocapitalize": auto_capitalize,
            "auto-complete": auto_complete,
        }
        bool (no_impl) {
            "disabled": disabled,
        }
        bool {
            "allowfullscreen": allow_full_screen,
            "allowpaymentrequest": allow_payment_request,
            "auto-focus": auto_focus,
            "auto-play": auto_play,
            "checked": checked,
            "content-editable": content_editable,
            "controls": controls,
            "default": default,
            "defer": defer,
            "formnovalidate": form_no_validate,
            "hidden": hidden,
            "is-map": is_map,
            "itemscope": item_scope,
            "loop": loop_,
            "multiple": multiple,
            "muted": muted,
            "nomodule": no_module,
            "no-validate": no_validate,
            "open": open,
            "playsinline": plays_in_line,
            "read-only": read_only,
            "required": required,
            "reversed": reversed,
            "selected": selected,
            "spell-check": spell_check,
        }
        u32 {
            "cols": cols,
            "col-span": col_span,
            "height": height,
            "rows": rows,
            "row-span": row_span,
            "size": size,
            "span": span,
            "width": width,
        }
        f32 {
            "high": high,
            "low": low,
            "optimum": optimum,
        }
        i32 {
            "max-length": max_length,
            "min-length": min_length,
            "tab-index": tab_index,
            "start": start,
        }
    );
}

impl<Msg> AttributeApi for El<Msg> {
    fn set_attr(mut self, attr: impl ToString, val: impl ToString) -> Self {
        self.attrs
            .vals
            .insert(At::from(attr.to_string()), val.to_string().into());
        self
    }

    fn get_attr(&self, attr: impl ToString) -> Option<&str> {
        match self.attrs.vals.get(&At::from(attr.to_string())) {
            Some(AtValue::Some(val)) => Some(val.as_str()),
            Some(AtValue::Ignored) | Some(AtValue::None) | None => None,
        }
    }

    fn disabled(mut self, val: bool) -> Self {
        match val {
            true => self.set_attr(At::Disabled, val),
            false => {
                self.attrs.vals.remove(&At::Disabled);
                self
            }
        }
    }
}

impl<Msg> AttributeApi for Node<Msg> {
    fn set_attr(self, attr: impl ToString, val: impl ToString) -> Self {
        self.and_el(|el| el.set_attr(attr, val))
    }

    fn get_attr(&self, attr: impl ToString) -> Option<&str> {
        match self {
            Node::Element(el) => el.get_attr(attr),
            _ => None,
        }
    }

    fn disabled(self, val: bool) -> Self {
        self.and_el(|el| el.disabled(val))
    }
}

/// Helper trait used to make working with `El` inside node easier.
pub trait AndEl<Msg> {
    // TODO: add docs
    fn and_el(self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
}

impl<Msg> AndEl<Msg> for Node<Msg> {
    fn and_el(mut self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self {
        if let Node::Element(el) = self {
            self = Node::Element(conf(el));
        }
        self
    }
}

/// Helper triat used to make working with `ElRef` convenient.
pub trait ElRefExt {
    // TODO: add docs
    fn el_ref<E: Clone>(self, reference: &ElRef<E>) -> Self;
}

impl<Msg> ElRefExt for El<Msg> {
    fn el_ref<E: Clone>(mut self, reference: &ElRef<E>) -> Self {
        self.refs.push(reference.clone().shared_node_ws);
        self
    }
}

impl<Msg> ElRefExt for Node<Msg> {
    fn el_ref<E: Clone>(self, reference: &ElRef<E>) -> Self {
        self.and_el(|mut el| {
            el.refs.push(reference.clone().shared_node_ws);
            el
        })
    }
}

impl<Msg> DeclarativeConfig for Node<Msg> {}
impl<Msg> DeclarativeConfig for El<Msg> {}
