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
        let mut map = StyleMap::default();
        map.try_add(St::Width, self.width)
            .try_add(St::MinWidth, self.min_width)
            .try_add(St::MaxWidth, self.max_width)
            .try_add(St::Height, self.height)
            .try_add(St::MinHeight, self.min_height)
            .try_add(St::MaxHeight, self.max_height);
        map
    }
}

impl Size {
    pub fn full(&mut self) -> &mut Self {
        self.width(1.0).height(1.0)
    }

    pub fn half(&mut self) -> &mut Self {
        self.width(0.5).height(0.5)
    }

    pub fn min_content(&mut self) -> &mut Self {
        self.width(val::MinContent).height(val::MinContent)
    }

    pub fn max_content(&mut self) -> &mut Self {
        self.width(val::MaxContent).height(val::MaxContent)
    }

    pub fn auto(&mut self) -> &mut Self {
        self.width(val::Auto).height(val::Auto)
    }

    pub fn resize(&mut self, width: impl Into<Length>, height: impl Into<Length>) -> &mut Self {
        self.width(width).height(height)
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
