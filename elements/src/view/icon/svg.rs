use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Clone, Element, Rich)]
#[element(style(svg_icon), events(svg_icon))]
pub struct Svg<PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write)]
    pub styler: Option<Styler<PMsg>>,
    #[rich(write)]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write)]
    #[element(theme_lens)]
    pub view_box: Option<att::ViewBox>,
    #[rich(write)]
    pub draw: Vec<Node<PMsg>>,
}

impl<PMsg> Svg<PMsg> {
    pub fn new(draw: impl IntoIterator<Item = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            view_box: None,
            draw: draw.into_iter().collect(),
        }
    }
}

impl<PMsg> View for Svg<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(&self))
                .unwrap_or_else(|| self.theme.svg_icon()(&self.theme_lens())),
        )
    }
}

impl<PMsg> StyledView for Svg<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Self::Style) -> Self::Output {
        html::svg()
            .class("svg-icon")
            .try_set(self.view_box)
            .set(style.svg_icon)
            .set(&self.events.svg_icon)
            .add(self.draw.clone())
    }
}

pub type Styler<PMsg> = theme::Styler<Svg<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<SvgLens<'a>, Style>;
