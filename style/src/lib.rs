//! Types and traits for working with CSS.
#![forbid(unsafe_code)]

#[macro_use]
extern crate derive_more;

#[macro_use]
pub mod style;
pub mod background;
pub mod border;
pub mod box_align;
pub mod box_shadow;
pub mod calc;
pub mod color;
pub mod cursor;
pub mod display;
pub mod flexbox;
pub mod font;
pub mod gap;
pub mod margin;
pub mod node;
pub mod padding;
pub mod position;
pub mod size;
pub mod text;
pub mod transition;
pub mod unit;
pub mod values;
pub mod visibility;

pub use self::{
    background::Background,
    border::Border,
    box_align::*,
    box_shadow::BoxShadow,
    color::{Color, Opacity},
    cursor::Cursor,
    display::Display,
    flexbox::{
        Basis as FlexBasis, Direction as FlexDirection, Grow as FlexGrow, Order as FlexOrder,
        Shrink as FlexShrink, Wrap as FlexWrap,
    },
    font::Font,
    gap::Gap,
    margin::Margin,
    padding::Padding,
    position::Position,
    size::Size,
    style::{St, Style, StyleValues, UpdateStyleValues},
    text::Text,
    transition::Transition,
    visibility::Visibility,
};

pub mod prelude {
    pub use crate::{node::StyleApi, Style};
}
