use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Clone, Element, Rich)]
#[element(style(html_icon), events(html_icon))]
pub struct Html<PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<PMsg>>,
    #[rich(write(style = compose))]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write)]
    #[element(theme_lens)]
    pub view_box: Option<att::ViewBox>,
    #[rich(write)]
    pub html: Cow<'static, str>,
}

impl<PMsg> Html<PMsg> {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            view_box: None,
            html: html.into(),
        }
    }
}

impl<PMsg> View for Html<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.html_icon().get(&self.theme_lens())),
        )
    }
}

impl<PMsg> StyledView for Html<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Self::Style) -> Self::Output {
        html::svg()
            .class("html-icon")
            .try_set(self.view_box)
            .set(style.html_icon)
            .set(&self.events.html_icon)
            .add(html::raw(self.html.as_ref()))
    }
}

pub type Styler<PMsg> = theme::Styler<Html<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<HtmlLens<'a>, Style>;
