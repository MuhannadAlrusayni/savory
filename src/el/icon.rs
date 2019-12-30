use crate::{
    css::{color::Color, size::Size},
    theme::Theme,
    view::View,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Debug, From)]
pub enum Icon<ParentMsg: 'static> {
    Svg(SvgIcon<ParentMsg>),
    Html(HtmlIcon),
    Url(UrlIcon),
}

impl<ParentMsg: Clone + 'static> View<ParentMsg> for Icon<ParentMsg> {
    fn view(&self, theme: &impl Theme) -> Node<ParentMsg> {
        match self {
            Self::Svg(icon) => icon.view(theme),
            Self::Html(icon) => icon.view(theme),
            Self::Url(icon) => icon.view(theme),
        }
    }
}

impl<ParentMsg: 'static> Icon<ParentMsg> {
    pub fn svg(draw: impl IntoIterator<Item = Node<ParentMsg>>) -> SvgIcon<ParentMsg> {
        SvgIcon::new(draw)
    }
}

impl Icon<!> {
    pub fn html(html: impl Into<Cow<'static, str>>) -> HtmlIcon {
        HtmlIcon::new(html)
    }

    pub fn url(url: impl Into<Cow<'static, str>>) -> UrlIcon {
        UrlIcon::new(url)
    }
}

#[derive(Debug, Rich)]
pub struct SvgIcon<ParentMsg: 'static> {
    pub draw: Vec<Node<ParentMsg>>,
    #[rich(write(take))]
    pub color: Option<Color>,
    #[rich(write(take, style = compose))]
    pub size: Size,
}

impl<ParentMsg: 'static> SvgIcon<ParentMsg> {
    pub fn new(draw: impl IntoIterator<Item = Node<ParentMsg>>) -> Self {
        Self {
            draw: draw.into_iter().collect(),
            color: None,
            size: Size::default(),
        }
    }

    pub fn draw(mut self, draw: impl IntoIterator<Item = Node<ParentMsg>>) -> Self {
        self.draw = draw.into_iter().collect();
        self
    }
}

impl<ParentMsg: Clone + 'static> View<ParentMsg> for SvgIcon<ParentMsg> {
    fn view(&self, theme: &impl Theme) -> Node<ParentMsg> {
        svg![
            theme.svg_icon(self),
            attrs![
                At::ViewBox => "0 0 100 100",
            ],
            self.draw.clone(),
        ]
    }
}


#[derive(Debug, Rich)]
pub struct HtmlIcon {
    pub html: Cow<'static, str>,
    #[rich(write(take))]
    pub color: Option<Color>,
    #[rich(write(take, style = compose))]
    pub size: Size,
}

impl HtmlIcon {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            html: html.into(),
            color: None,
            size: Size::default(),
        }
    }

    pub fn html(mut self, html: impl Into<Cow<'static, str>>) -> Self {
        self.html = html.into();
        self
    }
}

impl<ParentMsg: Clone + 'static> View<ParentMsg> for HtmlIcon {
    fn view(&self, theme: &impl Theme) -> Node<ParentMsg> {
        svg![
            theme.html_icon(self),
            attrs![
                At::ViewBox => "0 0 100 100",
            ],
            raw![self.html.as_ref()],
        ]
    }
}


#[derive(Debug, Rich)]
pub struct UrlIcon {
    pub url: Cow<'static, str>,
    #[rich(write(take, style = compose))]
    pub size: Size,
}

impl UrlIcon {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            url: url.into(),
            size: Size::default(),
        }
    }

    pub fn url(mut self, url: impl Into<Cow<'static, str>>) -> Self {
        self.url = url.into();
        self
    }
}

impl<ParentMsg: Clone + 'static> View<ParentMsg> for UrlIcon {
    fn view(&self, theme: &impl Theme) -> Node<ParentMsg> {
        img![
            theme.url_icon(self),
            attrs![
                At::Src => self.url,
            ]
        ]
    }
}
