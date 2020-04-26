use crate::{header_bar::HeaderBarLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

#[derive(Rich, Element)]
#[element(style(dialog, dialog_background), events(dialog, dialog_background))]
pub struct Dialog<PMsg, C> {
    // general element properties
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(props(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg, C>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    // dialog element properties
    #[rich(read)]
    #[element(theme_lens(nested), props(default))]
    header_bar: HeaderBar<Msg>,
    #[element(props(required))]
    pub child: C,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, props(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_mouse_on_widget))]
    #[element(theme_lens)]
    mouse_on_dialog: bool,
    #[element(theme_lens, props(default = "State::Closed"))]
    state: State,
}

pub enum Msg {
    // EventsStore<Events<PMsg>>
    EventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    UpdateEventsStore(Rc<dyn Any>),
    // Option<Styler<PMsg, C>>
    Styler(Rc<dyn Any>),
    // Box<dyn Fn(Styler<PMsg, C>) -> Styler<PMsg, C>>
    UpdateStyler(Rc<dyn Any>),
    Theme(Theme),
    Title(Option<Label<Msg>>),
    Subtitle(Option<Label<Msg>>),
    MouseOnDialog(bool),
    ClickedOutSide,
    Toggled(bool),
    Toggle,
    CloseButton(button::Msg),
}

impl<PMsg, C> Element<PMsg> for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>> + 'static,
{
    type Message = Msg;
    type Props = Props<PMsg, C>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let local_events = || {
            events()
                .and_dialog_background(|conf| conf.click(|_| Msg::clicked_out_side()))
                .and_dialog(|conf| {
                    conf.mouse_enter(|_| Msg::mouse_on_dialog(true))
                        .mouse_leave(|_| Msg::mouse_on_dialog(false))
                })
        };

        let header_bar = props.header_bar.close_button(
            Button::build(Msg::close_button)
                // FIXME: use icon insted of label
                .label("X")
                .events(|| button::events().and_button(|conf| conf.click(|_| Msg::close())))
                .init(&mut orders),
        );

        Self {
            msg_mapper: props.msg_mapper,
            local_events: local_events.into(),
            events: props.events,
            styler: props.styler,
            theme: props.theme,
            header_bar,
            child: props.child,
            disabled: props.disabled,
            state: props.state,
            mouse_on_dialog: false,
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
                if let Ok(val) = val.downcast::<Option<Styler<PMsg, C>>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(Styler<PMsg, C>) -> Styler<PMsg, C>>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Title(val) => self.header_bar.title = val,
            Msg::Subtitle(val) => self.header_bar.subtitle = val,
            Msg::MouseOnDialog(val) => self.mouse_on_dialog = val,
            Msg::ClickedOutSide => {
                if !self.mouse_on_dialog {
                    self.set_toggled(false, &mut orders);
                }
            }
            Msg::Toggled(val) => self.set_toggled(val, &mut orders),
            Msg::Toggle => self.toggle(&mut orders),
            Msg::CloseButton(msg) => {
                if let Some(ref mut btn) = self.header_bar.close_button {
                    btn.update(msg, &mut orders)
                }
            }
        }
    }
}

impl<PMsg, C> View for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.dialog().get(&self.theme_lens())),
        )
    }
}

impl<PMsg, C> StyledView for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    type Style = Style;

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
            .class("dialog-background")
            .set(style.dialog_background)
            .set(&local_events.dialog_background)
            .map_msg_with(&self.msg_mapper)
            .add(dialog)
            .add(&events.dialog_background)
    }
}

impl<PMsg, C> Props<PMsg, C>
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

impl<PMsg: 'static, C> Dialog<PMsg, C> {
    fn set_toggled(&mut self, val: bool, orders: &mut impl Orders<Msg>) {
        if val {
            // open
            match self.state {
                State::Opened => {}
                State::Closed | State::Closing => {
                    self.state = State::Opening;
                    orders.after_next_render(|_| Msg::open());
                }
                State::Opening => self.state = State::Opened,
            }
        } else {
            // close
            match self.state {
                State::Closed => {}
                State::Opened | State::Opening => {
                    self.state = State::Closing;
                    orders.perform_cmd_after(400, || Msg::close());
                }
                State::Closing => {
                    self.state = State::Closed;
                }
            }
        }
    }

    fn toggle(&mut self, orders: &mut impl Orders<Msg>) {
        match self.state {
            State::Opened | State::Opening => self.set_toggled(false, orders),
            State::Closed | State::Closing => self.set_toggled(true, orders),
        }
    }
}

pub fn events<PMsg>() -> Events<PMsg> {
    Events::default()
}

pub fn style() -> Style {
    Style::default()
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
}

pub type Styler<PMsg, C> = theme::Styler<Dialog<PMsg, C>, Style>;
pub type ThemeStyler<'a> = theme::Styler<DialogLens<'a>, Style>;

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static, C: 'static>(val: Styler<PMsg, C>) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static, C: 'static>(
        val: impl Fn(Styler<PMsg, C>) -> Styler<PMsg, C> + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static, C: 'static>(val: Option<Styler<PMsg, C>>) -> Self {
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
