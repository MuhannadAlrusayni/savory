use crate::css::{values as val, St, StyleValues, UpdateStyleValues};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
#[display(fmt = "visibility: {};")]
pub enum Visibility {
    Visible(val::Visible),
    Hidden(val::Hidden),
    Collapse(val::Collapse),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

impl From<bool> for Visibility {
    fn from(source: bool) -> Self {
        if source {
            val::Visible.into()
        } else {
            val::Hidden.into()
        }
    }
}

impl UpdateStyleValues for Visibility {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::Visibility, self.to_string())
    }
}
