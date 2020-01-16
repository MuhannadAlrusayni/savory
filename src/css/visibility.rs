use crate::css::{self, unit::*, St, Style, ToStyle};

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Visibility {
    #[from]
    Visible(css::Visible),
    #[from]
    Hidden(css::Hidden),
    #[from]
    Collapse(css::Collapse),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

impl ToStyle for Visibility {
    fn to_style(&self) -> Style {
        Style::new().add(St::Visibility, self)
    }
}
