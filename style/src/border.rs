use crate::{color::Color, unit::*, values as val, St, StyleValues, UpdateStyleValues};
use derive_rich::Rich;
use savory::prelude::DeclarativeConfig;

/// ```
/// use savory_style::{values as val, Style, unit::px, Color};
///
/// Style::default()
///     .and_border(|conf| {
///         conf.solid() // or .style(val::Solid)
///             .width(px(2))
///             .color(Color::DimGray)
///             .radius(px(4))
///     });
/// ```
// TODO: add shadow
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Border {
    #[rich(write(rename = left), write(style = compose))]
    pub left: Side,
    #[rich(write(rename = top), write(style = compose))]
    pub top: Side,
    #[rich(write(rename = right), write(style = compose))]
    pub right: Side,
    #[rich(write(rename = bottom), write(style = compose))]
    pub bottom: Side,
    #[rich(write(rename = top_left), write(option, rename = try_top_left))]
    pub top_left: Option<Radius>,
    #[rich(write(rename = top_right), write(option, rename = try_top_right))]
    pub top_right: Option<Radius>,
    #[rich(write(rename = bottom_left), write(option, rename = try_bottom_left))]
    pub bottom_left: Option<Radius>,
    #[rich(write(rename = bottom_right), write(option, rename = try_bottom_right))]
    pub bottom_right: Option<Radius>,
}

impl DeclarativeConfig for Border {}

impl UpdateStyleValues for Border {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
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

impl<T: Into<Color>> From<T> for Border {
    fn from(source: T) -> Self {
        Self::default().color(source.into())
    }
}

impl From<val::None> for Border {
    fn from(_: val::None) -> Self {
        Self::default().none()
    }
}

macro_rules! sides_style_shortcut_functions {
    ( $( $fn:ident() $(,)? )* ) => {
        $(
            pub fn $fn(self) -> Self {
                self.all_side(|side| side.$fn())
            }
        )*
    }
}

impl Border {
    pub fn all_side(self, value: impl Fn(Side) -> Side + Clone) -> Self {
        self.and_left(value.clone())
            .and_top(value.clone())
            .and_right(value.clone())
            .and_bottom(value)
    }

    pub fn style(self, style: impl Into<Style>) -> Self {
        let style = style.into();
        self.all_side(|side| side.style(style))
    }

    pub fn width(self, width: impl Into<Width>) -> Self {
        let width = width.into();
        self.all_side(|side| side.width(width.clone()))
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
        self.top_left(rad.clone())
            .top_right(rad.clone())
            .bottom_left(rad.clone())
            .bottom_right(rad)
    }

    sides_style_shortcut_functions! {
        none(), hidden(), dotted(), dashed(), solid(), double(),
        groove(), ridge(), inset(), outset(),
    }
}

#[derive(Rich, Clone, Debug, PartialEq, From, Default)]
pub struct Side {
    #[rich(write(rename = style), write(option, rename = try_style), value_fns = {
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
    pub style: Option<Style>,
    #[rich(write(rename = width), write(option, rename = try_width), value_fns = {
        thick = val::Thick,
        thin = val::Thin,
        medium = val::Medium,
        initial_width = val::Initial,
        inherit_width = val::Inherit,
    })]
    pub width: Option<Width>,
    #[rich(write(rename = color), write(option, rename = try_color))]
    pub color: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Style {
    None(val::None),
    Hidden(val::Hidden),
    Dotted(val::Dotted),
    Dashed(val::Dashed),
    Solid(val::Solid),
    Double(val::Double),
    Groove(val::Groove),
    Ridge(val::Ridge),
    Inset(val::Inset),
    Outset(val::Outset),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Width {
    Length(Length),
    Thin(val::Thin),
    Medium(val::Medium),
    Thick(val::Thick),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Radius {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}
