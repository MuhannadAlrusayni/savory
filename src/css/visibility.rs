use crate::css::{values as val, St, StyleMap, ToStyleMap};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
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

impl From<bool> for Visibility {
    fn from(source: bool) -> Self {
        if source {
            val::Visible.into()
        } else {
            val::Hidden.into()
        }
    }
}

impl ToStyleMap for Visibility {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::Visibility, self);
        map
    }
}
