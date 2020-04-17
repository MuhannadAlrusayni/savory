// use crate::prelude::*;
use crate::{
    attribute::{Attribute, Attributes},
    css::Style,
    events::Events,
    prelude::*,
};
use seed::virtual_dom::{attrs::Attrs as SeedAttrs, style::Style as SeedStyle};
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
impl<Msg> AddForEl<&Events<Msg>> for El<Msg> {
    fn add(self, val: &Events<Msg>) -> Self {
        self.add_events(val)
    }
}

impl<Msg> AddForEl<&Events<Msg>> for Node<Msg> {
    fn add(self, val: &Events<Msg>) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<&Style> for El<Msg> {
    fn add(mut self, val: &Style) -> Self {
        if let Some(style) = val.to_seed_style() {
            for (key, val) in style.vals.into_iter() {
                self.add_style(key, val);
            }
        }
        self
    }
}

impl<Msg> AddForEl<&Style> for Node<Msg> {
    fn add(self, val: &Style) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<T: Into<Attribute>, Msg> AddForEl<T> for El<Msg> {
    fn add(self, val: T) -> Self {
        self.add_attribute(val.into())
    }
}

impl<T: Into<Attribute>, Msg> AddForEl<T> for Node<Msg> {
    fn add(self, val: T) -> Self {
        self.and_element(|el| el.add(val))
    }
}

impl<Msg> AddForEl<Attributes> for El<Msg> {
    fn add(self, val: Attributes) -> Self {
        self.add_attributes(val)
    }
}

impl<Msg> AddForEl<Attributes> for Node<Msg> {
    fn add(self, val: Attributes) -> Self {
        self.and_element(|el| el.add(val))
    }
}

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

// impl TryAddForEl
impl<Msg> TryAddForEl<Option<&Events<Msg>>> for El<Msg> {
    fn try_add(self, val: Option<&Events<Msg>>) -> Self {
        self.try_add_events(val)
    }
}

impl<Msg> TryAddForEl<Option<&Events<Msg>>> for Node<Msg> {
    fn try_add(self, val: Option<&Events<Msg>>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<Msg> TryAddForEl<Option<&Style>> for El<Msg> {
    fn try_add(self, val: Option<&Style>) -> Self {
        self.try_add_style(val)
    }
}

impl<Msg> TryAddForEl<Option<&Style>> for Node<Msg> {
    fn try_add(self, val: Option<&Style>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

impl<T: Into<Attribute>, Msg> TryAddForEl<Option<T>> for El<Msg> {
    fn try_add(self, val: Option<T>) -> Self {
        self.try_add_attribute(val.map(|val| val.into()))
    }
}

impl<T: Into<Attribute>, Msg> TryAddForEl<Option<T>> for Node<Msg> {
    fn try_add(self, val: Option<T>) -> Self {
        self.and_element(|el| el.try_add(val))
    }
}

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

// impl SetForEl
impl<Msg> SetForEl<&Events<Msg>> for El<Msg> {
    fn set(self, val: &Events<Msg>) -> Self {
        self.set_events(val)
    }
}

impl<Msg> SetForEl<&Events<Msg>> for Node<Msg> {
    fn set(self, val: &Events<Msg>) -> Self {
        self.and_element(|el| el.set(val))
    }
}

impl<Msg> SetForEl<&Style> for El<Msg> {
    fn set(self, val: &Style) -> Self {
        self.set_style(val)
    }
}

impl<Msg> SetForEl<&Style> for Node<Msg> {
    fn set(self, val: &Style) -> Self {
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
impl<Msg> TrySetForEl<Option<&Events<Msg>>> for El<Msg> {
    fn try_set(self, val: Option<&Events<Msg>>) -> Self {
        self.try_set_events(val)
    }
}

impl<Msg> TrySetForEl<Option<&Events<Msg>>> for Node<Msg> {
    fn try_set(self, val: Option<&Events<Msg>>) -> Self {
        self.and_element(|el| el.try_set(val))
    }
}

impl<Msg> TrySetForEl<Option<&Style>> for El<Msg> {
    fn try_set(self, val: Option<&Style>) -> Self {
        self.try_set_style(val)
    }
}

impl<Msg> TrySetForEl<Option<&Style>> for Node<Msg> {
    fn try_set(self, val: Option<&Style>) -> Self {
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
    fn add_events(self, val: &Events<Msg>) -> Self;
    fn try_add_events(self, val: Option<&Events<Msg>>) -> Self;
    fn set_events(self, val: &Events<Msg>) -> Self;
    fn try_set_events(self, val: Option<&Events<Msg>>) -> Self;
    fn and_events(self, conf: impl FnOnce(Events<Msg>) -> Events<Msg>) -> Self;

    // NOTE: method name overlab with `El::add_style()` method
    // fn add_style(self, style: Style) -> Self;
    fn try_add_style(self, val: Option<&Style>) -> Self;
    fn set_style(self, val: &Style) -> Self;
    fn try_set_style(self, val: Option<&Style>) -> Self;
    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self;

    // TODO: Remove this function since attributes can only be setted to Node
    fn add_attribute(self, val: Attribute) -> Self;
    fn try_add_attribute(self, val: Option<Attribute>) -> Self;
    fn set_attribute(self, val: Attribute) -> Self;
    fn try_set_attribute(self, val: Option<Attribute>) -> Self;

    fn add_attributes(self, val: Attributes) -> Self;
    fn set_attributes(self, val: Attributes) -> Self;
    fn and_attributes(self, conf: impl FnOnce(Attributes) -> Attributes) -> Self;

    fn add_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self;
    fn try_add_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self;
    fn set_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self;
    fn try_set_children(self, children: Option<impl IntoIterator<Item = Node<Msg>>>) -> Self;

    fn el_ref<E: Clone>(self, reference: &ElRef<E>) -> Self;

    fn config(self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
    fn config_if(self, _: bool, _: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
    fn config_if_else(
        self,
        _: bool,
        _: impl FnOnce(El<Msg>) -> El<Msg>,
        _: impl FnOnce(El<Msg>) -> El<Msg>,
    ) -> Self;
}

impl<Msg> ElExt<Msg> for El<Msg> {
    fn add_events(mut self, val: &Events<Msg>) -> Self {
        for event in val.clone().events.into_iter() {
            self.add_event_handler(event);
        }
        self
    }

    fn try_add_events(self, val: Option<&Events<Msg>>) -> Self {
        if let Some(events) = val {
            self.add_events(events)
        } else {
            self
        }
    }

    fn set_events(mut self, val: &Events<Msg>) -> Self {
        use seed::virtual_dom::event_handler_manager::EventHandlerManager;
        self.event_handler_manager = EventHandlerManager::with_event_handlers(val.events.clone());
        self
    }

    fn try_set_events(self, val: Option<&Events<Msg>>) -> Self {
        if let Some(events) = val {
            self.set_events(events)
        } else {
            self
        }
    }

    fn and_events(self, conf: impl FnOnce(Events<Msg>) -> Events<Msg>) -> Self {
        self.set_events(&conf(Events::default()))
    }

    fn try_add_style(self, val: Option<&Style>) -> Self {
        if let Some(val) = val {
            self.add(val)
        } else {
            self
        }
    }

    fn set_style(mut self, val: &Style) -> Self {
        if let Some(style) = val.to_seed_style() {
            self.style = style;
        } else {
            self.style = SeedStyle::empty();
        }
        self
    }

    fn try_set_style(self, val: Option<&Style>) -> Self {
        if let Some(val) = val {
            self.set_style(val)
        } else {
            self
        }
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.set_style(&conf(Style::default()))
    }

    fn add_attribute(mut self, val: Attribute) -> Self {
        val.update_el(&mut self);
        self
    }

    fn try_add_attribute(self, val: Option<Attribute>) -> Self {
        if let Some(attr) = val {
            self.add_attribute(attr)
        } else {
            self
        }
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

    fn add_attributes(mut self, val: Attributes) -> Self {
        val.update_el(&mut self);
        self
    }

    fn set_attributes(mut self, val: Attributes) -> Self {
        self.attrs = SeedAttrs::empty();
        self.add_attributes(val)
    }

    fn and_attributes(self, conf: impl FnOnce(Attributes) -> Attributes) -> Self {
        self.add_attributes(conf(Attributes::default()))
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
}

impl<Msg> ElExt<Msg> for Node<Msg> {
    fn add_events(self, val: &Events<Msg>) -> Self {
        self.and_element(|el| el.add_events(val))
    }

    fn try_add_events(self, val: Option<&Events<Msg>>) -> Self {
        self.and_element(|el| el.try_add_events(val))
    }

    fn set_events(self, val: &Events<Msg>) -> Self {
        self.and_element(|el| el.set_events(val))
    }

    fn try_set_events(self, val: Option<&Events<Msg>>) -> Self {
        self.and_element(|el| el.try_set_events(val))
    }

    fn and_events(self, conf: impl FnOnce(Events<Msg>) -> Events<Msg>) -> Self {
        self.and_element(|el| el.and_events(conf))
    }

    fn try_add_style(self, val: Option<&Style>) -> Self {
        self.and_element(|el| el.try_add_style(val))
    }

    fn set_style(self, val: &Style) -> Self {
        self.and_element(|el| el.set_style(val))
    }

    fn try_set_style(self, val: Option<&Style>) -> Self {
        self.and_element(|el| el.try_set_style(val))
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.and_element(|el| el.and_style(conf))
    }

    fn add_attribute(self, val: Attribute) -> Self {
        self.and_element(|el| el.add_attribute(val))
    }

    fn try_add_attribute(self, val: Option<Attribute>) -> Self {
        self.and_element(|el| el.try_add_attribute(val))
    }

    fn set_attribute(self, val: Attribute) -> Self {
        self.and_element(|el| el.set_attribute(val))
    }

    fn try_set_attribute(self, val: Option<Attribute>) -> Self {
        self.and_element(|el| el.try_set_attribute(val))
    }

    fn add_attributes(self, val: Attributes) -> Self {
        self.and_element(|el| el.add_attributes(val))
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
