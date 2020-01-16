use crate::css::{self, unit::*, St, Style, ToStyle};
use derive_rich::Rich;

#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Position {
    #[rich(value_fns(take) = {
        static_pos = css::Static,
        absolute = css::Absolute,
        fixed = css::Fixed,
        relative = css::Relative,
        sticky = css::Sticky,
        initial = css::Initial,
        inherit = css::Inherit,
    })]
    position: Option<PositionType>,
    #[rich(write(take))]
    left: Option<LengthPercent>,
    #[rich(write(take))]
    top: Option<LengthPercent>,
    #[rich(write(take))]
    right: Option<LengthPercent>,
    #[rich(write(take))]
    bottom: Option<LengthPercent>,
    #[rich(write(take))]
    z_index: Option<i32>,
    #[rich(write(take, style = compose))]
    clip: Option<Clip>,
}

impl ToStyle for Position {
    fn to_style(&self) -> Style {
        Style::default()
            .try_add(St::Position, self.position.as_ref())
            .try_add(St::Left, self.left.as_ref())
            .try_add(St::Top, self.top.as_ref())
            .try_add(St::Right, self.right.as_ref())
            .try_add(St::Bottom, self.bottom.as_ref())
            .try_add(St::ZIndex, self.z_index.as_ref())
            .try_add(St::Clip, self.clip.as_ref())
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum LengthPercent {
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
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

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Clip {
    #[from]
    Auto(css::Auto),
    #[display(fmt = "rect({}, {}, {}, {})", top, right, bottom, left)]
    ShapeRect {
        top: Length,
        right: Length,
        bottom: Length,
        left: Length,
    },
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

impl Default for Clip {
    fn default() -> Self {
        css::Initial.into()
    }
}

impl Clip {
    pub fn auto(self) -> Self {
        css::Auto.into()
    }

    pub fn rect(
        top: impl Into<Length>,
        right: impl Into<Length>,
        bottom: impl Into<Length>,
        left: impl Into<Length>,
    ) -> Self {
        Self::ShapeRect {
            top: top.into(),
            right: right.into(),
            bottom: bottom.into(),
            left: left.into(),
        }
    }

    pub fn initial(self) -> Self {
        css::Initial.into()
    }

    pub fn inherit(self) -> Self {
        css::Inherit.into()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
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

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum PositionType {
    #[from]
    Static(css::Static),
    #[from]
    Absolute(css::Absolute),
    #[from]
    Fixed(css::Fixed),
    #[from]
    Relative(css::Relative),
    #[from]
    Sticky(css::Sticky),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}
