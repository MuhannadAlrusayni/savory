use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(style(button, switch), events(button, switch))]
pub struct Switch {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[rich(read)]
    events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Switch as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // switch element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(config(default = "false"))]
    toggled: bool,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default = "false"))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
}

pub enum Msg {
    Styler(Option<<Switch as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Switch>),
    Theme(Theme),
    Toggled(bool),
    Toggle,
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl Element for Switch {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let events = || {
            Events::default().and_switch(|conf| {
                conf.focus(|_| Msg::focus(true))
                    .blur(|_| Msg::focus(false))
                    .mouse_enter(|_| Msg::mouse_over(true))
                    .mouse_leave(|_| Msg::mouse_over(false))
                    .click(|_| Msg::toggle())
            })
        };

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            theme: config.theme,
            events: events.into(),
            styler: config.styler,
            disabled: config.disabled,
            toggled: config.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.switch())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Toggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
        }
    }
}

impl Stylable for Switch {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.switch().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Switch {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Switch {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let events = self.events.get();

        let button = html::div()
            .class("button")
            .set(style.button)
            .set(&events.button);

        html::button()
            .id(self.id.clone())
            .class("switch")
            .set(att::disabled(self.disabled))
            .set(style.switch)
            .set(&events.switch)
            .add(button)
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Switch {
        Switch::init(self, orders)
    }
}

impl Msg {
    pub fn styler(val: <Switch as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Switch>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Switch as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|v| v.into()))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn toggled(val: bool) -> Self {
        Msg::Toggled(val)
    }

    pub fn toggle_ond() -> Self {
        Msg::toggled(true)
    }

    pub fn toggle_off() -> Self {
        Msg::toggled(false)
    }

    pub fn toggle() -> Self {
        Msg::Toggle
    }

    pub fn disabled(val: bool) -> Self {
        Msg::Disabled(val)
    }

    pub fn disable() -> Self {
        Self::disabled(true)
    }

    pub fn focus(val: bool) -> Self {
        Msg::Focus(val)
    }

    pub fn mouse_over(val: bool) -> Self {
        Msg::MouseOver(val)
    }
}
