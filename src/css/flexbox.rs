pub use super::box_align::{AlignContent, AlignItems, AlignSelf, JustifyContent};
use crate::css::{self, unit::*, values as val, St, Style, ToStyle};

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Basis {
    #[from]
    Content(val::Content),
    #[from]
    Auto(val::Auto),
    #[from]
    Inherit(val::Inherit),
    #[from]
    Em(Em),
    #[from]
    Ex(Ex),
    #[from]
    Cap(Cap),
    #[from]
    Ch(Ch),
    #[from]
    Ic(Ic),
    #[from]
    Rem(Rem),
    #[from]
    Rlh(Rlh),
    #[from]
    Vm(Vm),
    #[from]
    Vh(Vh),
    #[from]
    Vi(Vi),
    #[from]
    Vb(Vb),
    #[from]
    Vmin(Vmin),
    #[from]
    Vmax(Vmax),
    #[from]
    Cm(Cm),
    #[from]
    Mm(Mm),
    #[from]
    Q(Q),
    #[from]
    In(In),
    #[from]
    Pc(Pc),
    #[from]
    Pt(Pt),
    #[from]
    Px(Px),
    #[from(forward)]
    Percent(Percent),
}

impl ToStyle for Basis {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexBasis, self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
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

impl ToStyle for Direction {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexDirection, self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Wrap {
    #[from]
    Wrap(val::Wrap),
    #[from]
    Nowrap(val::Nowrap),
    #[from]
    WrapReverse(val::WrapReverse),
}

impl ToStyle for Wrap {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexWrap, self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub struct Order(i32);

impl ToStyle for Order {
    fn to_style(&self) -> Style {
        Style::new().add(St::Order, self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Display, From)]
pub struct Grow(f32);

impl ToStyle for Grow {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexGrow, self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Display, From)]
pub struct Shrink(f32);

impl ToStyle for Shrink {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexShrink, self.0)
    }
}
