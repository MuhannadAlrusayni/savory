pub mod common;
pub mod border;
pub mod background;
pub mod color;
// pub mod event;
pub mod margin;
pub mod padding;
pub mod size;
pub mod unit;
pub mod gap;
pub mod box_align;
pub mod flexbox;

pub use self::{
    common::*,
    border::Border,
    background::Background,
    color::Color,
    margin::Margin,
    padding::Padding,
    size::Size,
    gap::Gap,
};

use std::collections::HashMap;
pub use seed::{prelude::{St, UpdateEl}, virtual_dom::node::el::El};

#[derive(Default)]
pub struct Style(HashMap<St, String>);

impl Style {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn add(mut self, key: impl Into<St>, value: impl ToString) -> Self {
        self.0.insert(key.into(), value.to_string());
        self
    }

    pub fn try_add(self, key: impl Into<St>, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            self.add(key, value)
        } else {
            self
        }
    }

    pub fn merge(mut self, other: &impl ToStyle) -> Self {
        self.0.extend(other.to_style().0.into_iter());
        self
    }

    pub fn try_merge(self, other: Option<&impl ToStyle>) -> Self {
        if let Some(other) = other {
            self.merge(other)
        } else {
            self
        }
    }

    pub fn to_css(&self) -> Option<String> {
        self.0.iter()
            .fold(Option::None, |mut css, (key, value)| {
                *css.get_or_insert(String::default()) += &format!("{}: {};", key.as_str(), value);
                css
            })
    }

    pub fn to_seed_style(&self) -> Option<seed::virtual_dom::Style> {
        self.0.iter()
              .fold(Option::None, |mut style, (key, value)| {
                  style.get_or_insert(seed::virtual_dom::Style::empty())
                      .add(key.clone(), value);
                  style
              })
    }
}

impl<Msg> UpdateEl<El<Msg>> for Style {
    fn update(self, el: &mut El<Msg>) {
        if let Some(style) = self.to_seed_style() {
            el.style.merge(style);
        }
    }
}

pub trait ToStyle {
    fn to_style(&self) -> Style;
}
