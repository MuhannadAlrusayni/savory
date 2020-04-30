use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, borrow::Cow, rc::Rc};

#[derive(Rich, Element)]
#[element(style(checkbox, button, label), events(checkbox, button, label))]
pub struct Checkbox<PMsg> {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
    #[element(config(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, config(default))]
    theme: Theme,

    // checkbox element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens, config(default))]
    toggled: bool,
    #[rich(read)]
    #[element(theme_lens, config)]
    label: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, config(default))]
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
    Label(Option<Cow<'static, str>>),
    Toggled(bool),
    Toggle,
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl<PMsg: 'static> Element<PMsg> for Checkbox<PMsg> {
    type Message = Msg;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let local_events = || {
            Events::default()
                .and_checkbox(|conf| {
                    conf.focus(|_| Msg::focus(true))
                        .blur(|_| Msg::focus(false))
                        .mouse_enter(|_| Msg::mouse_over(true))
                        .mouse_leave(|_| Msg::mouse_over(false))
                        .click(|_| Msg::toggle())
                })
                .and_label(|conf| {
                    conf.mouse_enter(|_| Msg::mouse_over(true))
                        .mouse_leave(|_| Msg::mouse_over(false))
                })
        };

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            input_el_ref: ElRef::default(),
            label_el_ref: ElRef::default(),
            theme: config.theme,
            msg_mapper: config.msg_mapper,
            local_events: local_events.into(),
            events: config.events,
            label: config.label,
            styler: config.styler,
            disabled: config.disabled,
            toggled: config.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _orders: &mut impl Orders<PMsg>) {
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
                if let Ok(val) = val.downcast::<Option<Styler<PMsg>>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Label(val) => self.label = val,
            Msg::Toggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
        }
    }
}

impl<PMsg: 'static> View for Checkbox<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.checkbox().get(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Checkbox<PMsg> {
    type Style = Style;

    // TODO: use container block and assign the element id for it
    fn styled_view(&self, style: Style) -> Self::Output {
        let Style {
            checkbox,
            button,
            label,
        } = style;
        let events = self.events.get();
        let local_events = self.local_events.get();

        let checkbox = html::input()
            .class("checbox")
            .set(att::disabled(self.disabled))
            .set(att::checked(self.toggled))
            .set(att::Type::Checkbox)
            .set(checkbox)
            .set(&local_events.checkbox)
            .map_msg_with(&self.msg_mapper)
            .add(&events.checkbox)
            .el_ref(&self.input_el_ref)
            // add button if the checkbox is toggled
            .config_if(self.is_toggled(), |conf| {
                let button = html::div()
                    .class("button")
                    .set(button)
                    .map_msg_with(&self.msg_mapper)
                    .add(&events.button);
                conf.add(button)
            });

        match self.label.as_ref() {
            None => checkbox.id(self.id.clone()),
            Some(lbl) => html::label()
                .id(self.id.clone())
                .class("label")
                .set(label)
                .set(&local_events.label)
                .map_msg_with(&self.msg_mapper)
                .add(&events.label)
                .add(checkbox)
                .add(lbl.clone())
                .el_ref(&self.label_el_ref),
        }
    }
}

impl<PMsg: 'static> Config<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Checkbox<PMsg> {
        Checkbox::init(self, orders)
    }
}

pub fn events<PMsg>() -> Events<PMsg> {
    Events::default()
}

pub fn style() -> Style {
    Style::default()
}

pub type Styler<PMsg> = theme::Styler<Checkbox<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<CheckboxLens<'a>, Style>;

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static>(val: Styler<PMsg>) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static>(
        val: impl Fn(Styler<PMsg>) -> Styler<PMsg> + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static>(val: Option<Styler<PMsg>>) -> Self {
        Msg::Styler(Rc::new(val))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn label(val: Cow<'static, str>) -> Self {
        Msg::try_label(Some(val))
    }

    pub fn try_label(val: Option<Cow<'static, str>>) -> Self {
        Msg::Label(val)
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
