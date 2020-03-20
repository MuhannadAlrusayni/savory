pub use super::box_align::{AlignContent, AlignItems, AlignSelf, JustifyContent};
use crate::css::{unit::*, values as val, St, StyleMap, ToStyleMap};

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Basis {
    #[from]
    Content(val::Content),
    #[from]
    Auto(val::Auto),
    #[from]
    Inherit(val::Inherit),
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

impl ToStyleMap for Basis {
    fn style_map(&self) -> StyleMap {
        StyleMap::default().add(St::FlexBasis, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum Direction {
    #[from]
    Row(val::Row),
    #[from]
    RowReverse(val::RowReverse),
    #[from]
    Column(val::Column),
    #[from]
    ColumnReverse(val::ColumnReverse),
}

impl ToStyleMap for Direction {
    fn style_map(&self) -> StyleMap {
        StyleMap::default().add(St::FlexDirection, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum Wrap {
    #[from]
    Wrap(val::Wrap),
    #[from]
    Nowrap(val::Nowrap),
    #[from]
    WrapReverse(val::WrapReverse),
}

impl ToStyleMap for Wrap {
    fn style_map(&self) -> StyleMap {
        StyleMap::default().add(St::FlexWrap, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub struct Order(i32);

impl ToStyleMap for Order {
    fn style_map(&self) -> StyleMap {
        StyleMap::default().add(St::Order, self.0)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub struct Grow(f32);

impl ToStyleMap for Grow {
    fn style_map(&self) -> StyleMap {
        StyleMap::default().add(St::FlexGrow, self.0)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub struct Shrink(f32);

impl ToStyleMap for Shrink {
    fn style_map(&self) -> StyleMap {
        StyleMap::default().add(St::FlexShrink, self.0)
    }
}
