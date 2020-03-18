use crate::prelude::*;
use att::Attributes;

pub trait ElExt<Msg: 'static> {
    fn add_events(&mut self, val: &Events<Msg>) -> &mut Self;
    fn set_events(&mut self, val: &Events<Msg>) -> &mut Self;
    fn and_events(&mut self, conf: impl FnOnce(&mut Events<Msg>) -> &mut Events<Msg>) -> &mut Self;

    // NOTE: method name overlab with `El::add_style()` method
    // fn add_style(&mut self, style: Style) -> &mut Self;
    fn set_style(&mut self, val: Style) -> &mut Self;
    fn and_style(&mut self, conf: impl FnOnce(&mut Style) -> &mut Style) -> &mut Self;

    fn add_attributes(&mut self, val: Attributes) -> &mut Self;
    fn set_attributes(&mut self, val: Attributes) -> &mut Self;
    fn and_attributes(
        &mut self,
        conf: impl FnOnce(&mut Attributes) -> &mut Attributes,
    ) -> &mut Self;

    fn add_children(&mut self, children: impl IntoIterator<Item = Node<Msg>>) -> &mut Self;
    fn el_ref<E: Clone>(&mut self, reference: &ElRef<E>) -> &mut Self;

    fn config(&mut self, conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>) -> &mut Self;
    fn config_if(&mut self, _: bool, _: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>) -> &mut Self;
    fn config_if_else(
        &mut self,
        _: bool,
        _: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>,
        _: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>,
    ) -> &mut Self;
}

impl<Msg: 'static> ElExt<Msg> for El<Msg> {
    fn add_events(&mut self, val: &Events<Msg>) -> &mut Self {
        for event in val.clone().events.into_iter() {
            self.add_event_handler(event);
        }
        self
    }

    fn set_events(&mut self, val: &Events<Msg>) -> &mut Self {
        use seed::virtual_dom::event_handler_manager::EventHandlerManager;
        self.event_handler_manager = EventHandlerManager::with_event_handlers(val.events.clone());
        self
    }

    fn and_events(&mut self, conf: impl FnOnce(&mut Events<Msg>) -> &mut Events<Msg>) -> &mut Self {
        let mut events = Events::default();
        conf(&mut events);
        self.set_events(&events)
    }

    fn set_style(&mut self, val: Style) -> &mut Self {
        if let Some(style) = val.to_seed_style() {
            self.style = style;
        }
        self
    }

    fn and_style(&mut self, conf: impl FnOnce(&mut Style) -> &mut Style) -> &mut Self {
        let mut style = Style::default();
        conf(&mut style);
        self.set_style(style)
    }

    fn add_attributes(&mut self, val: Attributes) -> &mut Self {
        val.update_el(self);
        self
    }

    fn set_attributes(&mut self, val: Attributes) -> &mut Self {
        self.attrs = seed::virtual_dom::attrs::Attrs::empty();
        self.add_attributes(val);
        self
    }

    fn and_attributes(
        &mut self,
        conf: impl FnOnce(&mut Attributes) -> &mut Attributes,
    ) -> &mut Self {
        let mut attributes = Attributes::default();
        conf(&mut attributes);
        self.add_attributes(attributes);
        self
    }

    fn add_children(&mut self, children: impl IntoIterator<Item = Node<Msg>>) -> &mut Self {
        for child in children.into_iter() {
            self.add_child(child);
        }
        self
    }

    fn el_ref<E: Clone>(&mut self, reference: &ElRef<E>) -> &mut Self {
        self.refs.push(reference.clone().shared_node_ws);
        self
    }

    fn config(&mut self, conf: impl FnOnce(&mut Self) -> &mut Self) -> &mut Self {
        conf(self)
    }

    fn config_if(
        &mut self,
        condition: bool,
        conf: impl FnOnce(&mut Self) -> &mut Self,
    ) -> &mut Self {
        if condition {
            conf(self);
        }
        self
    }

    fn config_if_else(
        &mut self,
        condition: bool,
        true_conf: impl FnOnce(&mut Self) -> &mut Self,
        false_conf: impl FnOnce(&mut Self) -> &mut Self,
    ) -> &mut Self {
        if condition {
            true_conf(self)
        } else {
            false_conf(self)
        }
    }
}

impl<Msg: 'static> ElExt<Msg> for Node<Msg> {
    fn add_events(&mut self, val: &Events<Msg>) -> &mut Self {
        self.and_element(|el| el.add_events(val))
    }

    fn set_events(&mut self, val: &Events<Msg>) -> &mut Self {
        self.and_element(|el| el.set_events(val))
    }

    fn and_events(&mut self, conf: impl FnOnce(&mut Events<Msg>) -> &mut Events<Msg>) -> &mut Self {
        self.and_element(|el| el.and_events(conf))
    }

    fn set_style(&mut self, val: Style) -> &mut Self {
        self.and_element(|el| el.set_style(val))
    }

    fn and_style(&mut self, conf: impl FnOnce(&mut Style) -> &mut Style) -> &mut Self {
        self.and_element(|el| el.and_style(conf))
    }

    fn add_attributes(&mut self, val: Attributes) -> &mut Self {
        self.and_element(|el| el.add_attributes(val))
    }

    fn set_attributes(&mut self, val: Attributes) -> &mut Self {
        self.and_element(|el| el.set_attributes(val))
    }

    fn and_attributes(
        &mut self,
        conf: impl FnOnce(&mut Attributes) -> &mut Attributes,
    ) -> &mut Self {
        self.and_element(|el| el.and_attributes(conf))
    }

    fn add_children(&mut self, children: impl IntoIterator<Item = Node<Msg>>) -> &mut Self {
        self.and_element(|el| el.add_children(children))
    }

    fn el_ref<E: Clone>(&mut self, reference: &ElRef<E>) -> &mut Self {
        self.and_element(|el| el.el_ref(reference))
    }

    fn config(&mut self, conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>) -> &mut Self {
        self.and_element(|el| el.config(conf))
    }

    fn config_if(
        &mut self,
        cond: bool,
        conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>,
    ) -> &mut Self {
        self.and_element(|el| el.config_if(cond, conf))
    }

    fn config_if_else(
        &mut self,
        condition: bool,
        true_conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>,
        false_conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>,
    ) -> &mut Self {
        self.and_element(|el| el.config_if_else(condition, true_conf, false_conf))
    }
}

pub trait NodeExt<Msg: 'static> {
    fn and_element(&mut self, conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>) -> &mut Self;
}

impl<Msg: 'static> NodeExt<Msg> for Node<Msg> {
    fn and_element(&mut self, conf: impl FnOnce(&mut El<Msg>) -> &mut El<Msg>) -> &mut Self {
        if let Node::Element(el) = self {
            conf(el);
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
