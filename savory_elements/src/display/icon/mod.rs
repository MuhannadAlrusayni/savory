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

impl<PMsg> Render for Icon<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        match self {
            Self::Svg(icon) => icon.render(),
            Self::Html(icon) => icon.render(),
            Self::Url(icon) => icon.render(),
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
