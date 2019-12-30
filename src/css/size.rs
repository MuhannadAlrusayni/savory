use crate::css::{St, self, unit::*, ToStyle, Style};
use derive_rich::Rich;

#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Size {
    #[rich(read, write(take))]
    width: Option<Length>,
    #[rich(read, write(take))]
    min_width: Option<Length>,
    #[rich(read, write(take))]
    max_width: Option<Length>,
    #[rich(read, write(take))]
    height: Option<Length>,
    #[rich(read, write(take))]
    min_height: Option<Length>,
    #[rich(read, write(take))]
    max_height: Option<Length>,
}

impl ToStyle for Size {
    fn to_style(&self) -> Style {
        Style::new()
            .try_add(St::Width, self.width)
            .try_add(St::MinWidth, self.min_width)
            .try_add(St::MaxWidth, self.max_width)
            .try_add(St::Height, self.height)
            .try_add(St::MinHeight, self.min_height)
            .try_add(St::MaxHeight, self.max_height)
    }
}

impl Size {
    pub fn full(self) -> Self {
        self.width(1.0)
            .height(1.0)
    }

    pub fn half(self) -> Self {
        self.width(0.5)
            .height(0.5)
    }

    pub fn min_content(self) -> Self {
        self.width(css::MinContent)
            .height(css::MinContent)
    }

    pub fn max_content(self) -> Self {
        self.width(css::MaxContent)
            .height(css::MaxContent)
    }

    pub fn auto(self) -> Self {
        self.width(css::Auto)
            .height(css::Auto)
    }

    pub fn resize(self, width: impl Into<Length>, height: impl Into<Length>) -> Self {
        self.width(width)
            .height(height)
    }
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Auto(css::Auto),
    #[from]
    MinContent(css::MinContent),
    #[from]
    MaxContent(css::MaxContent),
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
