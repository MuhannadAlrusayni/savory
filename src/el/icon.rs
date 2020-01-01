use crate::{
    css::{color::Color, size::Size},
    theme::Theme,
    render::Render,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Debug, From)]
pub enum Icon<ParentMsg: 'static> {
    #[from]
    Svg(SvgIcon<ParentMsg>),
    #[from]
    Html(HtmlIcon),
    #[from]
    Url(UrlIcon),
}

// impl<ParentMsg: 'static, T: Into<UrlIcon>> From<T> for Icon<ParentMsg> {
//     fn from(url: T) -> Self {
//         url.into().into()
//     }
// }

impl<ParentMsg: Clone + 'static> Render<ParentMsg> for Icon<ParentMsg> {
    fn render(&self, theme: &impl Theme) -> Node<ParentMsg> {
        match self {
            Self::Svg(icon) => icon.render(theme),
            Self::Html(icon) => icon.render(theme),
            Self::Url(icon) => icon.render(theme),
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

impl<ParentMsg: Clone + 'static> Render<ParentMsg> for SvgIcon<ParentMsg> {
    fn render(&self, theme: &impl Theme) -> Node<ParentMsg> {
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

impl<ParentMsg: Clone + 'static> Render<ParentMsg> for HtmlIcon {
    fn render(&self, theme: &impl Theme) -> Node<ParentMsg> {
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

impl<T: ToString> From<T> for UrlIcon {
    fn from(url: T) -> Self {
        Self::new(url.to_string())
    }
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

impl<ParentMsg: Clone + 'static> Render<ParentMsg> for UrlIcon {
    fn render(&self, theme: &impl Theme) -> Node<ParentMsg> {
        img![
            theme.url_icon(self),
            attrs![
                At::Src => self.url,
            ]
        ]
    }
}
