use crate::{button::ButtonLens, label::LabelLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(
    style(
        header_bar,
        titles_container,
        title(label::Style),
        subtitle(label::Style),
        close_button(button::Style),
    ),
    events(titles_container, header_bar)
)]
pub struct HeaderBar<PMsg> {
    // general element properties
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write)]
    pub styler: Option<Styler<Self, Style>>,
    #[rich(write)]
    #[element(theme_lens)]
    pub theme: Theme,

    // dialog element properties
    #[rich(write)]
    #[element(theme_lens(nested))]
    pub title: Option<Label<PMsg>>,
    #[rich(write)]
    #[element(theme_lens(nested))]
    pub subtitle: Option<Label<PMsg>>,
    #[rich(write)]
    #[element(theme_lens(nested))]
    pub close_button: Option<Button<PMsg>>,
    #[rich(read(rename = is_hidden), write(rename = hidden))]
    #[element(theme_lens)]
    pub hidden: bool,
}

impl<PMsg> Default for HeaderBar<PMsg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<PMsg> Stylable for HeaderBar<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.header_bar().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg: 'static> View for HeaderBar<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

impl<PMsg: 'static> StyledView for HeaderBar<PMsg> {
    fn styled_view(&self, style: Style) -> Self::Output {
        let Style {
            header_bar,
            titles_container,
            title,
            subtitle,
            close_button,
        } = style;

        let title = self.title.as_ref().map(|t| t.styled_view(title));
        let subtitle = self.subtitle.as_ref().map(|s| s.styled_view(subtitle));

        let titles = html::div()
            .class("titles-container")
            .set(titles_container)
            .set(&self.events.titles_container)
            .try_add(title)
            .try_add(subtitle);

        let close_button = self
            .close_button
            .as_ref()
            .map(|btn| btn.styled_view(close_button));

        html::div()
            .try_id(self.id.clone())
            .class("header-bar")
            .set(header_bar)
            .set(&self.events.header_bar)
            .try_add(close_button)
            .add(titles)
    }
}

impl<PMsg> HeaderBar<PMsg> {
    pub fn new() -> Self {
        Self {
            id: None,
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            title: None,
            subtitle: None,
            close_button: None,
            hidden: false,
        }
    }
}

pub type ThemeStyler<'a> = Styler<HeaderBarLens<'a>, Style>;
