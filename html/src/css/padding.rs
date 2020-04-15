use super::{
    unit::{self, *},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;

/// ```
/// use savory::css::{values as val, Style, unit::px};
///
/// let mut style = Style::default();
/// style
///     .and_padding(|conf| {
///         conf.set_x(px(2))
///             .set_y(px(4))
///     });
/// ```
#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Padding {
    #[rich(read, write)]
    top: Option<Length>,
    #[rich(read, write)]
    right: Option<Length>,
    #[rich(read, write)]
    bottom: Option<Length>,
    #[rich(read, write)]
    left: Option<Length>,
}

impl From<Length> for Padding {
    fn from(source: Length) -> Self {
        Padding::default().set_all(source)
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
    Length(unit::Length),
    #[from(forward)]
    Percent(Percent),
    #[from]
    Inherit(val::Inherit),
}
