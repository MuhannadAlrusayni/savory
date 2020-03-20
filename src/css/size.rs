use crate::css::{
    unit::{self, *},
    values as val, St, StyleMap, ToStyleMap,
};
use derive_rich::Rich;

/// ```
/// use khalas::css::{Style, unit::em};
///
/// let mut style = Style::default();
/// style
///     .and_size(|conf| {
///         conf.set_width(em(2.))
///             .set_height(em(1.5))
///             .set_min_width(em(1.5))
///             .set_min_height(em(1.))
///             .set_max_width(em(4.))
///             .set_max_height(em(3.))
///     });
/// ```
#[derive(Rich, Copy, Clone, Debug, PartialEq, Default)]
pub struct Size {
    #[rich(read, write)]
    width: Option<Length>,
    #[rich(read, write)]
    min_width: Option<Length>,
    #[rich(read, write)]
    max_width: Option<Length>,
    #[rich(read, write)]
    height: Option<Length>,
    #[rich(read, write)]
    min_height: Option<Length>,
    #[rich(read, write)]
    max_height: Option<Length>,
}

impl ToStyleMap for Size {
    fn style_map(&self) -> StyleMap {
        StyleMap::default()
            .try_add(St::Width, self.width)
            .try_add(St::MinWidth, self.min_width)
            .try_add(St::MaxWidth, self.max_width)
            .try_add(St::Height, self.height)
            .try_add(St::MinHeight, self.min_height)
            .try_add(St::MaxHeight, self.max_height)
    }
}

impl Size {
    pub fn full(self) -> Self {
        self.set_width(1.0).set_height(1.0)
    }

    pub fn half(self) -> Self {
        self.set_width(0.5).set_height(0.5)
    }

    pub fn min_content(self) -> Self {
        self.set_width(val::MinContent).set_height(val::MinContent)
    }

    pub fn max_content(self) -> Self {
        self.set_width(val::MaxContent).set_height(val::MaxContent)
    }

    pub fn auto(self) -> Self {
        self.set_width(val::Auto).set_height(val::Auto)
    }

    pub fn resize(self, width: impl Into<Length>, height: impl Into<Length>) -> Self {
        self.set_width(width).set_height(height)
    }

    pub fn set_all(self, val: impl Into<Length>) -> Self {
        let val = val.into();
        self.set_all_widths(val).set_all_heights(val)
    }

    pub fn set_all_widths(self, width: impl Into<Length>) -> Self {
        let width = width.into();
        self.set_width(width)
            .set_min_width(width)
            .set_max_width(width)
    }

    pub fn set_all_heights(self, val: impl Into<Length>) -> Self {
        let val = val.into();
        self.set_height(val).set_min_height(val).set_max_height(val)
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
