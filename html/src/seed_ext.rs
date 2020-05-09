// use crate::prelude::*;
use crate::{
    attribute::{Attribute, Attributes},
    css::Style,
    prelude::*,
};
use seed::{dom_entity_names::At, prelude::AtValue, virtual_dom::style::Style as SeedStyle};
use std::borrow::Cow;

pub trait AddForEl<T> {
    fn add(self, val: T) -> Self;
}

pub trait TryAddForEl<T> {
    fn try_add(self, val: T) -> Self;
}

pub trait SetForEl<T> {
    fn set(self, val: T) -> Self;
}

pub trait TrySetForEl<T> {
    fn try_set(self, val: T) -> Self;
}

// impl AddForEl
impl<Msg> AddForEl<Style> for El<Msg> {
    fn add(mut self, val: Style) -> Self {
        val.update_el(&mut self);
        self
    }
}

impl<Msg> AddForEl<Style> for Node<Msg> {
    fn add(self, val: Style) -> Self {
        self.and_element(|el| el.add(val))
    }
}

// NOTE: Adding attribute need special handling for each attribute, thus we
// cannot support this functionality yet

impl<Msg> AddForEl<Node<Msg>> for El<Msg> {
    fn add(self, val: Node<Msg>) -> Self {
        self.add_children(vec![val])
    }
}

impl<Msg> AddForEl<Node<Msg>> for Node<Msg> {
    fn add(self, val: Node<Msg>) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<Vec<Node<Msg>>> for El<Msg> {
    fn add(self, val: Vec<Node<Msg>>) -> Self {
        self.add_children(val)
    }
}

impl<Msg> AddForEl<Vec<Node<Msg>>> for Node<Msg> {
    fn add(self, val: Vec<Node<Msg>>) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<&'static str> for El<Msg> {
    fn add(self, val: &'static str) -> Self {
        self.add(html::text(val))
    }
}

impl<Msg> AddForEl<&'static str> for Node<Msg> {
    fn add(self, val: &'static str) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<String> for El<Msg> {
    fn add(self, val: String) -> Self {
        self.add(html::text(val))
    }
}

impl<Msg> AddForEl<String> for Node<Msg> {
    fn add(self, val: String) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<Cow<'static, str>> for El<Msg> {
    fn add(self, val: Cow<'static, str>) -> Self {
        self.add(html::text(val))
    }
}

impl<Msg> AddForEl<Cow<'static, str>> for Node<Msg> {
    fn add(self, val: Cow<'static, str>) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<att::Class> for El<Msg> {
    fn add(mut self, val: att::Class) -> Self {
        self.add_class(val);
        self
    }
}

impl<Msg> AddForEl<att::Class> for Node<Msg> {
    fn add(self, val: att::Class) -> Self {
        self.and_element(|el| el.add(val))
    }
}

// impl TryAddForEl
impl<Msg> TryAddForEl<Option<Style>> for El<Msg> {
    fn try_add(self, val: Option<Style>) -> Self {
        self.try_add_style(val)
    }
}

impl<Msg> TryAddForEl<Option<Style>> for Node<Msg> {
    fn try_add(self, val: Option<Style>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

// NOTE: Adding attribute need special handling for each attribute, thus we
// cannot support this functionality yet

impl<Msg> TryAddForEl<Option<Node<Msg>>> for El<Msg> {
    fn try_add(self, val: Option<Node<Msg>>) -> Self {
        self.try_add_children(val.map(|node| vec![node]))
    }
}

impl<Msg> TryAddForEl<Option<Node<Msg>>> for Node<Msg> {
    fn try_add(self, val: Option<Node<Msg>>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<Msg> TryAddForEl<Option<Vec<Node<Msg>>>> for El<Msg> {
    fn try_add(self, val: Option<Vec<Node<Msg>>>) -> Self {
        self.try_add_children(val)
    }
}

impl<Msg> TryAddForEl<Option<Vec<Node<Msg>>>> for Node<Msg> {
    fn try_add(self, val: Option<Vec<Node<Msg>>>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<Msg> TryAddForEl<Option<&'static str>> for El<Msg> {
    fn try_add(self, val: Option<&'static str>) -> Self {
        self.try_add(val.map(html::text))
    }
}

impl<Msg> TryAddForEl<Option<&'static str>> for Node<Msg> {
    fn try_add(self, val: Option<&'static str>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<Msg> TryAddForEl<Option<String>> for El<Msg> {
    fn try_add(self, val: Option<String>) -> Self {
        self.try_add(val.map(html::text))
    }
}

impl<Msg> TryAddForEl<Option<String>> for Node<Msg> {
    fn try_add(self, val: Option<String>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<Msg> TryAddForEl<Option<Cow<'static, str>>> for El<Msg> {
    fn try_add(self, val: Option<Cow<'static, str>>) -> Self {
        self.try_add(val.map(html::text))
    }
}

impl<Msg> TryAddForEl<Option<Cow<'static, str>>> for Node<Msg> {
    fn try_add(self, val: Option<Cow<'static, str>>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<Msg> TryAddForEl<Option<att::Class>> for El<Msg> {
    fn try_add(self, val: Option<att::Class>) -> Self {
        if let Some(val) = val {
            self.add(val)
        } else {
            self
        }
    }
}

impl<Msg> TryAddForEl<Option<att::Class>> for Node<Msg> {
    fn try_add(self, val: Option<att::Class>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

// impl SetForEl
impl<Msg> SetForEl<Style> for El<Msg> {
    fn set(self, val: Style) -> Self {
        self.set_style(val)
    }
}

impl<Msg> SetForEl<Style> for Node<Msg> {
    fn set(self, val: Style) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<T: Into<Attribute>, Msg> SetForEl<T> for El<Msg> {
    fn set(self, val: T) -> Self {
        self.set_attribute(val.into())
    }
}

impl<T: Into<Attribute>, Msg> SetForEl<T> for Node<Msg> {
    fn set(self, val: T) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<Attributes> for El<Msg> {
    fn set(self, val: Attributes) -> Self {
        self.set_attributes(val)
    }
}

impl<Msg> SetForEl<Attributes> for Node<Msg> {
    fn set(self, val: Attributes) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<Node<Msg>> for El<Msg> {
    fn set(self, val: Node<Msg>) -> Self {
        self.set_children(vec![val])
    }
}

impl<Msg> SetForEl<Node<Msg>> for Node<Msg> {
    fn set(self, val: Node<Msg>) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<Vec<Node<Msg>>> for El<Msg> {
    fn set(self, val: Vec<Node<Msg>>) -> Self {
        self.set_children(val)
    }
}

impl<Msg> SetForEl<Vec<Node<Msg>>> for Node<Msg> {
    fn set(self, val: Vec<Node<Msg>>) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<&'static str> for El<Msg> {
    fn set(self, val: &'static str) -> Self {
        self.set(html::text(val))
    }
}

impl<Msg> SetForEl<&'static str> for Node<Msg> {
    fn set(self, val: &'static str) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<String> for El<Msg> {
    fn set(self, val: String) -> Self {
        self.set(html::text(val))
    }
}

impl<Msg> SetForEl<String> for Node<Msg> {
    fn set(self, val: String) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<Cow<'static, str>> for El<Msg> {
    fn set(self, val: Cow<'static, str>) -> Self {
        self.set(html::text(val))
    }
}

impl<Msg> SetForEl<Cow<'static, str>> for Node<Msg> {
    fn set(self, val: Cow<'static, str>) -> Self {
        self.and_element(|el| el.set(val))
    }
}

// impl TrySetForEl
impl<Msg> TrySetForEl<Option<Style>> for El<Msg> {
    fn try_set(self, val: Option<Style>) -> Self {
        self.try_set_style(val)
    }
}

impl<Msg> TrySetForEl<Option<Style>> for Node<Msg> {
    fn try_set(self, val: Option<Style>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<T: Into<Attribute>, Msg> TrySetForEl<Option<T>> for El<Msg> {
    fn try_set(self, val: Option<T>) -> Self {
        self.try_set_attribute(val.map(|val| val.into()))
    }
}

impl<T: Into<Attribute>, Msg> TrySetForEl<Option<T>> for Node<Msg> {
    fn try_set(self, val: Option<T>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<Msg> TrySetForEl<Option<Node<Msg>>> for El<Msg> {
    fn try_set(self, val: Option<Node<Msg>>) -> Self {
        self.try_set_children(val.map(|val| vec![val]))
    }
}

impl<Msg> TrySetForEl<Option<Node<Msg>>> for Node<Msg> {
    fn try_set(self, val: Option<Node<Msg>>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<Msg> TrySetForEl<Option<Vec<Node<Msg>>>> for El<Msg> {
    fn try_set(self, val: Option<Vec<Node<Msg>>>) -> Self {
        self.try_set_children(val)
    }
}

impl<Msg> TrySetForEl<Option<Vec<Node<Msg>>>> for Node<Msg> {
    fn try_set(self, val: Option<Vec<Node<Msg>>>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<Msg> TrySetForEl<Option<&'static str>> for El<Msg> {
    fn try_set(self, val: Option<&'static str>) -> Self {
        self.try_set(val.map(html::text))
    }
}

impl<Msg> TrySetForEl<Option<&'static str>> for Node<Msg> {
    fn try_set(self, val: Option<&'static str>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<Msg> TrySetForEl<Option<String>> for El<Msg> {
    fn try_set(self, val: Option<String>) -> Self {
        self.try_set(val.map(html::text))
    }
}

impl<Msg> TrySetForEl<Option<String>> for Node<Msg> {
    fn try_set(self, val: Option<String>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<Msg> TrySetForEl<Option<Cow<'static, str>>> for El<Msg> {
    fn try_set(self, val: Option<Cow<'static, str>>) -> Self {
        self.try_set(val.map(html::text))
    }
}

impl<Msg> TrySetForEl<Option<Cow<'static, str>>> for Node<Msg> {
    fn try_set(self, val: Option<Cow<'static, str>>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

pub trait ElExt<Msg> {
    // NOTE: method name overlab with `El::add_style()`
    // fn add_style(self, style: Style) -> Self;
    fn try_add_style(self, val: Option<Style>) -> Self;
    fn set_style(self, val: Style) -> Self;
    fn try_set_style(self, val: Option<Style>) -> Self;
    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self;

    // NOTE: Adding attribute need special handling for each attribute, thus we
    // cannot support this functionality yet
    // fn add_attribute(self, val: Attribute) -> Self;
    // fn try_add_attribute(self, val: Option<Attribute>) -> Self;
    fn set_attribute(self, val: Attribute) -> Self;
    fn try_set_attribute(self, val: Option<Attribute>) -> Self;

    // fn add_attributes(self, val: Attributes) -> Self;
    fn set_attributes(self, val: Attributes) -> Self;
    fn and_attributes(self, conf: impl FnOnce(Attributes) -> Attributes) -> Self;

    fn add_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self;
    fn try_add_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self;
    fn set_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self;
    fn try_set_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self;

    // NOTE: method name overlab with `El::add_class()`
    // fn add_class(self, class: att::Class) -> Self;
    fn get_class(&self) -> Option<att::Class>;
    fn class(self, class: impl Into<att::Class>) -> Self;

    fn get_id(&self) -> Option<att::Id>;
    fn try_id(self, id: Option<impl Into<att::Id>>) -> Self;
    fn id(self, id: impl Into<att::Id>) -> Self;

    fn el_ref<E: Clone>(self, reference: &ElRef<E>) -> Self;

    fn config(self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
    fn config_if(self, _: bool, _: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
    fn config_if_else(
        self,
        _: bool,
        _: impl FnOnce(El<Msg>) -> El<Msg>,
        _: impl FnOnce(El<Msg>) -> El<Msg>,
    ) -> Self;

    // lookup methods
    fn for_class(
        self,
        class: impl Into<att::Class>,
        f: impl Fn(Node<Msg>) -> Node<Msg> + Copy,
    ) -> Self;
    fn for_id(self, id: impl Into<att::Id>, f: impl Fn(Node<Msg>) -> Node<Msg> + Copy) -> Self;
    // fn for_selector(s: impl Into<Selector>, f: impl FnOnce(Node) -> Node<Msg>);
}

impl<Msg> ElExt<Msg> for El<Msg> {
    fn try_add_style(self, val: Option<Style>) -> Self {
        if let Some(val) = val {
            self.add(val)
        } else {
            self
        }
    }

    fn set_style(mut self, val: Style) -> Self {
        if let Some(style) = val.into_seed_style() {
            self.style = style;
        } else {
            self.style = SeedStyle::empty();
        }
        self
    }

    fn try_set_style(self, val: Option<Style>) -> Self {
        if let Some(val) = val {
            self.set_style(val)
        } else {
            self
        }
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.add(conf(Style::default()))
    }

    fn set_attribute(mut self, val: Attribute) -> Self {
        val.update_el(&mut self);
        self
    }

    fn try_set_attribute(self, val: Option<Attribute>) -> Self {
        if let Some(attr) = val {
            self.set_attribute(attr)
        } else {
            self
        }
    }

    fn set_attributes(mut self, val: Attributes) -> Self {
        val.update_el(&mut self);
        self
    }

    fn and_attributes(self, conf: impl FnOnce(Attributes) -> Attributes) -> Self {
        self.set_attributes(conf(Default::default()))
    }

    fn add_children(mut self, children: impl IntoIterator<Item = Node<Msg>>) -> Self {
        for child in children.into_iter() {
            self.add_child(child);
        }
        self
    }

    fn try_add_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self {
        if let Some(children) = children {
            self.add_children(children)
        } else {
            self
        }
    }

    fn set_children(mut self, children: impl IntoIterator<Item = Node<Msg>>) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    fn try_set_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self {
        if let Some(children) = children {
            self.set_children(children)
        } else {
            self
        }
    }

    fn get_class(&self) -> Option<att::Class> {
        self.attrs.vals.get(&At::Class).and_then(|v| match v {
            AtValue::Some(val) => Some(att::Class::from(val.clone())),
            _ => None,
        })
    }

    fn class(self, class: impl Into<att::Class>) -> Self {
        self.set(class.into())
    }

    fn get_id(&self) -> Option<att::Id> {
        self.attrs.vals.get(&At::Id).and_then(|v| match v {
            AtValue::Some(val) => Some(att::Id::from(val.clone())),
            _ => None,
        })
    }

    fn try_id(self, id: Option<impl Into<att::Id>>) -> Self {
        if let Some(id) = id {
            self.id(id)
        } else {
            self
        }
    }

    fn id(self, id: impl Into<att::Id>) -> Self {
        self.set(id.into())
    }

    fn el_ref<E: Clone>(mut self, reference: &ElRef<E>) -> Self {
        self.refs.push(reference.clone().shared_node_ws);
        self
    }

    fn config(self, conf: impl FnOnce(Self) -> Self) -> Self {
        conf(self)
    }

    fn config_if(mut self, condition: bool, conf: impl FnOnce(Self) -> Self) -> Self {
        if condition {
            self = conf(self);
        }
        self
    }

    fn config_if_else(
        self,
        condition: bool,
        true_conf: impl FnOnce(Self) -> Self,
        false_conf: impl FnOnce(Self) -> Self,
    ) -> Self {
        if condition {
            true_conf(self)
        } else {
            false_conf(self)
        }
    }

    fn for_class(
        mut self,
        class: impl Into<att::Class>,
        f: impl Fn(Node<Msg>) -> Node<Msg> + Copy,
    ) -> Self {
        let class = class.into();
        self.children = self
            .children
            .into_iter()
            .map(move |mut child| {
                if child.get_class().map_or(false, |c| c.contains(&class)) {
                    child = f(child);
                }
                child.for_class(class.clone(), f)
            })
            .collect();
        self
    }

    fn for_id(mut self, id: impl Into<att::Id>, f: impl Fn(Node<Msg>) -> Node<Msg> + Copy) -> Self {
        let id = id.into();
        self.children = self
            .children
            .into_iter()
            .map(move |mut child| {
                if child.get_id().as_ref() == Some(&id) {
                    child = f(child);
                }
                child.for_id(id.clone(), f)
            })
            .collect();
        self
    }
}

impl<Msg> ElExt<Msg> for Node<Msg> {
    fn try_add_style(self, val: Option<Style>) -> Self {
        self.and_element(|el| el.try_add_style(val))
    }

    fn set_style(self, val: Style) -> Self {
        self.and_element(|el| el.set_style(val))
    }

    fn try_set_style(self, val: Option<Style>) -> Self {
        self.and_element(|el| el.try_set_style(val))
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.and_element(|el| el.and_style(conf))
    }

    fn set_attribute(self, val: Attribute) -> Self {
        self.and_element(|el| el.set_attribute(val))
    }

    fn try_set_attribute(self, val: Option<Attribute>) -> Self {
        self.and_element(|el| el.try_set_attribute(val))
    }

    fn set_attributes(self, val: Attributes) -> Self {
        self.and_element(|el| el.set_attributes(val))
    }

    fn and_attributes(self, conf: impl FnOnce(Attributes) -> Attributes) -> Self {
        self.and_element(|el| el.and_attributes(conf))
    }

    fn add_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self {
        self.and_element(|el| el.add_children(children))
    }

    fn try_add_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self {
        self.and_element(|el| el.try_add_children(children))
    }

    fn set_children(self, val: impl IntoIterator<Item = Node<Msg>>) -> Self {
        self.and_element(|el| el.set_children(val))
    }

    fn try_set_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self {
        self.and_element(|el| el.try_set_children(children))
    }

    fn el_ref<E: Clone>(self, reference: &ElRef<E>) -> Self {
        self.and_element(|el| el.el_ref(reference))
    }

    fn get_class(&self) -> Option<att::Class> {
        if let Node::Element(ref el) = self {
            el.get_class()
        } else {
            None
        }
    }

    fn class(self, class: impl Into<att::Class>) -> Self {
        self.and_element(|el| el.class(class))
    }

    fn get_id(&self) -> Option<att::Id> {
        if let Node::Element(ref el) = self {
            el.get_id()
        } else {
            None
        }
    }

    fn try_id(self, id: Option<impl Into<att::Id>>) -> Self {
        self.and_element(|el| el.try_id(id))
    }

    fn id(self, id: impl Into<att::Id>) -> Self {
        self.and_element(|el| el.id(id))
    }

    fn config(self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self {
        self.and_element(|el| el.config(conf))
    }

    fn config_if(self, cond: bool, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self {
        self.and_element(|el| el.config_if(cond, conf))
    }

    fn config_if_else(
        self,
        condition: bool,
        true_conf: impl FnOnce(El<Msg>) -> El<Msg>,
        false_conf: impl FnOnce(El<Msg>) -> El<Msg>,
    ) -> Self {
        self.and_element(|el| el.config_if_else(condition, true_conf, false_conf))
    }

    fn for_class(
        self,
        class: impl Into<att::Class>,
        f: impl Fn(Node<Msg>) -> Node<Msg> + Copy,
    ) -> Self {
        self.and_element(|el| el.for_class(class, f))
    }

    fn for_id(self, id: impl Into<att::Id>, f: impl Fn(Node<Msg>) -> Node<Msg> + Copy) -> Self {
        self.and_element(|el| el.for_id(id, f))
    }
}

pub trait NodeExt<Msg> {
    fn and_element(self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
}

impl<Msg> NodeExt<Msg> for Node<Msg> {
    fn and_element(mut self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self {
        if let Node::Element(el) = self {
            self = Node::Element(conf(el));
        }
        self
    }
}

pub trait ElRefExt<T> {
    fn get_then(&self, conf: impl FnOnce(T)) -> &Self;
}

impl<T: Clone + wasm_bindgen::JsCast> ElRefExt<T> for ElRef<T> {
    fn get_then(&self, conf: impl FnOnce(T)) -> &Self {
        if let Some(el) = self.get() {
            conf(el);
        }
        self
    }
}
