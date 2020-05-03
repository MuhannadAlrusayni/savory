use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Clone, Element, Rich)]
#[element(style(svg_icon), events(svg_icon))]
pub struct Svg<PMsg> {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write)]
    pub styler: Option<Styler<Self, Style>>,
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
            id: None,
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            view_box: None,
            draw: draw.into_iter().collect(),
        }
    }
}

impl<PMsg> Stylable for Svg<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.svg_icon().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg> View for Svg<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

impl<PMsg> StyledView for Svg<PMsg> {
    fn styled_view(&self, style: Self::Style) -> Self::Output {
        html::svg()
            .try_id(self.id.clone())
            .class("svg-icon")
            .try_set(self.view_box)
            .set(style.svg_icon)
            .set(&self.events.svg_icon)
            .add(self.draw.clone())
    }
}

pub type ThemeStyler<'a> = Styler<SvgLens<'a>, Style>;
