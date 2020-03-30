use crate::css::{
    unit::{self, *},
    values as val, St, StyleMap, ToStyleMap,
};
use derive_rich::Rich;

/// ```
/// use khalas::css::{values as val, Style, unit::px};
///
/// let mut style = Style::default();
/// style
///     .and_margin(|conf| {
///         conf.set_x(val::Auto)
///             .set_y(px(4))
///     });
/// ```
#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Margin {
    #[rich(read, write)]
    top: Option<Length>,
    #[rich(read, write)]
    right: Option<Length>,
    #[rich(read, write)]
    bottom: Option<Length>,
    #[rich(read, write)]
    left: Option<Length>,
}

impl_add_and_add_assign!(Margin { top right bottom left });

impl From<Length> for Margin {
    fn from(source: Length) -> Self {
        Self::default().set_all(source)
    }
}

impl ToStyleMap for Margin {
    fn style_map(&self) -> StyleMap {
        StyleMap::default()
            .try_add(St::MarginTop, self.top)
            .try_add(St::MarginRight, self.right)
            .try_add(St::MarginBottom, self.bottom)
            .try_add(St::MarginLeft, self.left)
    }
}

impl Margin {
    pub fn set_all(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.set_right(value)
            .set_top(value)
            .set_left(value)
            .set_bottom(value)
    }

    pub fn zero(self) -> Self {
        self.set_all(px(0.))
    }

    pub fn set_x(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.set_left(value).set_right(value)
    }

    pub fn set_y(self, value: impl Into<Length>) -> Self {
        let value = value.into();
        self.set_top(value).set_bottom(value)
    }

    pub fn set_horizontal(self, value: impl Into<Length>) -> Self {
        self.set_y(value)
    }

    pub fn set_vertical(self, value: impl Into<Length>) -> Self {
        self.set_x(value)
    }

    pub fn auto(self) -> Self {
        self.set_all(val::Auto)
    }

    pub fn full(self) -> Self {
        self.set_all(1.)
    }

    pub fn half(self) -> Self {
        self.set_all(0.5)
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
