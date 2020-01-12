use super::{unit::*, St, Style, ToStyle};
use derive_rich::Rich;

#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Padding {
    #[rich(read, write(take))]
    top: Option<Length>,
    #[rich(read, write(take))]
    right: Option<Length>,
    #[rich(read, write(take))]
    bottom: Option<Length>,
    #[rich(read, write(take))]
    left: Option<Length>,
}

impl From<Length> for Padding {
    fn from(source: Length) -> Self {
        Padding::default().all(source)
    }
}

impl ToStyle for Padding {
    fn to_style(&self) -> Style {
        Style::new()
            .try_add(St::PaddingTop, self.top)
            .try_add(St::PaddingRight, self.right)
            .try_add(St::PaddingBottom, self.bottom)
            .try_add(St::PaddingLeft, self.left)
    }
}

impl Padding {
    pub fn all(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.right(value).top(value).left(value).bottom(value)
    }

    pub fn zero(self) -> Self {
        self.all(px(0.))
    }

    pub fn x(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.left(value).right(value)
    }

    pub fn y(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.top(value).bottom(value)
    }

    pub fn horizontal(self, value: impl Into<Length>) -> Self {
        self.y(value)
    }

    pub fn vertical(self, value: impl Into<Length>) -> Self {
        self.x(value)
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

impl Default for Length {
    fn default() -> Self {
        px(0.0).into()
    }
}
