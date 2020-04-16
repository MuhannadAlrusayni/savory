use crate::css::{
    unit::{self, *},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;

/// ```
/// use savory_html::css::{Style, unit::em};
///
/// Style::default()
///     .and_size(|conf| {
///         conf.width(em(2.))
///             .height(em(1.5))
///             .min_width(em(1.5))
///             .min_height(em(1.))
///             .max_width(em(4.))
///             .max_height(em(3.))
///     });
/// ```
#[derive(Rich, Copy, Clone, Debug, PartialEq, Default)]
pub struct Size {
    #[rich(write(rename = width), write(option, rename = try_width))]
    pub width: Option<Length>,
    #[rich(write(rename = min_width), write(option, rename = try_min_width))]
    pub min_width: Option<Length>,
    #[rich(write(rename = max_width), write(option, rename = try_max_width))]
    pub max_width: Option<Length>,
    #[rich(write(rename = height), write(option, rename = try_height))]
    pub height: Option<Length>,
    #[rich(write(rename = min_height), write(option, rename = try_min_height))]
    pub min_height: Option<Length>,
    #[rich(write(rename = max_height), write(option, rename = try_max_height))]
    pub max_height: Option<Length>,
}

impl UpdateStyleValues for Size {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
            .try_add(St::Width, self.width)
            .try_add(St::MinWidth, self.min_width)
            .try_add(St::MaxWidth, self.max_width)
            .try_add(St::Height, self.height)
            .try_add(St::MinHeight, self.min_height)
            .try_add(St::MaxHeight, self.max_height)
    }
}

impl From<Length> for Size {
    fn from(source: Length) -> Self {
        Self::default().all(source)
    }
}

impl From<unit::Length> for Size {
    fn from(source: unit::Length) -> Self {
        Self::default().all(source)
    }
}

impl From<Percent> for Size {
    fn from(source: Percent) -> Self {
        Self::default().all(source)
    }
}

impl Size {
    pub fn full(self) -> Self {
        self.width(1.0).height(1.0)
    }

    pub fn half(self) -> Self {
        self.width(0.5).height(0.5)
    }

    pub fn min_content(self) -> Self {
        self.width(val::MinContent).height(val::MinContent)
    }

    pub fn max_content(self) -> Self {
        self.width(val::MaxContent).height(val::MaxContent)
    }

    pub fn auto(self) -> Self {
        self.width(val::Auto).height(val::Auto)
    }

    pub fn resize(self, width: impl Into<Length>, height: impl Into<Length>) -> Self {
        self.width(width).height(height)
    }

    pub fn all(self, val: impl Into<Length>) -> Self {
        let val = val.into();
        self.all_widths(val).all_heights(val)
    }

    pub fn all_widths(self, width: impl Into<Length>) -> Self {
        let width = width.into();
        self.width(width).min_width(width).max_width(width)
    }

    pub fn all_heights(self, val: impl Into<Length>) -> Self {
        let val = val.into();
        self.height(val).min_height(val).max_height(val)
    }
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Auto(val::Auto),
    #[from]
    MinContent(val::MinContent),
    #[from]
    MaxContent(val::MaxContent),
    #[from]
    Length(unit::Length),
    #[from(forward)]
    Percent(Percent),
}
