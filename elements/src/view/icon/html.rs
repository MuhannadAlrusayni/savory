use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Clone, Element, Rich)]
#[element(style(html_icon), events(html_icon))]
pub struct Html<PMsg> {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<Self, Style>>,
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
            id: None,
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            view_box: None,
            html: html.into(),
        }
    }
}

impl<PMsg> Stylable for Html<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.html_icon().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg> View for Html<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

impl<PMsg> StyledView for Html<PMsg> {
    fn styled_view(&self, style: Self::Style) -> Self::Output {
        html::svg()
            .try_id(self.id.clone())
            .class("html-icon")
            .try_set(self.view_box)
            .set(style.html_icon)
            .set(&self.events.html_icon)
            .add(html::raw(self.html.as_ref()))
    }
}

pub type ThemeStyler<'a> = Styler<HtmlLens<'a>, Style>;
