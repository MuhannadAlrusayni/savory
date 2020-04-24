use super::{
    unit::{self, *},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;

/// ```
/// use savory_html::css::{values as val, Style, unit::px};
///
/// Style::default()
///     .and_padding(|conf| {
///         conf.x(px(2)) // equal to conf.left(val::Auto).right(val::Auto)
///             .y(px(4))
///     });
/// ```
#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Padding {
    #[rich(write, write(option))]
    pub top: Option<Length>,
    #[rich(write, write(option))]
    pub right: Option<Length>,
    #[rich(write, write(option))]
    pub bottom: Option<Length>,
    #[rich(write, write(option))]
    pub left: Option<Length>,
}

impl From<Length> for Padding {
    fn from(source: Length) -> Self {
        Self::default().all(source)
    }
}

impl From<unit::Length> for Padding {
    fn from(source: unit::Length) -> Self {
        Self::default().all(source)
    }
}

impl From<Percent> for Padding {
    fn from(source: Percent) -> Self {
        Self::default().all(source)
    }
}

impl UpdateStyleValues for Padding {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
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
    Length(unit::Length),
    #[from(forward)]
    Percent(Percent),
    #[from]
    Inherit(val::Inherit),
}
