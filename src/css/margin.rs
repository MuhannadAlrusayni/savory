use crate::css::{self, unit::*, St, Style, ToStyle};
use derive_rich::Rich;

#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Margin {
    #[rich(read, write(take, style = compose))]
    top: Option<Length>,
    #[rich(read, write(take, style = compose))]
    right: Option<Length>,
    #[rich(read, write(take, style = compose))]
    bottom: Option<Length>,
    #[rich(read, write(take, style = compose))]
    left: Option<Length>,
}

impl ToStyle for Margin {
    fn to_style(&self) -> Style {
        Style::new()
            .try_add(St::MarginTop, self.top)
            .try_add(St::MarginRight, self.right)
            .try_add(St::MarginBottom, self.bottom)
            .try_add(St::MarginLeft, self.left)
    }
}

impl Margin {
    pub fn x(self, len: impl Fn(Length) -> Length + Clone) -> Self {
        self.left(|left| len.clone()(left))
            .right(|right| len(right))
    }

    pub fn y(self, len: impl Fn(Length) -> Length + Clone) -> Self {
        self.top(|top| len.clone()(top))
            .bottom(|bottom| len.clone()(bottom))
    }

    pub fn auto(self) -> Self {
        self.all(|m| m.auto())
    }

    pub fn full(self) -> Self {
        self.all(|m| m.full())
    }

    pub fn half(self) -> Self {
        self.all(|m| m.half())
    }

    pub fn all(self, value: impl Fn(Length) -> Length + Copy) -> Self {
        self.right(value).top(value).left(value).bottom(value)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Auto(css::Auto),
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

impl Default for Length {
    fn default() -> Self {
        css::Auto.into()
    }
}

impl Length {
    pub fn auto(self) -> Self {
        css::Auto.into()
    }

    pub fn full(self) -> Self {
        1.0.into()
    }

    pub fn half(self) -> Self {
        0.5.into()
    }
}
