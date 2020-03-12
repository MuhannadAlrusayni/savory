use crate::{
    css::{self, color::Color, size::Size},
    prelude::*,
};
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Clone, From)]
pub enum Icon<PMsg: 'static> {
    #[from]
    Svg(SvgIcon<PMsg>),
    #[from]
    Html(HtmlIcon<PMsg>),
    #[from]
    Url(UrlIcon<PMsg>),
}

impl<PMsg: 'static> Render<PMsg> for Icon<PMsg> {
    type View = Node<PMsg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, theme: &impl Theme, _: Self::Style) -> Self::View {
        match self {
            Self::Svg(icon) => icon.render(theme),
            Self::Html(icon) => icon.render(theme),
            Self::Url(icon) => icon.render(theme),
        }
    }
}

impl<PMsg: 'static> Icon<PMsg> {
    pub fn svg(draw: impl IntoIterator<Item = Node<PMsg>>) -> SvgIcon<PMsg> {
        SvgIcon::new(draw)
    }

    pub fn html(html: impl Into<Cow<'static, str>>) -> HtmlIcon<PMsg> {
        HtmlIcon::new(html)
    }

    pub fn url(url: impl Into<Cow<'static, str>>) -> UrlIcon<PMsg> {
        UrlIcon::new(url)
    }
}

#[derive(Clone, Rich)]
pub struct SvgIcon<PMsg: 'static> {
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    #[rich(write, read)]
    view_box: Option<att::ViewBox>,
    pub draw: Vec<Node<PMsg>>,
    #[rich(write)]
    pub color: Option<Color>,
    #[rich(write(style = compose))]
    pub size: Size,
    #[rich(write(style = compose))]
    pub style: SvgStyle,
}

impl<PMsg: 'static> SvgIcon<PMsg> {
    pub fn new(draw: impl IntoIterator<Item = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            view_box: None,
            draw: draw.into_iter().collect(),
            color: None,
            size: Size::default(),
            style: SvgStyle::default(),
        }
    }

    pub fn draw(&mut self, draw: impl IntoIterator<Item = Node<PMsg>>) -> &mut Self {
        self.draw = draw.into_iter().collect();
        self
    }
}

pub type SvgStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for SvgIcon<PMsg> {
    type View = Node<PMsg>;
    type Style = SvgStyle;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.svg_icon(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        svg![
            style,
            self.events.events.clone(),
            self.view_box,
            // att::try_att(self.view_box),
            self.draw.clone(),
        ]
    }
}

#[derive(Clone, Rich)]
pub struct HtmlIcon<PMsg> {
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    #[rich(write, read)]
    view_box: Option<att::ViewBox>,
    pub html: Cow<'static, str>,
    #[rich(write)]
    pub color: Option<Color>,
    #[rich(write(style = compose))]
    pub size: Size,
    #[rich(write(style = compose))]
    pub style: HtmlStyle,
}

impl<PMsg> HtmlIcon<PMsg> {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            view_box: None,
            html: html.into(),
            color: None,
            size: Size::default(),
            style: HtmlStyle::default(),
        }
    }

    pub fn html(&mut self, html: impl Into<Cow<'static, str>>) -> &mut Self {
        self.html = html.into();
        self
    }
}

pub type HtmlStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for HtmlIcon<PMsg> {
    type View = Node<PMsg>;
    type Style = HtmlStyle;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.html_icon(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        svg![
            style,
            self.events.events.clone(),
            self.view_box,
            // att::try_att(self.view_box),
            raw![self.html.as_ref()],
        ]
    }
}

#[derive(Rich, Clone)]
pub struct UrlIcon<PMsg> {
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    pub url: Cow<'static, str>,
    #[rich(write(style = compose))]
    pub size: Size,
    #[rich(write(style = compose))]
    pub style: UrlStyle,
}

impl<PMsg, T: ToString> From<T> for UrlIcon<PMsg> {
    fn from(url: T) -> Self {
        Self::new(url.to_string())
    }
}

impl<PMsg> UrlIcon<PMsg> {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            url: url.into(),
            size: Size::default(),
            style: UrlStyle::default(),
        }
    }

    pub fn url(&mut self, url: impl Into<Cow<'static, str>>) -> &mut Self {
        self.url = url.into();
        self
    }
}

pub type UrlStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for UrlIcon<PMsg> {
    type View = Node<PMsg>;
    type Style = UrlStyle;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.url_icon(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        img![
            style,
            self.events.events.clone(),
            att::src(self.url.clone()),
        ]
    }
}
