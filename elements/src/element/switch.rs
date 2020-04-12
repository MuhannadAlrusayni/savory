use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(style(button, switch), events(button, switch))]
pub struct Switch<PMsg> {
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
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    // switch element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens, props(default = "false"))]
    toggled: bool,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, props(default = "false"))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,
}

pub enum Msg {
    SetTheme(Theme),
    SetToggled(bool),
    Toggle,
    ToggleOn,
    ToggleOff,
    SetDisabled(bool),
    Disable,
    Enable,
    SetFocus(bool),
    SetMouseOver(bool),
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Switch<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default().and_switch(|conf| {
            conf.focus(|_| Msg::SetFocus(true))
                .blur(|_| Msg::SetFocus(false))
                .mouse_enter(|_| Msg::SetMouseOver(true))
                .mouse_leave(|_| Msg::SetMouseOver(false))
                .click(|_| Msg::Toggle)
        });

        Self {
            theme: props.theme,
            msg_mapper: props.msg_mapper,
            local_events,
            events: props.events,
            styler: props.styler,
            disabled: props.disabled,
            toggled: props.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetToggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
            Msg::ToggleOn => self.toggled = true,
            Msg::ToggleOff => self.toggled = false,
            Msg::SetDisabled(val) => self.disabled = val,
            Msg::Disable => self.disabled = true,
            Msg::Enable => self.disabled = false,
            Msg::SetFocus(val) => self.focused = val,
            Msg::SetMouseOver(val) => self.mouse_over = val,
        }
    }
}

impl<PMsg: 'static> View for Switch<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.switch()(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Switch<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let button = html::div()
            .set(att::class("button"))
            .set(&style.button)
            .set(&self.local_events.button)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.button);

        html::button()
            .set(att::class("switch"))
            .set(att::disabled(self.disabled))
            .set(&style.switch)
            .set(&self.local_events.switch)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.switch)
            .add(button)
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init<GMsg: 'static>(self, orders: &mut impl Orders<PMsg, GMsg>) -> Switch<PMsg> {
        Switch::init(self, orders)
    }
}

impl<PMsg: 'static> Switch<PMsg> {
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn try_set_styler<GMsg: 'static>(
        &mut self,
        val: Option<impl Into<Styler<PMsg>>>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler<GMsg: 'static>(
        &mut self,
        val: impl Into<Styler<PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.try_set_styler(Some(val), orders);
    }
}

pub type Styler<PMsg> = theme::Styler<Switch<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<SwitchLens<'a>, Style>;
