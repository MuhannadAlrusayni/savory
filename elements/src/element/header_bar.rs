use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(style(
    header_bar,
    titles_container,
    title(label::Style),
    subtitle(label::Style),
    close_button(button::Style),
))]
pub struct HeaderBar {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[rich(read)]
    #[element(config)]
    styler: Option<<HeaderBar as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // dialog element properties
    #[rich(read)]
    #[element(config)]
    title: Option<Label>,
    #[rich(read)]
    #[element(config)]
    subtitle: Option<Label>,
    #[rich(read)]
    // FIXME: use icons once #5 implemented
    #[element(config(nested, default = "Button::config().label(\"X\")"))]
    close_button: Option<Button>,
    #[rich(read(rename = is_hidden))]
    #[element(config(default = "false"))]
    hidden: bool,
}

impl Msg {
    pub fn styler(val: <HeaderBar as Stylable>::Styler) -> Msg {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<HeaderBar>>) -> Msg {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<HeaderBar as Stylable>::Styler>>) -> Msg {
        Msg::Styler(val.map(|v| v.into()))
    }

    pub fn theme(val: Theme) -> Msg {
        Msg::Theme(val)
    }

    pub fn title(val: impl Into<Label>) -> Msg {
        Msg::try_title(Some(val))
    }

    pub fn try_title(val: Option<impl Into<Label>>) -> Msg {
        Msg::Title(val.map(|t| t.into()))
    }

    pub fn subtitle(val: impl Into<Label>) -> Msg {
        Msg::try_subtitle(Some(val))
    }

    pub fn try_subtitle(val: Option<impl Into<Label>>) -> Msg {
        Msg::Subtitle(val.map(|t| t.into()))
    }

    pub fn hidden(val: bool) -> Msg {
        Msg::Hidden(val)
    }

    pub fn hide() -> Msg {
        Msg::hidden(true)
    }

    pub fn show() -> Msg {
        Msg::hidden(false)
    }

    pub fn close_button(val: button::Msg) -> Msg {
        Msg::CloseButton(val)
    }
}

pub enum Msg {
    Styler(Option<<HeaderBar as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<HeaderBar>),
    Theme(Theme),
    Title(Option<Label>),
    Subtitle(Option<Label>),
    Hidden(bool),
    CloseButton(button::Msg),
}

impl Element for HeaderBar {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::Theme(theme.0));

        HeaderBar {
            id: config.id.unwrap_or_else(Id::generate),
            theme: config.theme,
            styler: config.styler,
            title: config.title,
            subtitle: config.subtitle,
            close_button: Some(
                config
                    .close_button
                    .init(&mut orders.proxy(Msg::CloseButton)),
            ),
            hidden: config.hidden,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.header_bar())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Title(val) => self.title = val,
            Msg::Subtitle(val) => self.subtitle = val,
            Msg::Hidden(val) => self.hidden = val,
            Msg::CloseButton(msg) => {
                if let Some(ref mut btn) = self.close_button {
                    btn.update(msg, &mut orders.proxy(Msg::CloseButton))
                }
            }
        }
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> HeaderBar {
        HeaderBar::init(self, orders)
    }
}

impl Stylable for HeaderBar {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.header_bar().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for HeaderBar {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for HeaderBar {
    fn styled_view(&self, style: Style) -> Node<Msg> {
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
            .try_add(title)
            .try_add(subtitle);

        let close_button = self
            .close_button
            .as_ref()
            .map(|btn| btn.styled_view(close_button).map_msg(Msg::CloseButton));

        html::div()
            .id(self.id.clone())
            .class("header-bar")
            .set(header_bar)
            .try_add(close_button)
            .add(titles)
    }
}
