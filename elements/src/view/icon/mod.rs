pub mod html;
pub mod svg;
pub mod url;

use crate::prelude::*;
use savory_core::prelude::*;
use std::borrow::Cow;

pub use html::Html;
pub use svg::Svg;
pub use url::Url;

#[derive(Clone, From)]
pub enum Icon<PMsg> {
    #[from]
    Svg(Svg<PMsg>),
    #[from]
    Html(Html),
    #[from]
    Url(Url),
}

#[derive(Clone, Debug, PartialEq, From)]
pub enum Style {
    None,
    Svg(svg::Style),
    Html(html::Style),
    Url(url::Style),
}

impl Default for Style {
    fn default() -> Self {
        Self::None
    }
}

impl<PMsg> View<Node<PMsg>> for Icon<PMsg> {
    fn view(&self) -> Node<PMsg> {
        match self {
            Self::Svg(icon) => icon.view(),
            Self::Html(icon) => icon.view(),
            Self::Url(icon) => icon.view(),
        }
    }
}

impl<PMsg> Stylable for Icon<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        Styler::from(|icon: &Self| match icon {
            Icon::Svg(icon) => Style::from(icon.style()),
            Icon::Html(icon) => Style::from(icon.style()),
            Icon::Url(icon) => Style::from(icon.style()),
        })
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg> StyledView<Node<PMsg>> for Icon<PMsg> {
    fn styled_view(&self, style: Self::Style) -> Node<PMsg> {
        match self {
            Self::Svg(icon) => {
                if let Style::Svg(style) = style {
                    icon.styled_view(style)
                } else {
                    icon.view()
                }
            }
            Self::Html(icon) => {
                if let Style::Html(style) = style {
                    icon.styled_view(style)
                } else {
                    icon.view()
                }
            }
            Self::Url(icon) => {
                if let Style::Url(style) = style {
                    icon.styled_view(style)
                } else {
                    icon.view()
                }
            }
        }
    }
}

impl<PMsg> Icon<PMsg> {
    pub fn svg(draw: impl IntoIterator<Item = Node<PMsg>>) -> Svg<PMsg> {
        Svg::new(draw)
    }

    pub fn html(html: impl Into<Cow<'static, str>>) -> Html {
        Html::new(html)
    }

    pub fn url(url: impl Into<Cow<'static, str>>) -> Url {
        Url::new(url)
    }
}
