use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Clone, Element, Rich)]
#[element(style(html_icon), events(html_icon))]
pub struct Html {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<Self, Style>>,
    #[rich(write(style = compose))]
    pub theme: Theme,

    #[rich(write)]
    pub view_box: Option<att::ViewBox>,
    #[rich(write)]
    pub html: Cow<'static, str>,
}

impl Html {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: None,
            styler: None,
            theme: Theme::default(),
            view_box: None,
            html: html.into(),
        }
    }
}

impl Stylable for Html {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.html_icon().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<Msg> View<Node<Msg>> for Html {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl<Msg> StyledView<Node<Msg>> for Html {
    fn styled_view(&self, style: Self::Style) -> Node<Msg> {
        html::svg()
            .try_id(self.id.clone())
            .class("html-icon")
            .try_set(self.view_box)
            .set(style.html_icon)
            .add(html::raw(self.html.as_ref()))
    }
}
