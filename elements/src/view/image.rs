//! Image view
//!
//! Image is a simple view that display an image from a given source
//!
//! # Usage
//! TODO
use crate::id::Id;
use derive_rich::Rich;
use savory::prelude::*;
use std::borrow::Cow;

/// Image view
#[derive(Clone, Rich)]
pub struct Image {
    #[rich(write)]
    pub id: Option<Id>,
    pub src: Cow<'static, str>,
}

impl Image {
    pub fn new(src: Cow<'static, str>) -> Self {
        Self { id: None, src }
    }
}

impl<Msg> View<Node<Msg>> for Image {
    fn view(&self) -> Node<Msg> {
        html::image()
            .try_id(self.id.clone())
            .class("image")
            .src(self.src.clone())
    }
}
