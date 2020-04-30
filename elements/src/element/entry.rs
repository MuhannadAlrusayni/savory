use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, borrow::Cow, rc::Rc};

#[derive(Rich, Element)]
#[element(style(input, container), events(input, container))]
pub struct Entry<PMsg> {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    el_ref: ElRef<web_sys::HtmlInputElement>,
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

    // entry element properties
    #[rich(read)]
    #[element(config)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy))]
    #[element(theme_lens, config)]
    max_length: Option<att::MaxLength>,
    #[rich(read)]
    #[element(theme_lens, config)]
    placeholder: Option<Cow<'static, str>>,
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
    Text(Option<Cow<'static, str>>),
    MaxLength(Option<att::MaxLength>),
    Placeholder(Option<Cow<'static, str>>),
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
    ResyncText,
}

impl<PMsg: 'static> Element<PMsg> for Entry<PMsg> {
    type Message = Msg;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let local_events = || {
            Events::default().and_input(|conf| {
                conf.focus(|_| Msg::focus(true))
                    .blur(|_| Msg::focus(false))
                    .mouse_enter(|_| Msg::mouse_over(true))
                    .mouse_leave(|_| Msg::mouse_over(false))
                    .input(|_| Msg::resync_text())
            })
        };

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            el_ref: ElRef::default(),
            msg_mapper: config.msg_mapper,
            local_events: local_events.into(),
            events: config.events,
            styler: config.styler,
            theme: config.theme,
            text: config.text,
            max_length: config.max_length,
            placeholder: config.placeholder,
            disabled: config.disabled,
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
            Msg::Text(val) => self.text = val,
            Msg::MaxLength(val) => self.max_length = val,
            Msg::Placeholder(val) => self.placeholder = val,
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::ResyncText => {
                if let Some(input) = self.el_ref.get() {
                    self.text = Some(input.value().into());
                }
            }
        }
    }
}

impl<PMsg: 'static> View for Entry<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.entry().get(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Entry<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let local_events = self.local_events.get();
        let events = self.events.get();

        let input = html::input()
            .set(&local_events.input)
            .set(style.input)
            .and_attributes(|conf| {
                conf.class("input")
                    .disabled(self.disabled)
                    .try_value(self.text.clone())
                    .try_max_length(self.max_length)
                    .try_placeholder(self.placeholder.clone())
            })
            .map_msg_with(&self.msg_mapper)
            .add(&events.input);

        html::div()
            .id(self.id.clone())
            .set(style.container)
            .set(&local_events.container)
            .map_msg_with(&self.msg_mapper)
            .add(&events.container)
            .add(input)
    }
}

impl<PMsg: 'static> Config<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Entry<PMsg> {
        Entry::init(self, orders)
    }
}

pub fn events<PMsg>() -> Events<PMsg> {
    Events::default()
}

pub fn style() -> Style {
    Style::default()
}

pub type Styler<PMsg> = theme::Styler<Entry<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<EntryLens<'a>, Style>;

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

    pub fn try_text(val: Option<Cow<'static, str>>) -> Self {
        Msg::Text(val)
    }

    pub fn text(val: Cow<'static, str>) -> Self {
        Msg::try_text(Some(val))
    }

    pub fn try_max_length(val: Option<impl Into<att::MaxLength>>) -> Self {
        Msg::MaxLength(val.map(|val| val.into()))
    }

    pub fn max_length(val: impl Into<att::MaxLength>) -> Self {
        Msg::try_max_length(Some(val))
    }

    pub fn try_placeholder(val: Option<Cow<'static, str>>) -> Self {
        Msg::Placeholder(val)
    }

    pub fn placeholder(val: Cow<'static, str>) -> Self {
        Msg::try_placeholder(Some(val))
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

    fn resync_text() -> Self {
        Msg::ResyncText
    }
}
