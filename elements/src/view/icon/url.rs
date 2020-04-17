use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element, Clone)]
#[element(style(url_icon), events(url_icon))]
pub struct Url<PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<PMsg>>,
    #[rich(write(style = compose))]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write)]
    #[element(theme_lens)]
    pub url: Cow<'static, str>,
}

impl<PMsg, T: ToString> From<T> for Url<PMsg> {
    fn from(url: T) -> Self {
        Self::new(url.to_string())
    }
}

impl<PMsg> Url<PMsg> {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            url: url.into(),
        }
    }
}

impl<PMsg> View for Url<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(&self))
                .unwrap_or_else(|| self.theme.url_icon()(&self.theme_lens())),
        )
    }
}

impl<PMsg> StyledView for Url<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Self::Style) -> Self::Output {
        html::img()
            .class("url-icon")
            .set(att::src(self.url.clone()))
            .set(style.url_icon)
            .set(&self.events.url_icon)
    }
}

pub type Styler<PMsg> = theme::Styler<Url<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<UrlLens<'a>, Style>;
