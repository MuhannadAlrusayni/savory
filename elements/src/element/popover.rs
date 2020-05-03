use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

// TODO: add placement property
#[derive(Clone, Rich, Element)]
#[element(style(panel, popover), events(panel, popover))]
pub struct Popover<PMsg, C, T> {
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[element(config(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    #[element(config(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Popover<PMsg, C, T> as Stylable>::Styler>,
    #[rich(read)]
    #[element(theme_lens, config(default))]
    theme: Theme,

    #[element(config(required))]
    pub child: C,
    #[element(config(required))]
    pub target: T,
    #[element(
        theme_lens,
        config(nested, default = "Toggle::config(Msg::Toggle).close_after(400)")
    )]
    toggle: Toggle<Msg>,
    #[rich(read(copy))]
    #[element(theme_lens, config(default = "0"))]
    offset: i8,
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
    Toggle(toggle::Msg),
    Offset(i8),
}

impl<PMsg, C, T> Element<PMsg> for Popover<PMsg, C, T>
where
    PMsg: 'static,
    C: 'static,
    T: 'static,
{
    type Message = Msg;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            msg_mapper: config.msg_mapper,
            events: config.events,
            styler: config.styler,
            theme: config.theme,
            child: config.child,
            target: config.target,
            toggle: config.toggle.init(&mut orders),
            offset: config.offset,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

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
                if let Ok(val) =
                    val.downcast::<Box<dyn Fn(<Self as Stylable>::Styler) -> <Self as Stylable>::Styler>>()
                {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Offset(val) => self.offset = val,
            Msg::Toggle(msg) => self.toggle.update(msg, &mut orders),
        }
    }
}

impl<PMsg, C, T> Stylable for Popover<PMsg, C, T> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.popover().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg, C, T> View for Popover<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

pub type ThemeStyler<'a> = Styler<PopoverLens<'a>, Style>;

impl<PMsg, C, T> StyledView for Popover<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    fn styled_view(&self, style: Style) -> Self::Output {
        let events = self.events.get();

        let panel = html::div()
            .class("panel")
            .set(style.panel)
            .set(&events.panel)
            .add(self.child.view());

        html::div()
            .id(self.id.clone())
            .class("popover")
            .set(style.popover)
            .set(&events.popover)
            .add(self.target.view().add(att::class("popover-target")))
            .add(panel)
    }
}

impl<PMsg, C, T> Popover<PMsg, C, T> {
    pub fn is_toggled(&self) -> bool {
        self.toggle.is_toggled()
    }
}

impl<PMsg: 'static, C, T> Config<PMsg, C, T>
where
    C: View<Output = Node<PMsg>> + 'static,
    T: View<Output = Node<PMsg>> + 'static,
{
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Popover<PMsg, C, T> {
        Popover::init(self, orders)
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

    pub fn styler<PMsg: 'static, C: 'static, T: 'static>(
        val: <Popover<PMsg, C, T> as Stylable>::Styler,
    ) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static, C: 'static, T: 'static>(
        val: impl Fn(
                <Popover<PMsg, C, T> as Stylable>::Styler,
            ) -> <Popover<PMsg, C, T> as Stylable>::Styler
            + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static, C: 'static, T: 'static>(
        val: Option<<Popover<PMsg, C, T> as Stylable>::Styler>,
    ) -> Self {
        Msg::Styler(Rc::new(val))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn toggled(val: bool) -> Self {
        Msg::Toggle(toggle::Msg::Toggled(val))
    }

    pub fn open() -> Self {
        Msg::toggled(true)
    }

    pub fn close() -> Self {
        Msg::toggled(false)
    }

    pub fn toggle() -> Self {
        Msg::Toggle(toggle::Msg::Toggle)
    }

    pub fn offset(val: i8) -> Self {
        Msg::Offset(val)
    }
}
