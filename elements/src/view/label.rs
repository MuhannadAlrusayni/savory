use crate::prelude::*;

use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Clone, Element)]
#[element(style(label), events(label))]
pub struct Label<PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(rename = styler))]
    pub styler: Option<Styler<PMsg>>,
    #[rich(write(rename = theme))]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write(rename = text))]
    #[element(theme_lens)]
    pub text: Cow<'static, str>,
}

impl<PMsg> View for Label<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(&self))
                .unwrap_or_else(|| self.theme.label()(&self.theme_lens())),
        )
    }
}

impl<PMsg> StyledView for Label<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        html::span()
            .set(att::class("label"))
            .add(self.text.clone())
            .set(&style.label)
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
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            text: text.into(),
        }
    }
}

pub type Styler<PMsg> = theme::Styler<Label<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<LabelLens<'a>, Style>;
