use crate::theme::Theme;
use seed::{dom_types::Style, prelude::*};

pub trait View<Msg: 'static + Clone> {
    fn view(&self, theme: &impl Theme) -> Node<Msg>;
}

impl<Msg: 'static + Clone> View<Msg> for Node<Msg> {
    fn view(&self, _: &impl Theme) -> Node<Msg> {
        self.clone()
    }
}

impl<Msg: 'static + Clone> View<Msg> for El<Msg> {
    fn view(&self, _: &impl Theme) -> Node<Msg> {
        Node::Element(self.clone())
    }
}
