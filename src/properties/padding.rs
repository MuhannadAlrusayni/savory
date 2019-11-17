use crate::{macros::*, properties::unit::*};
use seed::{dom_types::Style, prelude::*};

#[derive(Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Padding {
    top: Option<Length>,
    right: Option<Length>,
    bottom: Option<Length>,
    left: Option<Length>,
}

impl Padding {
    pub fn all(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.right(value).top(value).left(value).bottom(value)
    }

    builder_functions! {
        right(Length),
        top(Length),
        left(Length),
        bottom(Length),
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
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

impl From<&Padding> for Style {
    fn from(padding: &Padding) -> Self {
        style![
            St::PaddingTop => padding.top,
            St::PaddingRight => padding.right,
            St::PaddingBottom => padding.bottom,
            St::PaddingLeft => padding.left,
        ]
    }
}
