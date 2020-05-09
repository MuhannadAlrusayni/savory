use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element, Clone)]
#[element(style(url_icon), events(url_icon))]
pub struct Url {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<Self, Style>>,
    #[rich(write(style = compose))]
    pub theme: Theme,

    #[rich(write)]
    pub url: Cow<'static, str>,
}

impl<T: ToString> From<T> for Url {
    fn from(url: T) -> Self {
        Self::new(url.to_string())
    }
}

impl Url {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: None,
            styler: None,
            theme: Theme::default(),
            url: url.into(),
        }
    }
}

impl Stylable for Url {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.url_icon().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<Msg> View<Node<Msg>> for Url {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl<Msg> StyledView<Node<Msg>> for Url {
    fn styled_view(&self, style: Self::Style) -> Node<Msg> {
        html::img()
            .try_id(self.id.clone())
            .class("url-icon")
            .set(att::src(self.url.clone()))
            .set(style.url_icon)
    }
}
