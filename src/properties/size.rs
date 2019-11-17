use crate::{macros::*, properties::unit::*};
use seed::{dom_types::Style, prelude::*};

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "min-content")]
    MinContent,
    #[display(fmt = "max-content")]
    MaxContent,
    #[from]
    Em(Em),
    #[from]
    Ex(Ex),
    #[from]
    Cap(Cap),
    #[from]
    Ch(Ch),
    #[from]
    Ic(Ic),
    #[from]
    Rem(Rem),
    #[from]
    Rlh(Rlh),
    #[from]
    Vm(Vm),
    #[from]
    Vh(Vh),
    #[from]
    Vi(Vi),
    #[from]
    Vb(Vb),
    #[from]
    Vmin(Vmin),
    #[from]
    Vmax(Vmax),
    #[from]
    Cm(Cm),
    #[from]
    Mm(Mm),
    #[from]
    Q(Q),
    #[from]
    In(In),
    #[from]
    Pc(Pc),
    #[from]
    Pt(Pt),
    #[from]
    Px(Px),
    #[from(forward)]
    Percent(Percent),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Size {
    width: Option<Length>,
    min_width: Option<Length>,
    max_width: Option<Length>,
    height: Option<Length>,
    min_height: Option<Length>,
    max_height: Option<Length>,
}

impl Size {
    pub fn full(self) -> Self {
        self.full_width().full_height()
    }

    pub fn half(self) -> Self {
        self.half_width().half_height()
    }

    pub fn as_min_content(self) -> Self {
        self.height_as_min_content().width_as_min_content()
    }

    pub fn as_max_content(self) -> Self {
        self.height_as_max_content().width_as_max_content()
    }

    pub fn auto(self) -> Self {
        self.auto_height().auto_width()
    }

    pub fn resize(self, width: impl Into<Length>, height: impl Into<Length>) -> Self {
        self.width(width).height(height)
    }

    builder_functions! {
        width(Length),
        min_width(Length),
        max_width(Length),
        height(Length),
        min_height(Length),
        max_height(Length),
    }

    builder_enum_functions! {
        width {
            width_as_min_content() => Length::MinContent,
            width_as_max_content() => Length::MaxContent,
            auto_width() => Length::Auto,
            full_width() => 1.0,
            half_width() => 0.5,
        },
        min_width {
            min_width_as_min_content() => Length::MinContent,
            min_width_as_max_content() => Length::MaxContent,
            auto_min_width() => Length::Auto,
            full_min_width() => 1.0,
            half_min_width() => 0.5,
        },
        max_width {
            max_width_as_min_content() => Length::MinContent,
            max_width_as_max_content() => Length::MaxContent,
            auto_max_width() => Length::Auto,
            full_max_width() => 1.0,
            half_max_width() => 0.5,
        },
        height {
            height_as_min_content() => Length::MinContent,
            height_as_max_content() => Length::MaxContent,
            auto_height() => Length::Auto,
            full_height() => 1.0,
            half_height() => 0.5,
        },
        min_height {
            min_height_as_min_content() => Length::MinContent,
            min_height_as_max_content() => Length::MaxContent,
            auto_min_height() => Length::Auto,
            full_min_height() => 1.0,
            half_min_height() => 0.5,
        },
        max_height {
            max_height_as_min_content() => Length::MinContent,
            max_height_as_max_content() => Length::MaxContent,
            auto_max_height() => Length::Auto,
            full_max_height() => 1.0,
            half_max_height() => 0.5,
        }
    }
}

impl From<&Size> for Style {
    fn from(size: &Size) -> Self {
        style![
            St::Width => size.height,
            St::MinWidth => size.min_width,
            St::MaxWidth => size.max_width,
            St::Height => size.height,
            St::MinHeight => size.min_height,
            St::MaxHeight => size.max_height,
        ]
    }
}

#[cfg(test)]
mod tests {}
