use crate::css::{color::Color, unit::*, values as val, St, StyleMap, ToStyleMap};
use derive_rich::Rich;

// TODO: add shadow
#[derive(Rich, Copy, Clone, Debug, PartialEq, Default)]
pub struct Border {
    #[rich(read, write(style = compose))]
    left: Side,
    #[rich(read, write(style = compose))]
    top: Side,
    #[rich(read, write(style = compose))]
    right: Side,
    #[rich(read, write(style = compose))]
    bottom: Side,
    #[rich(read, write)]
    top_left: Option<Radius>,
    #[rich(read, write)]
    top_right: Option<Radius>,
    #[rich(read, write)]
    bottom_left: Option<Radius>,
    #[rich(read, write)]
    bottom_right: Option<Radius>,
}

impl ToStyleMap for Border {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map
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
            .try_add(St::BorderBottomRightRadius, self.bottom_right);
        map
    }
}

macro_rules! sides_style_shortcut_functions {
    ( $( $fn:ident() $(,)? )* ) => {
        $(
            pub fn $fn(&mut self) -> &mut Self {
                self.all_side(|side| side.$fn())
            }
        )*
    }
}

impl Border {
    pub fn all_side(&mut self, value: impl Fn(&mut Side) -> &mut Side + Copy) -> &mut Self {
        self.left(value).top(value).right(value).bottom(value)
    }

    pub fn style(&mut self, style: impl Into<Style>) -> &mut Self {
        let style = style.into();
        self.all_side(|side| side.style(style))
    }

    pub fn width(&mut self, width: impl Into<Width>) -> &mut Self {
        let width = width.into();
        self.all_side(|side| side.width(width))
    }

    pub fn color(&mut self, color: impl Into<Color>) -> &mut Self {
        let color = color.into();
        self.all_side(|side| side.color(color))
    }

    pub fn transparent(&mut self) -> &mut Self {
        self.color(Color::Transparent)
    }

    pub fn radius(&mut self, rad: impl Into<Radius>) -> &mut Self {
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

#[derive(Rich, Copy, Clone, Debug, PartialEq, From, Default)]
pub struct Side {
    #[rich(write, read, value_fns = {
        none = val::None,
        hidden = val::Hidden,
        dotted = val::Dotted,
        dashed = val::Dashed,
        solid = val::Solid,
        double = val::Double,
        groove = val::Groove,
        ridge = val::Ridge,
        inset = val::Inset,
        outset = val::Outset,
        initial_style = val::Initial,
        inherit_style = val::Inherit,
    })]
    style: Option<Style>,
    #[rich(read, write, value_fns = {
        thick = val::Thick,
        thin = val::Thin,
        medium = val::Medium,
        initial_width = val::Initial,
        inherit_width = val::Inherit,
    })]
    width: Option<Width>,
    #[rich(read, write)]
    color: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Style {
    #[from]
    None(val::None),
    #[from]
    Hidden(val::Hidden),
    #[from]
    Dotted(val::Dotted),
    #[from]
    Dashed(val::Dashed),
    #[from]
    Solid(val::Solid),
    #[from]
    Double(val::Double),
    #[from]
    Groove(val::Groove),
    #[from]
    Ridge(val::Ridge),
    #[from]
    Inset(val::Inset),
    #[from]
    Outset(val::Outset),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
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
    Thin(val::Thin),
    #[from]
    Medium(val::Medium),
    #[from]
    Thick(val::Thick),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
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
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

// impl Default for Radius {
//     fn default() -> Self {
//         Self::Inherit
//     }
// }
