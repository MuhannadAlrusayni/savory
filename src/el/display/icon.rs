use crate::prelude::*;
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Clone, From)]
pub enum Icon<PMsg> {
    #[from]
    Svg(SvgIcon<PMsg>),
    #[from]
    Html(HtmlIcon<PMsg>),
    #[from]
    Url(UrlIcon<PMsg>),
}

impl<PMsg> Render for Icon<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, _: &Theme) -> Style {
        Style::default()
    }

    fn render_with_style(&self, theme: &Theme, _: Style) -> Self::View {
        match self {
            Self::Svg(icon) => icon.render(theme),
            Self::Html(icon) => icon.render(theme),
            Self::Url(icon) => icon.render(theme),
        }
    }
}

impl<PMsg> Icon<PMsg> {
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

#[derive(Clone, Element, Rich)]
pub struct SvgIcon<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    view_box: Option<att::ViewBox>,
    #[rich(read, write)]
    draw: Vec<Node<PMsg>>,
}

impl<PMsg> SvgIcon<PMsg> {
    pub fn new(draw: impl IntoIterator<Item = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            user_style: Style::default(),
            view_box: None,
            draw: draw.into_iter().collect(),
        }
    }
}

impl<PMsg> Render for SvgIcon<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.svg_icon(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // svg!()
        //     .and_attributes(|conf| conf.set_class("svg-icon").try_set_view_box(self.view_box))
        //     .set(style["svg-icon"])
        //     .try_set_events(self.events.get("svg-icon"))
        //     .add(self.draw.clone())
    }
}

#[derive(Clone, Element, Rich)]
pub struct HtmlIcon<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    view_box: Option<att::ViewBox>,
    #[rich(read, write)]
    html: Cow<'static, str>,
}

impl<PMsg> HtmlIcon<PMsg> {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            user_style: Style::default(),
            view_box: None,
            html: html.into(),
        }
    }
}

impl<PMsg> Render for HtmlIcon<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.html_icon(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // svg!()
        //     .set(style["html-icon"])
        //     .and_attributes(|conf| conf.set_class("html-icon").try_set_view_box(self.view_box))
        //     .try_set_events(self.events.get("html-icon"))
        //     .add(raw![self.html.as_ref()])
    }
}

#[derive(Rich, Element, Clone)]
pub struct UrlIcon<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    #[rich(read, write)]
    url: Cow<'static, str>,
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
            user_style: Style::default(),
            url: url.into(),
        }
    }
}

impl<PMsg> Render for UrlIcon<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.url_icon(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // img!()
        //     .set(att::class("url-icon"))
        //     .set(att::src(self.url.clone()))
        //     .set(style["url-icon"])
        //     .try_set_events(self.events.get("url-icon"))
    }
}
