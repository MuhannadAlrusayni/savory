use crate::css::{
    unit::{self, *},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;

/// ```
/// use savory::css::{values as val, Style, unit::px};
///
/// Style::default()
///     .and_margin(|conf| {
///         conf.x(val::Auto) // equal to conf.left(val::Auto).right(val::Auto)
///             .y(px(4))
///     });
/// ```
#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Margin {
    #[rich(write(rename = top), write(option, rename = try_top))]
    pub top: Option<Length>,
    #[rich(write(rename = right), write(option, rename = try_right))]
    pub right: Option<Length>,
    #[rich(write(rename = bottom), write(option, rename = try_bottom))]
    pub bottom: Option<Length>,
    #[rich(write(rename = left), write(option, rename = try_left))]
    pub left: Option<Length>,
}

impl From<Length> for Margin {
    fn from(source: Length) -> Self {
        Self::default().all(source)
    }
}

impl UpdateStyleValues for Margin {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
            .try_add(St::MarginTop, self.top)
            .try_add(St::MarginRight, self.right)
            .try_add(St::MarginBottom, self.bottom)
            .try_add(St::MarginLeft, self.left)
    }
}

impl Margin {
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

    pub fn auto(self) -> Self {
        self.all(val::Auto)
    }

    pub fn full(self) -> Self {
        self.all(1.)
    }

    pub fn half(self) -> Self {
        self.all(0.5)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Auto(val::Auto),
    #[from]
    Inherit(val::Inherit),
    #[from]
    Initial(val::Initial),
    #[from]
    Length(unit::Length),
    #[from(forward)]
    Percent(Percent),
}
