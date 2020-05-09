use crate::prelude::*;

use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Clone, Element)]
#[element(style(label))]
pub struct Label {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write)]
    pub styler: Option<<Label as Stylable>::Styler>,
    #[rich(write)]
    pub theme: Theme,

    #[rich(write)]
    pub text: Cow<'static, str>,
}

impl Stylable for Label {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.label().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<Msg> View<Node<Msg>> for Label {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl<Msg> StyledView<Node<Msg>> for Label {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        html::span()
            .try_id(self.id.clone())
            .class("label")
            .add(self.text.clone())
            .set(style.label)
    }
}

impl<T: ToString> From<T> for Label {
    fn from(source: T) -> Self {
        Self::new(source.to_string())
    }
}

impl Label {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: None,
            styler: None,
            theme: Theme::default(),
            text: text.into(),
        }
    }
}
