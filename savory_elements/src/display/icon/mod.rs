pub mod html;
pub mod svg;
pub mod url;

use savory::prelude::*;
use savory_html::prelude::*;

use html::Html;
use std::borrow::Cow;
use svg::Svg;
use url::Url;

#[derive(Clone, From)]
pub enum Icon<PMsg> {
    #[from]
    Svg(Svg<PMsg>),
    #[from]
    Html(Html<PMsg>),
    #[from]
    Url(Url<PMsg>),
}

pub enum IconLens<'lens> {
    Svg(svg::SvgLens<'lens>),
    Html(html::HtmlLens<'lens>),
    Url(url::UrlLens<'lens>),
}

impl<'lens, PMsg> crate::theme::ThemeLens<'lens> for Icon<PMsg> {
    type Lens = IconLens<'lens>;

    fn theme_lens(&'lens self) -> Self::Lens {
        match self {
            Self::Svg(svg) => IconLens::Svg(svg.theme_lens()),
            Self::Html(html) => IconLens::Html(html.theme_lens()),
            Self::Url(url) => IconLens::Url(url.theme_lens()),
        }
    }
}

impl<PMsg> View for Icon<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        match self {
            Self::Svg(icon) => icon.view(),
            Self::Html(icon) => icon.view(),
            Self::Url(icon) => icon.view(),
        }
    }
}

impl<PMsg> Icon<PMsg> {
    pub fn svg(draw: impl IntoIterator<Item = Node<PMsg>>) -> Svg<PMsg> {
        Svg::new(draw)
    }

    pub fn html(html: impl Into<Cow<'static, str>>) -> Html<PMsg> {
        Html::new(html)
    }

    pub fn url(url: impl Into<Cow<'static, str>>) -> Url<PMsg> {
        Url::new(url)
    }
}
