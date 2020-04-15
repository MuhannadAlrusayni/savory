use crate::css::{color::Color, unit::*, values as val, St, StyleValues, UpdateStyleValues};
use derive_rich::Rich;
use std::ops::{Add, AddAssign};

/// ```
/// use savory::css::{values as val, Style, unit::px, Color};
///
/// let mut style = Style::default();
/// style
///     .and_border(|conf| {
///         conf.solid() // or .style(val::Solid)
///             .set_width(px(2))
///             .set_color(Color::DimGray)
///             .set_radius(px(4))
///     });
/// ```
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

impl Add for Border {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for Border {
    fn add_assign(&mut self, other: Self) {
        self.left += other.left;
        self.top += other.top;
        self.right += other.right;
        self.bottom += other.bottom;
        self.top_left = other.top_left.or_else(|| self.top_left);
        self.top_right = other.top_right.or_else(|| self.top_right);
        self.bottom_left = other.bottom_left.or_else(|| self.bottom_left);
        self.bottom_right = other.bottom_right.or_else(|| self.bottom_right);
    }
}

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
    pub fn all_side(self, value: impl Fn(Side) -> Side + Copy) -> Self {
        self.and_left(value)
            .and_top(value)
            .and_right(value)
            .and_bottom(value)
    }

    pub fn set_style(self, style: impl Into<Style>) -> Self {
        let style = style.into();
        self.all_side(|side| side.set_style(style))
    }

    pub fn set_width(self, width: impl Into<Width>) -> Self {
        let width = width.into();
        self.all_side(|side| side.set_width(width))
    }

    pub fn set_color(self, color: impl Into<Color>) -> Self {
        let color = color.into();
        self.all_side(|side| side.set_color(color))
    }

    pub fn transparent(self) -> Self {
        self.set_color(Color::Transparent)
    }

    pub fn set_radius(self, rad: impl Into<Radius>) -> Self {
        let rad = rad.into();
        self.set_top_left(rad)
            .set_top_right(rad)
            .set_bottom_left(rad)
            .set_bottom_right(rad)
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

impl Add for Side {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for Side {
    fn add_assign(&mut self, other: Self) {
        self.style = other.style.or_else(|| self.style);
        self.color = other.color.or_else(|| self.color);
        self.width = other.width.or_else(|| self.width);
    }
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
    Length(Length),
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
    Length(Length),
    #[from(forward)]
    Percent(Percent),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}
