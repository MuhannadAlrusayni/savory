use crate::css::{self, color::Color, unit::*, St, ToStyle};
use derive_rich::Rich;

// TODO: add shadow
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Border {
    #[rich(read, write(take, style = compose))]
    left: Side,
    #[rich(read, write(take, style = compose))]
    top: Side,
    #[rich(read, write(take, style = compose))]
    right: Side,
    #[rich(read, write(take, style = compose))]
    bottom: Side,
    #[rich(read, write(take))]
    top_left: Option<Radius>,
    #[rich(read, write(take))]
    top_right: Option<Radius>,
    #[rich(read, write(take))]
    bottom_left: Option<Radius>,
    #[rich(read, write(take))]
    bottom_right: Option<Radius>,
}

impl ToStyle for Border {
    fn to_style(&self) -> css::Style {
        css::Style::new()
            // left side
            .try_add(St::BorderLeftColor, self.left.color)
            .try_add(St::BorderLeftWidth, self.left.width)
            .try_add(St::BorderLeftStyle, self.left.style)
            // top side
            .try_add(St::BorderTopColor, self.top.color)
            .try_add(St::BorderTopWidth, self.top.width)
            .try_add(St::BorderTopStyle, self.top.style)
            // right side
            .try_add(St::BorderRightColor, self.right.color)
            .try_add(St::BorderRightWidth, self.right.width)
            .try_add(St::BorderRightStyle, self.right.style)
            // bottom side
            .try_add(St::BorderBottomColor, self.bottom.color)
            .try_add(St::BorderBottomWidth, self.bottom.width)
            .try_add(St::BorderBottomStyle, self.bottom.style)
            // radius
            .try_add(St::BorderTopLeftRadius, self.top_left)
            .try_add(St::BorderTopRightRadius, self.top_right)
            .try_add(St::BorderBottomLeftRadius, self.bottom_left)
            .try_add(St::BorderBottomRightRadius, self.bottom_right)
    }
}

macro sides_style_shortcut_functions( $( $fn:ident() $(,)? )* ) {
    $(
        pub fn $fn(self) -> Self {
            self.all_side(|side| side.$fn())
        }
    )*
}

impl Border {
    pub fn all_side(self, value: impl Fn(Side) -> Side + Copy) -> Self {
        self.left(value).top(value).right(value).bottom(value)
    }

    pub fn style(self, style: impl Into<Style>) -> Self {
        let style = style.into();
        self.all_side(|side| side.style(style))
    }

    pub fn width(self, width: impl Into<Width>) -> Self {
        let width = width.into();
        self.all_side(|side| side.width(width))
    }

    pub fn color(self, color: impl Into<Color>) -> Self {
        let color = color.into();
        self.all_side(|side| side.color(color))
    }

    pub fn transparent(self) -> Self {
        self.color(Color::Transparent)
    }

    pub fn radius(self, rad: impl Into<Radius>) -> Self {
        let rad = rad.into();
        self.top_left(rad)
            .top_right(rad)
            .bottom_left(rad)
            .bottom_right(rad)
    }

    sides_style_shortcut_functions! {
        none(), hidden(), dotted(), dashed(), solid(), double(),
        groove(), ridge(), inset(), outset(),
    }
}

#[derive(Rich, Clone, Debug, PartialEq, From, Default)]
pub struct Side {
    #[rich(write(take), read, value_fns(take) = {
        none = css::None,
        hidden = css::Hidden,
        dotted = css::Dotted,
        dashed = css::Dashed,
        solid = css::Solid,
        double = css::Double,
        groove = css::Groove,
        ridge = css::Ridge,
        inset = css::Inset,
        outset = css::Outset,
        initial_style = css::Initial,
        inherit_style = css::Inherit,
    })]
    style: Option<Style>,
    #[rich(read, write(take), value_fns(take) = {
        thick = css::Thick,
        thin = css::Thin,
        medium = css::Medium,
        initial_width = css::Initial,
        inherit_width = css::Inherit,
    })]
    width: Option<Width>,
    #[rich(read, write(take))]
    color: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Style {
    #[from]
    None(css::None),
    #[from]
    Hidden(css::Hidden),
    #[from]
    Dotted(css::Dotted),
    #[from]
    Dashed(css::Dashed),
    #[from]
    Solid(css::Solid),
    #[from]
    Double(css::Double),
    #[from]
    Groove(css::Groove),
    #[from]
    Ridge(css::Ridge),
    #[from]
    Inset(css::Inset),
    #[from]
    Outset(css::Outset),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Width {
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
    #[from]
    Thin(css::Thin),
    #[from]
    Medium(css::Medium),
    #[from]
    Thick(css::Thick),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Radius {
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
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

// impl Default for Radius {
//     fn default() -> Self {
//         Self::Inherit
//     }
// }
