use crate::{header_bar::HeaderBarLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

#[derive(Rich, Element)]
#[element(style(dialog, dialog_background), events(dialog, dialog_background))]
pub struct Dialog<PMsg, C> {
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
    styler: Option<<Dialog<PMsg, C> as Stylable>::Styler>,
    #[rich(read)]
    #[element(theme_lens, config(default))]
    theme: Theme,

    // dialog element properties
    #[rich(read)]
    #[element(theme_lens(nested), config(default))]
    header_bar: HeaderBar<Msg>,
    #[element(config(required))]
    pub child: C,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, config(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_mouse_on_widget))]
    #[element(theme_lens)]
    mouse_on_dialog: bool,
    #[element(
        theme_lens,
        config(nested, default = "Toggle::config(Msg::Toggle).close_after(400)")
    )]
    toggle: Toggle<Msg>,
}

pub enum Msg {
    // EventsStore<Events<PMsg>>
    EventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    UpdateEventsStore(Rc<dyn Any>),
    // Option<<Self as Stylable>::Styler>
    Styler(Rc<dyn Any>),
    // Box<dyn Fn(<Self as Stylable>::Styler) -> <Self as Stylable>::Styler>
    UpdateStyler(Rc<dyn Any>),
    Theme(Theme),
    Title(Option<Label<Msg>>),
    Subtitle(Option<Label<Msg>>),
    MouseOnDialog(bool),
    ClickedOutSide,
    Toggle(toggle::Msg),
    CloseButton(button::Msg),
}

impl<PMsg, C> Element<PMsg> for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: 'static,
{
    type Message = Msg;

    fn init(config: Self::Config, porders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = porders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let local_events = || {
            events()
                .and_dialog_background(|conf| conf.click(|_| Msg::clicked_out_side()))
                .and_dialog(|conf| {
                    conf.mouse_enter(|_| Msg::mouse_on_dialog(true))
                        .mouse_leave(|_| Msg::mouse_on_dialog(false))
                })
        };

        let header_bar = config.header_bar.close_button(
            Button::config(Msg::close_button)
                // FIXME: use icon insted of label
                .label("X")
                .events(|| button::events().and_button(|conf| conf.click(|_| Msg::close())))
                .init(&mut orders),
        );

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            msg_mapper: config.msg_mapper,
            local_events: local_events.into(),
            events: config.events,
            styler: config.styler,
            theme: config.theme,
            header_bar,
            child: config.child,
            disabled: config.disabled,
            mouse_on_dialog: false,
            toggle: config.toggle.init(&mut orders),
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
                if let Ok(val) = val.downcast::<Box<dyn Fn(<Self as Stylable>::Styler) -> <Self as Stylable>::Styler>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Title(val) => self.header_bar.title = val,
            Msg::Subtitle(val) => self.header_bar.subtitle = val,
            Msg::MouseOnDialog(val) => self.mouse_on_dialog = val,
            Msg::ClickedOutSide => {
                if !self.mouse_on_dialog {
                    self.toggle.toggled(false, &mut orders);
                    // self.set_toggled(false, &mut orders);
                }
            }
            Msg::Toggle(msg) => self.toggle.update(msg, &mut orders),
            Msg::CloseButton(msg) => {
                if let Some(ref mut btn) = self.header_bar.close_button {
                    btn.update(msg, &mut orders)
                }
            }
        }
    }
}

impl<PMsg, C> Stylable for Dialog<PMsg, C> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.dialog().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg, C> View for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

pub type ThemeStyler<'a> = Styler<DialogLens<'a>, Style>;

impl<PMsg, C> StyledView for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    fn styled_view(&self, style: Style) -> Self::Output {
        let local_events = self.local_events.get();
        let events = self.events.get();

        let dialog = html::div()
            .class("dialog")
            .set(style.dialog)
            .set(&local_events.dialog)
            .map_msg_with(&self.msg_mapper)
            .add(self.header_bar.view().map_msg_with(&self.msg_mapper))
            .add(self.child.view())
            .add(&events.dialog);

        html::div()
            .id(self.id.clone())
            .class("dialog-background")
            .set(style.dialog_background)
            .set(&local_events.dialog_background)
            .map_msg_with(&self.msg_mapper)
            .add(dialog)
            .add(&events.dialog_background)
    }
}

impl<PMsg, C> Config<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>> + 'static,
{
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Dialog<PMsg, C> {
        Dialog::init(self, orders)
    }

    pub fn title(mut self, val: impl Into<Label<Msg>>) -> Self {
        self.header_bar.title = Some(val.into());
        self
    }

    pub fn subtitle(mut self, val: impl Into<Label<Msg>>) -> Self {
        self.header_bar.subtitle = Some(val.into());
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
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

    pub fn styler<PMsg, C>(val: <Dialog<PMsg, C> as Stylable>::Styler) -> Self
    where
        PMsg: 'static,
        C: View<Output = Node<PMsg>> + 'static,
    {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg, C, F>(val: F) -> Self
    where
        PMsg: 'static,
        C: View<Output = Node<PMsg>> + 'static,
        F: Fn(<Dialog<PMsg, C> as Stylable>::Styler) -> <Dialog<PMsg, C> as Stylable>::Styler
            + 'static,
    {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg, C>(val: Option<<Dialog<PMsg, C> as Stylable>::Styler>) -> Self
    where
        PMsg: 'static,
        C: View<Output = Node<PMsg>> + 'static,
    {
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

    pub fn try_title(val: Option<impl Into<Label<Msg>>>) -> Self {
        Msg::Title(val.map(|val| val.into()))
    }

    pub fn title(val: impl Into<Label<Msg>>) -> Self {
        Msg::try_title(Some(val))
    }

    pub fn try_subtitle(val: Option<impl Into<Label<Msg>>>) -> Self {
        Msg::Subtitle(val.map(|val| val.into()))
    }

    pub fn subtitle(val: impl Into<Label<Msg>>) -> Self {
        Msg::try_subtitle(Some(val))
    }

    pub fn mouse_on_dialog(val: bool) -> Self {
        Msg::MouseOnDialog(val)
    }

    pub fn clicked_out_side() -> Self {
        Msg::ClickedOutSide
    }

    pub fn close_button(val: button::Msg) -> Self {
        Msg::CloseButton(val)
    }
}
