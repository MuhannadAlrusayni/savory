pub use super::box_align::{AlignContent, AlignItems, AlignSelf, JustifyContent};
use crate::{unit::*, values as val, St, StyleValues, UpdateStyleValues};

#[derive(Clone, Debug, PartialEq, Display, From)]
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

impl UpdateStyleValues for Basis {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::FlexBasis, self)
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

impl UpdateStyleValues for Direction {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::FlexDirection, self)
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

impl UpdateStyleValues for Wrap {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::FlexWrap, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub struct Order(i32);

impl UpdateStyleValues for Order {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::Order, self.0)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub struct Grow(f32);

impl From<u16> for Grow {
    fn from(source: u16) -> Self {
        Grow(source.into())
    }
}

impl UpdateStyleValues for Grow {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::FlexGrow, self.0)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub struct Shrink(f32);

impl UpdateStyleValues for Shrink {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::FlexShrink, self.0)
    }
}
