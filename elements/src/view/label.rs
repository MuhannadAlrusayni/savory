use crate::prelude::*;

use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Clone, Element)]
#[element(style(label), events(label))]
pub struct Label<PMsg> {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write)]
    pub styler: Option<Styler<Self, Style>>,
    #[rich(write)]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write)]
    #[element(theme_lens)]
    pub text: Cow<'static, str>,
}

impl<PMsg> Stylable for Label<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.label().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg> View for Label<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

pub type ThemeStyler<'a> = Styler<LabelLens<'a>, Style>;

impl<PMsg> StyledView for Label<PMsg> {
    fn styled_view(&self, style: Style) -> Self::Output {
        html::span()
            .try_id(self.id.clone())
            .class("label")
            .add(self.text.clone())
            .set(style.label)
            .set(&self.events.label)
    }
}

impl<T: ToString, PMsg> From<T> for Label<PMsg> {
    fn from(source: T) -> Self {
        Self::new(source.to_string())
    }
}

impl<PMsg> Label<PMsg> {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: None,
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            text: text.into(),
        }
    }
}
