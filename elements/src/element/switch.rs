use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

#[derive(Rich, Element)]
#[element(style(button, switch), events(button, switch))]
pub struct Switch<PMsg> {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[element(config(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Switch<PMsg> as Stylable>::Styler>,
    #[rich(read)]
    #[element(theme_lens, config(default))]
    theme: Theme,

    // switch element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens, config(default = "false"))]
    toggled: bool,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, config(default = "false"))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,
}

pub enum Msg {
    // EventsStore<Events<PMsg>>
    EventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    UpdateEventsStore(Rc<dyn Any>),
    // Option<Styler<PMsg>>
    Styler(Rc<dyn Any>),
    // Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>
    UpdateStyler(Rc<dyn Any>),
    Theme(Theme),
    Toggled(bool),
    Toggle,
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl<PMsg: 'static> Element<PMsg> for Switch<PMsg> {
    type Message = Msg;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let local_events = || {
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
            msg_mapper: config.msg_mapper,
            local_events: local_events.into(),
            events: config.events,
            styler: config.styler,
            disabled: config.disabled,
            toggled: config.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg>) {
        match msg {
            Msg::EventsStore(val) => {
                if let Ok(val) = val.downcast::<EventsStore<Events<PMsg>>>() {
                    self.events = val.into();
                }
            }
            Msg::UpdateEventsStore(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>>() {
                    self.events = val(self.events.clone());
                }
            }
            Msg::Styler(val) => {
                if let Ok(val) = val.downcast::<Option<<Self as Stylable>::Styler>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(<Self as Stylable>::Styler) -> <Self as Stylable>::Styler>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
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

impl<PMsg> Stylable for Switch<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.switch().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg: 'static> View for Switch<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

pub type ThemeStyler<'a> = Styler<SwitchLens<'a>, Style>;

impl<PMsg: 'static> StyledView for Switch<PMsg> {
    fn styled_view(&self, style: Style) -> Self::Output {
        let events = self.events.get();
        let local_events = self.local_events.get();

        let button = html::div()
            .class("button")
            .set(style.button)
            .set(&local_events.button)
            .map_msg_with(&self.msg_mapper)
            .add(&events.button);

        html::button()
            .id(self.id.clone())
            .class("switch")
            .set(att::disabled(self.disabled))
            .set(style.switch)
            .set(&local_events.switch)
            .map_msg_with(&self.msg_mapper)
            .add(&events.switch)
            .add(button)
    }
}

impl<PMsg: 'static> Config<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Switch<PMsg> {
        Switch::init(self, orders)
    }
}

pub fn events<PMsg>() -> Events<PMsg> {
    Events::default()
}

pub fn style() -> Style {
    Style::default()
}

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static>(val: <Switch<PMsg> as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static>(
        val: impl Fn(<Switch<PMsg> as Stylable>::Styler) -> <Switch<PMsg> as Stylable>::Styler + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static>(val: Option<<Switch<PMsg> as Stylable>::Styler>) -> Self {
        Msg::Styler(Rc::new(val))
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
