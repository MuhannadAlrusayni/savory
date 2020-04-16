//! Types and traits for working with CSS.

#[macro_use]
pub mod style;
pub mod background;
pub mod border;
pub mod box_align;
pub mod color;
pub mod cursor;
pub mod display;
pub mod flexbox;
pub mod font;
pub mod gap;
pub mod margin;
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