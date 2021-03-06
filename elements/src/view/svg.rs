//! Svg view
//!
//! Svg is used to display SVG from a given SVG nodes
//!
//! # Usage
//! TODO
use crate::id::Id;
use derive_rich::Rich;
use savory::prelude::*;

/// Svg view type
#[derive(Clone, Rich)]
pub struct Svg<Msg> {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write)]
    pub draw: Vec<Node<Msg>>,
}

// TODO: add from_html function
impl<Msg> Svg<Msg> {
    pub fn new(draw: impl IntoIterator<Item = Node<Msg>>) -> Self {
        Self {
            id: None,
            draw: draw.into_iter().collect(),
        }
    }
}

impl<Msg> View<Node<Msg>> for Svg<Msg> {
    fn view(&self) -> Node<Msg> {
        html::svg()
            .try_id(self.id.clone())
            .class("svg")
            .push(self.draw.clone())
    }
}
