pub use super::box_align::{JustifyContent, AlignItems, AlignContent, AlignSelf};
use crate::css::{St, ToStyle, Style, unit::*, self};

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Basis {
    #[from]
    Content(css::Content),
    #[from]
    Auto(css::Auto),
    #[from]
    Inherit(css::Inherit),
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
    Row(css::Row),
    #[from]
    RowReverse(css::RowReverse),
    #[from]
    Column(css::Column),
    #[from]
    ColumnReverse(css::ColumnReverse),
}

impl ToStyle for Direction {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexDirection, self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Wrap {
    #[from]
    Wrap(css::Wrap),
    #[from]
    NoWrap(css::NoWrap),
    #[from]
    WrapReverse(css::WrapReverse),
}

impl ToStyle for Wrap {
    fn to_style(&self) -> Style {
        Style::new().add(St::FlexWrap, self)
    }
}
