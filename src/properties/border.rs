use crate::{macros::*, properties::unit::*};
use seed::{dom_types::Style as SeedStyle, prelude::*};

// TODO: add shadow
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Border {
    left: Option<Side>,
    top: Option<Side>,
    right: Option<Side>,
    bottom: Option<Side>,
    collapse: Option<Collapse>,
    top_left: Option<Radius>,
    top_right: Option<Radius>,
    bottom_left: Option<Radius>,
    bottom_right: Option<Radius>,
    spacing: Option<Spacing>,
}

impl Border {
    builder_functions! {
        top_left(Radius),
        top_right(Radius),
        bottom_left(Radius),
        bottom_right(Radius),
        spacing(Spacing),
    }

    builder_enum_functions! {
        collapse {
            separate() => Collapse::Separate,
            collapse() => Collapse::Collapse,
        },
    }

    option_composition_functions! {
        left: Side,
        right: Side,
        top: Side,
        bottom: Side,
    }
}

pub fn display_helper(value: &Option<impl ToString>) -> String {
    value
        .as_ref()
        .map_or("".into(), |v| format!("{} ", v.to_string()))
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Side {
    #[display(
        fmt = "{}{}{}",
        "display_helper(_0)",
        "display_helper(_1)",
        "display_helper(_2).trim()"
    )]
    #[from]
    Props(Option<Style>, Option<Width>, Option<Color>),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
}

impl Default for Side {
    fn default() -> Self {
        Self::Initial
    }
}

macro style_shortcut_functions( $( $fn:ident() => $value:ident $(,)? )* ) {
    $(
        pub fn $fn(self) -> Self {
            self.style(Style::$value)
        }
    )*
}

impl Side {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial(self) -> Self {
        Self::Initial
    }

    pub fn inherit(self) -> Self {
        Self::Inherit
    }

    fn style(self, style: impl Into<Style>) -> Self {
        match self {
            Self::Props(_, width, color) => Self::Props(Some(style.into()), width, color),
            _ => Self::Props(Some(style.into()), None, None),
        }
    }

    style_shortcut_functions! {
        none() => None,
        hidden() => Hidden,
        dotted() => Dotted,
        dashed() => Dashed,
        solid() => Solid,
        double() => Double,
        groove() => Groove,
        ridge() => Ridge,
        inset() => Inset,
        outset() => Outset,
    }

    pub fn width(self, width: impl Into<Width>) -> Self {
        match self {
            Self::Props(style, _, color) => Self::Props(style, Some(width.into()), color),
            _ => Self::Props(None, Some(width.into()), None),
        }
    }

    pub fn color(self, color: impl Into<Color>) -> Self {
        match self {
            Self::Props(style, width, _) => Self::Props(style, width, Some(color.into())),
            _ => Self::Props(None, None, Some(color.into())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Style {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
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
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thick")]
    Thick,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub struct Color;

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Collapse {
    #[display(fmt = "separate")]
    Separate,
    #[display(fmt = "collapse")]
    Collapse,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
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
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Spacing {
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
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
}

impl From<&Border> for SeedStyle {
    fn from(border: &Border) -> Self {
        style![
            St::BorderCollapse => border.collapse,
            St::BorderSpacing => border.spacing,
            St::BorderLeft => border.left,
            St::BorderTop => border.top,
            St::BorderRight => border.right,
            St::BorderBottom => border.bottom,
            St::BorderTopLeftRadius => border.top_left,
            St::BorderTopRightRadius => border.top_right,
            St::BorderBottomLeftRadius => border.bottom_left,
            St::BorderBottomRightRadius => border.bottom_right,
        ]
    }
}
