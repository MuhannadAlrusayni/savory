use crate::{
    css::{self, color::Color, size::Size, values as val},
    render::Render,
    theme::{Theme, Themeable},
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Clone, Debug, From)]
pub enum Icon<Msg: 'static> {
    #[from]
    Svg(SvgIcon<Msg>),
    #[from]
    Html(HtmlIcon),
    #[from]
    Url(UrlIcon),
}

// impl<Msg: 'static, T: Into<UrlIcon>> From<T> for Icon<Msg> {
//     fn from(url: T) -> Self {
//         url.into().into()
//     }
// }

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for Icon<Msg> {
    type View = Node<ParentMsg>;

    fn render(
        &self,
        theme: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        match self {
            Self::Svg(icon) => icon.render(theme, map_msg),
            Self::Html(icon) => icon.render(theme, map_msg),
            Self::Url(icon) => icon.render(theme, map_msg),
        }
    }
}

impl<Msg: 'static> Icon<Msg> {
    pub fn svg(draw: impl IntoIterator<Item = Node<Msg>>) -> SvgIcon<Msg> {
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

#[derive(Clone, Debug, Rich)]
pub struct SvgIcon<Msg: 'static> {
    pub draw: Vec<Node<Msg>>,
    #[rich(write(take))]
    pub color: Option<Color>,
    #[rich(write(take, style = compose))]
    pub size: Size,
}

impl<Msg: 'static> SvgIcon<Msg> {
    pub fn new(draw: impl IntoIterator<Item = Node<Msg>>) -> Self {
        Self {
            draw: draw.into_iter().collect(),
            color: None,
            size: Size::default(),
        }
    }

    pub fn draw(mut self, draw: impl IntoIterator<Item = Node<Msg>>) -> Self {
        self.draw = draw.into_iter().collect();
        self
    }
}

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for SvgIcon<Msg> {
    type View = Node<ParentMsg>;

    fn render(
        &self,
        theme: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        svg![
            theme.svg_icon(self),
            attrs![
                At::ViewBox => "0 0 100 100",
            ],
            self.draw.clone(),
        ]
        .map_msg(map_msg)
    }
}

impl<Msg: 'static> Themeable for SvgIcon<Msg> {
    type StyleMap = css::Style;
}

#[derive(Debug, Clone, Rich)]
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

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for HtmlIcon {
    type View = Node<ParentMsg>;

    fn render(
        &self,
        theme: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        svg![
            theme.html_icon(self),
            attrs![
                At::ViewBox => "0 0 100 100",
            ],
            raw![self.html.as_ref()],
        ]
        .map_msg(map_msg)
    }
}

impl Themeable for HtmlIcon {
    type StyleMap = css::Style;
}

#[derive(Debug, Rich, Clone)]
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

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for UrlIcon {
    type View = Node<ParentMsg>;

    fn render(
        &self,
        theme: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        img![
            theme.url_icon(self),
            attrs![
                At::Src => self.url,
            ]
        ]
        .map_msg(map_msg)
    }
}

impl Themeable for UrlIcon {
    type StyleMap = css::Style;
}
