use crate::css::{self, values as val, unit::*, St, Style, ToStyle};

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Visibility {
    #[from]
    Visible(val::Visible),
    #[from]
    Hidden(val::Hidden),
    #[from]
    Collapse(val::Collapse),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

impl ToStyle for Visibility {
    fn to_style(&self) -> Style {
        Style::new().add(St::Visibility, self)
    }
}
