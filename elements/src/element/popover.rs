use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

// TODO: add placement property
#[derive(Clone, Rich, Element)]
#[element(style(panel, popover), events(panel, popover))]
pub struct Popover<PMsg, C, T> {
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    #[element(props(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg, C, T>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    #[element(props(required))]
    pub child: C,
    #[element(props(required))]
    pub target: T,
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens, props(default = "false"))]
    toggled: bool,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "0"))]
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
    Toggled(bool),
    Toggle,
    Offset(i8),
}

impl<PMsg, C, T> Element<PMsg> for Popover<PMsg, C, T>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>> + 'static,
    T: View<Output = Node<PMsg>> + 'static,
{
    type Message = Msg;
    type Props = Props<PMsg, C, T>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        Self {
            msg_mapper: props.msg_mapper,
            events: props.events,
            styler: props.styler,
            theme: props.theme,
            child: props.child,
            target: props.target,
            toggled: props.toggled,
            offset: props.offset,
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
                if let Ok(val) = val.downcast::<Option<Styler<PMsg, C, T>>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) =
                    val.downcast::<Box<dyn Fn(Styler<PMsg, C, T>) -> Styler<PMsg, C, T>>>()
                {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Toggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
            Msg::Offset(val) => self.offset = val,
        }
    }
}

impl<PMsg, C, T> View for Popover<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.popover()(&self.theme_lens())),
        )
    }
}

impl<PMsg, C, T> StyledView for Popover<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let events = self.events.get();

        let panel = html::div()
            .class("panel")
            .set(style.panel)
            .set(&events.panel)
            .add(self.child.view());

        html::div()
            .class("popover")
            .set(style.popover)
            .set(&events.popover)
            .add(self.target.view())
            .add(panel)
    }
}

impl<PMsg: 'static, C, T> Props<PMsg, C, T>
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

pub type Styler<PMsg, C, T> = theme::Styler<Popover<PMsg, C, T>, Style>;
pub type ThemeStyler<'a> = theme::Styler<PopoverLens<'a>, Style>;

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static, C: 'static, T: 'static>(val: Styler<PMsg, C, T>) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static, C: 'static, T: 'static>(
        val: impl Fn(Styler<PMsg, C, T>) -> Styler<PMsg, C, T> + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static, C: 'static, T: 'static>(
        val: Option<Styler<PMsg, C, T>>,
    ) -> Self {
        Msg::Styler(Rc::new(val))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn toggled(val: bool) -> Self {
        Msg::Toggled(val)
    }

    pub fn open() -> Self {
        Msg::toggled(true)
    }

    pub fn close() -> Self {
        Msg::toggled(false)
    }

    pub fn toggle() -> Self {
        Msg::Toggle
    }

    pub fn offset(val: i8) -> Self {
        Msg::Offset(val)
    }
}
