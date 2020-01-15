pub mod background;
pub mod border;
pub mod color;
pub mod common;
// pub mod event;
pub mod box_align;
pub mod flexbox;
pub mod gap;
pub mod margin;
pub mod padding;
pub mod size;
pub mod transition;
pub mod unit;

pub use self::{
    background::Background, border::Border, color::Color, common::*, gap::Gap, margin::Margin,
    padding::Padding, size::Size, transition::Transition,
};

pub use seed::{
    prelude::{St, UpdateEl},
    virtual_dom::node::el::El,
};
use std::collections::HashMap;

// TODO: add compose function for every css::* style (e.g. border(|border| ..))
#[derive(Default, Debug, Clone)]
pub struct Style(HashMap<St, String>);

macro css_props( $( $fn_ident:ident($prop_ty:ty) $(,)? )* ) {
    impl Style {
        $(
            pub fn $fn_ident<R: Into<$prop_ty>>(mut self, get_prop: impl Fn($prop_ty) -> R) -> Self {
                let value = get_prop(<$prop_ty>::default()).into();
                self.merge(&value)
            }
        )*
    }
}

macro simple_css_props( $( $fn_ident:ident($prop_ty:ty) $(,)? )* ) {
    impl Style {
        $(
            pub fn $fn_ident(mut self, value: impl Into<$prop_ty>) -> Self {
                self.merge(&value.into())
            }
        )*
    }
}

simple_css_props! {
    color(Color), gap(Gap), flex_wrap(flexbox::Wrap), flex_basis(flexbox::Basis),
    flex_direction(flexbox::Direction), justify_content(box_align::JustifyContent),
    align_content(box_align::AlignContent), align_items(box_align::AlignItems),
    justify_self(box_align::JustifySelf), align_self(box_align::AlignSelf),
}

css_props! {
    background(Background), border(Border), margin(Margin), padding(Padding), size(Size), transition(Transition),
}

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
        self.0.iter().fold(Option::None, |mut css, (key, value)| {
            *css.get_or_insert(String::default()) += &format!("{}: {};", key.as_str(), value);
            css
        })
    }

    pub fn to_seed_style(&self) -> Option<seed::virtual_dom::Style> {
        self.0.iter().fold(Option::None, |mut style, (key, value)| {
            style
                .get_or_insert(seed::virtual_dom::Style::empty())
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

impl ToStyle for Style {
    fn to_style(&self) -> Style {
        self.clone()
    }
}
