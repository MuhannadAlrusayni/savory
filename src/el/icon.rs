use crate::{
    macros::*,
    properties::{color::Color, size::Size},
    theme::Theme,
    view::View,
};
use seed::{dom_types::Style, prelude::*};
use std::borrow::Cow;

#[derive(Clone, Debug, From)]
pub enum Icon<Msg: 'static + Clone> {
    Svg(SvgIcon<Msg>),
    Html(HtmlIcon),
    Url(UrlIcon),
}

impl<Msg: 'static + Clone> Icon<Msg> {
    pub fn svg(draw: impl IntoIterator<Item = El<Msg>>) -> SvgIcon<Msg> {
        SvgIcon::new(draw)
    }

    pub fn html(html: impl Into<Cow<'static, str>>) -> HtmlIcon {
        HtmlIcon::new(html)
    }

    pub fn url(url: impl Into<Cow<'static, str>>) -> UrlIcon {
        UrlIcon::new(url)
    }
}

#[derive(Clone, Debug)]
pub struct SvgIcon<Msg: 'static + Clone> {
    draw: Vec<El<Msg>>,
    color: Option<Color>,
    size: Size,
}

impl<Msg: 'static + Clone> SvgIcon<Msg> {
    pub fn new(draw: impl IntoIterator<Item = El<Msg>>) -> Self {
        Self {
            draw: draw.into_iter().collect(),
            color: None,
            size: Size::default(),
        }
    }
    pub fn draw(mut self, draw: impl IntoIterator<Item = El<Msg>>) -> Self {
        self.draw = draw.into_iter().collect();
        self
    }

    builder_functions! {
        color(Color),
    }

    composition_functions! {
        size: Size,
    }
}

impl<Msg: 'static + Clone> View<Msg> for SvgIcon<Msg> {
    fn view(&self, _: &impl Theme) -> Node<Msg> {
        let mut style = style![
            St::Color => self.color,
        ];
        style.merge((&self.size).into());

        svg![
            attrs![
                At::ViewBox => "0 0 100 100",
            ],
            self.draw.clone(),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct HtmlIcon {
    html: Cow<'static, str>,
    color: Option<Color>,
    size: Size,
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

    builder_functions! {
        color(Color),
    }

    composition_functions! {
        size: Size,
    }
}

impl<Msg: 'static + Clone> View<Msg> for HtmlIcon {
    fn view(&self, _theme: &impl Theme) -> Node<Msg> {
        let mut style = style![
            St::Color => self.color,
        ];
        style.merge((&self.size).into());

        svg![
            attrs![
                At::ViewBox => "0 0 100 100",
            ],
            raw![self.html.as_ref()],
        ]
    }
}

#[derive(Clone, Debug)]
pub struct UrlIcon {
    url: Cow<'static, str>,
    size: Size,
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

    composition_functions! {
        size: Size,
    }
}

impl<Msg: 'static + Clone> View<Msg> for UrlIcon {
    fn view(&self, _theme: &impl Theme) -> Node<Msg> {
        img![
            Style::from(&self.size),
            attrs![
                At::Src => self.url,
            ]
        ]
    }
}
