use crate::{css, prelude::*};
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
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: SvgUserStyle,
    #[rich(read(copy), write)]
    view_box: Option<att::ViewBox>,
    #[rich(read, write)]
    draw: Vec<Node<PMsg>>,
}

impl<PMsg: 'static> SvgIcon<PMsg> {
    pub fn new(draw: impl IntoIterator<Item = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            user_style: SvgUserStyle::default(),
            view_box: None,
            draw: draw.into_iter().collect(),
        }
    }
}

pub type SvgUserStyle = css::Style;
pub type SvgStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for SvgIcon<PMsg> {
    type View = Node<PMsg>;
    type Style = SvgStyle;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.svg_icon(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        svg!()
            .set_style(style)
            .set_events(&self.events)
            .and_attributes(|conf| conf.try_set_view_box(self.view_box))
            .add_children(self.draw.clone())
    }
}

#[derive(Clone, Rich)]
pub struct HtmlIcon<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: HtmlUserStyle,
    #[rich(read(copy), write)]
    view_box: Option<att::ViewBox>,
    #[rich(read, write)]
    html: Cow<'static, str>,
}

impl<PMsg> HtmlIcon<PMsg> {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            user_style: HtmlUserStyle::default(),
            view_box: None,
            html: html.into(),
        }
    }
}

pub type HtmlUserStyle = css::Style;
pub type HtmlStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for HtmlIcon<PMsg> {
    type View = Node<PMsg>;
    type Style = HtmlStyle;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.html_icon(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        svg!()
            .set_style(style)
            .and_attributes(|conf| conf.try_set_view_box(self.view_box))
            .set_events(&self.events)
            .add_children(raw![self.html.as_ref()])
    }
}

#[derive(Rich, Clone)]
pub struct UrlIcon<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UrlUserStyle,
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
            user_style: UrlUserStyle::default(),
            url: url.into(),
        }
    }
}

pub type UrlUserStyle = css::Style;
pub type UrlStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for UrlIcon<PMsg> {
    type View = Node<PMsg>;
    type Style = UrlStyle;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.url_icon(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        img!()
            .set_style(style)
            .set_events(&self.events)
            .and_attributes(|conf| conf.set_src(self.url.clone()))
    }
}
