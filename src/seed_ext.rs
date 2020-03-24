use crate::prelude::*;
use att::{Attribute, Attributes};
use seed::virtual_dom::attrs::Attrs as SeedAttrs;
use seed::virtual_dom::style::Style as SeedStyle;

#[derive(From)]
pub enum AddArgument<'a, Msg: 'static> {
    #[from]
    RefEvents(&'a Events<Msg>),
    #[from]
    Style(Style),
    #[from]
    Attribute(Attribute),
    #[from]
    Attributes(Attributes),
    #[from]
    Node(Node<Msg>),
    #[from]
    Nodes(Vec<Node<Msg>>),
}

#[derive(From)]
pub enum SetArgument<'a, Msg: 'static> {
    #[from]
    RefEvents(&'a Events<Msg>),
    #[from]
    Style(Style),
    #[from]
    Attributes(Attributes),
    #[from]
    Node(Node<Msg>),
    #[from]
    Nodes(Vec<Node<Msg>>),
}

pub trait ElExt<Msg: 'static> {
    fn add<'a>(self, val: impl Into<AddArgument<'a, Msg>>) -> Self;
    fn set<'a>(self, val: impl Into<SetArgument<'a, Msg>>) -> Self;

    fn add_events(self, val: &Events<Msg>) -> Self;
    fn set_events(self, val: &Events<Msg>) -> Self;
    fn and_events(self, conf: impl FnOnce(Events<Msg>) -> Events<Msg>) -> Self;

    // NOTE: method name overlab with `El::add_style()` method
    // fn add_style(self, style: Style) -> Self;
    fn set_style(self, val: Style) -> Self;
    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self;

    fn add_attribute(self, val: Attribute) -> Self;

    fn add_attributes(self, val: Attributes) -> Self;
    fn set_attributes(self, val: Attributes) -> Self;
    fn and_attributes(self, conf: impl FnOnce(Attributes) -> Attributes) -> Self;

    fn add_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self;
    fn set_children(self, children: impl IntoIterator<Item = Node<Msg>>) -> Self;
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

impl<Msg: 'static> ElExt<Msg> for El<Msg> {
    fn add<'a>(mut self, val: impl Into<AddArgument<'a, Msg>>) -> Self {
        match val.into() {
            AddArgument::RefEvents(val) => self.add_events(val),
            AddArgument::Style(val) => {
                if let Some(style) = val.to_seed_style() {
                    for (key, val) in style.vals.into_iter() {
                        self.add_style(key, val);
                    }
                }
                self
            }
            AddArgument::Attribute(val) => self.add_attribute(val),
            AddArgument::Attributes(val) => self.add_attributes(val),
            AddArgument::Node(val) => self.add_children(vec![val]),
            AddArgument::Nodes(val) => self.add_children(val),
        }
    }

    fn set<'a>(self, val: impl Into<SetArgument<'a, Msg>>) -> Self {
        match val.into() {
            SetArgument::RefEvents(val) => self.set_events(val),
            SetArgument::Style(val) => self.set_style(val),
            SetArgument::Attributes(val) => self.set_attributes(val),
            SetArgument::Node(val) => self.set_children(vec![val]),
            SetArgument::Nodes(val) => self.set_children(val),
        }
    }

    fn add_events(mut self, val: &Events<Msg>) -> Self {
        for event in val.clone().events.into_iter() {
            self.add_event_handler(event);
        }
        self
    }

    fn set_events(mut self, val: &Events<Msg>) -> Self {
        use seed::virtual_dom::event_handler_manager::EventHandlerManager;
        self.event_handler_manager = EventHandlerManager::with_event_handlers(val.events.clone());
        self
    }

    fn and_events(self, conf: impl FnOnce(Events<Msg>) -> Events<Msg>) -> Self {
        self.set_events(&conf(Events::default()))
    }

    fn set_style(mut self, val: Style) -> Self {
        if let Some(style) = val.to_seed_style() {
            self.style = style;
        } else {
            self.style = SeedStyle::empty();
        }
        self
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.set_style(conf(Style::default()))
    }

    fn add_attribute(mut self, val: Attribute) -> Self {
        val.update_el(&mut self);
        self
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

    fn set_children(mut self, children: impl IntoIterator<Item = Node<Msg>>) -> Self {
        self.children = children.into_iter().collect();
        self
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

impl<Msg: 'static> ElExt<Msg> for Node<Msg> {
    fn add<'a>(self, val: impl Into<AddArgument<'a, Msg>>) -> Self {
        self.and_element(|el| el.add(val))
    }

    fn set<'a>(self, val: impl Into<SetArgument<'a, Msg>>) -> Self {
        self.and_element(|el| el.set(val))
    }

    fn add_events(self, val: &Events<Msg>) -> Self {
        self.and_element(|el| el.add_events(val))
    }

    fn set_events(self, val: &Events<Msg>) -> Self {
        self.and_element(|el| el.set_events(val))
    }

    fn and_events(self, conf: impl FnOnce(Events<Msg>) -> Events<Msg>) -> Self {
        self.and_element(|el| el.and_events(conf))
    }

    fn set_style(self, val: Style) -> Self {
        self.and_element(|el| el.set_style(val))
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.and_element(|el| el.and_style(conf))
    }

    fn add_attribute(self, val: Attribute) -> Self {
        self.and_element(|el| el.add_attribute(val))
    }

    fn set_children(self, val: impl IntoIterator<Item = Node<Msg>>) -> Self {
        self.and_element(|el| el.set_children(val))
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

pub trait NodeExt<Msg: 'static> {
    fn and_element(self, conf: impl FnOnce(El<Msg>) -> El<Msg>) -> Self;
}

impl<Msg: 'static> NodeExt<Msg> for Node<Msg> {
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
