use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Clone, Element, Rich)]
#[element(style(svg_icon))]
pub struct Svg<PMsg> {
    #[rich(write)]
    pub id: Option<Id>,
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
            styler: None,
            theme: Theme::default(),
            view_box: None,
            draw: draw.into_iter().collect(),
        }
    }
}

pub type ThemeStyler<'a> = Styler<SvgLens<'a>, Style>;

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

impl<PMsg> View<Node<PMsg>> for Svg<PMsg> {
    fn view(&self) -> Node<PMsg> {
        self.styled_view(self.style())
    }
}

impl<PMsg> StyledView<Node<PMsg>> for Svg<PMsg> {
    fn styled_view(&self, style: Self::Style) -> Node<PMsg> {
        html::svg()
            .try_id(self.id.clone())
            .class("svg-icon")
            .try_set(self.view_box)
            .set(style.svg_icon)
            .add(self.draw.clone())
    }
}
