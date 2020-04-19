use crate::{header_bar::HeaderBarLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(style(dialog, dialog_background), events(dialog, dialog_background))]
pub struct Dialog<PMsg, C> {
    // general element properties
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: Events<Msg>,
    #[rich(read)]
    #[element(props(default))]
    events: Events<PMsg>,
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
    #[rich(read)]
    #[element(props(required))]
    child: C,
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
    SetTheme(Theme),
    SetHeaderBar(HeaderBar<Msg>),
    SetHeaderBarHidden(bool),
    HideHeaderBar,
    ShowHeaderBar,
    SetTitle(Label<Msg>),
    TrySetTitle(Option<Label<Msg>>),
    SetSubtitle(Label<Msg>),
    TrySetSubtitle(Option<Label<Msg>>),
    SetMouseOnDialog(bool),
    ClickedOutside,
    SetToggled(bool),
    Toggle,
    Open,
    Close,
    CloseButton(button::Msg),
}

impl<PMsg, C> Element<PMsg> for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    type Message = Msg;
    type Props = Props<PMsg, C>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default()
            .and_dialog_background(|conf| conf.click(|_| Msg::ClickedOutside))
            .and_dialog(|conf| {
                conf.mouse_enter(|_| Msg::SetMouseOnDialog(true))
                    .mouse_leave(|_| Msg::SetMouseOnDialog(false))
            });

        let header_bar = props.header_bar.close_button(
            Button::build(Msg::CloseButton)
                // FIXME: use icon insted of label
                .label("X")
                .events(button::Events::default().and_button(|conf| conf.click(|_| Msg::Close)))
                .init(&mut orders),
        );

        Self {
            msg_mapper: props.msg_mapper,
            local_events,
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
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetHeaderBar(val) => self.header_bar = val,
            Msg::SetHeaderBarHidden(val) => self.header_bar.hidden = val,
            Msg::HideHeaderBar => self.header_bar.hidden = true,
            Msg::ShowHeaderBar => self.header_bar.hidden = false,
            Msg::SetTitle(val) => self.header_bar.title = Some(val),
            Msg::TrySetTitle(val) => self.header_bar.title = val,
            Msg::SetSubtitle(val) => self.header_bar.subtitle = Some(val),
            Msg::TrySetSubtitle(val) => self.header_bar.subtitle = val,
            Msg::SetMouseOnDialog(val) => self.mouse_on_dialog = val,
            Msg::ClickedOutside => {
                if !self.mouse_on_dialog {
                    self.set_toggled(false, &mut orders);
                }
            }
            Msg::SetToggled(val) => self.set_toggled(val, &mut orders),
            Msg::Toggle => self.toggle(&mut orders),
            Msg::Open => self.set_toggled(true, &mut orders),
            Msg::Close => self.set_toggled(false, &mut orders),
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
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.dialog()(&self.theme_lens())),
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
        let dialog = html::div()
            .class("dialog")
            .set(style.dialog)
            .set(&self.local_events.dialog)
            .map_msg_with(&self.msg_mapper)
            .add(self.header_bar.view().map_msg_with(&self.msg_mapper))
            .add(self.child.view())
            .add(&self.events.dialog);

        html::div()
            .class("dialog-background")
            .set(style.dialog_background)
            .set(&self.local_events.dialog_background)
            .map_msg_with(&self.msg_mapper)
            .add(dialog)
            .add(&self.events.dialog_background)
    }
}

impl<PMsg, C> Props<PMsg, C>
where
    PMsg: 'static,
    C: View<Output = Node<PMsg>>,
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
    pub fn and_events(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn try_set_styler(
        &mut self,
        val: Option<impl Into<Styler<PMsg, C>>>,
        _: &mut impl Orders<PMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler(&mut self, val: impl Into<Styler<PMsg, C>>, orders: &mut impl Orders<PMsg>) {
        self.try_set_styler(Some(val), orders)
    }

    pub fn update_child(&mut self, child_msg: C::Message, orders: &mut impl Orders<PMsg>)
    where
        C: Element<PMsg>,
    {
        self.child.update(child_msg, orders)
    }

    fn set_toggled(&mut self, val: bool, orders: &mut impl Orders<Msg>) {
        if val {
            // open
            match self.state {
                State::Opened => {}
                State::Closed | State::Closing => {
                    self.state = State::Opening;
                    orders.after_next_render(|_| Msg::Open);
                }
                State::Opening => self.state = State::Opened,
            }
        } else {
            // close
            match self.state {
                State::Closed => {}
                State::Opened | State::Opening => {
                    self.state = State::Closing;
                    orders.perform_cmd_after(400, || Msg::Close);
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

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
}

pub type Styler<PMsg, C> = theme::Styler<Dialog<PMsg, C>, Style>;
pub type ThemeStyler<'a> = theme::Styler<DialogLens<'a>, Style>;
