use crate::{
    unit::{self, *},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;
use savory::prelude::DeclarativeConfig;

/// ```
/// use savory_style::{values as val, Style, unit::px};
///
/// Style::default()
///     .and_margin(|conf| {
///         conf.x(val::Auto) // equal to conf.left(val::Auto).right(val::Auto)
///             .y(px(4))
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, From, Default)]
pub struct Margin {
    #[rich(write, write(option))]
    pub top: Option<Length>,
    #[rich(write, write(option))]
    pub right: Option<Length>,
    #[rich(write, write(option))]
    pub bottom: Option<Length>,
    #[rich(write, write(option))]
    pub left: Option<Length>,
}

impl DeclarativeConfig for Margin {}

impl From<val::Auto> for Margin {
    fn from(_: val::Auto) -> Self {
        Margin::default().auto()
    }
}

impl From<Length> for Margin {
    fn from(source: Length) -> Self {
        Self::default().all(source)
    }
}

impl From<unit::Length> for Margin {
    fn from(source: unit::Length) -> Self {
        Self::default().all(source)
    }
}

impl From<Percent> for Margin {
    fn from(source: Percent) -> Self {
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
        self.right(value.clone())
            .top(value.clone())
            .left(value.clone())
            .bottom(value)
    }

    pub fn zero(self) -> Self {
        self.all(px(0.))
    }

    pub fn x(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.left(value.clone()).right(value)
    }

    pub fn y(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.top(value.clone()).bottom(value)
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

#[derive(Clone, Debug, PartialEq, Display, From)]
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
