use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(radio, button, label), events(radio, button, label))]
pub struct Radio<PMsg> {
    // general element properties
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
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

    // radio element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens, props(default))]
    toggled: bool,
    #[rich(read)]
    #[element(theme_lens, props)]
    label: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, props(default))]
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
    SetLabel(Cow<'static, str>),
    TrySetLabel(Option<Cow<'static, str>>),
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

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Radio<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default()
            .and_radio(|conf| {
                conf.focus(|_| Msg::SetFocus(true))
                    .blur(|_| Msg::SetFocus(false))
                    .mouse_enter(|_| Msg::SetMouseOver(true))
                    .mouse_leave(|_| Msg::SetMouseOver(false))
                    .click(|_| Msg::Toggle)
            })
            .and_label(|conf| {
                conf.mouse_enter(|_| Msg::SetMouseOver(true))
                    .mouse_leave(|_| Msg::SetMouseOver(false))
            });

        Self {
            input_el_ref: ElRef::default(),
            label_el_ref: ElRef::default(),
            theme: props.theme,
            styler: props.styler,
            msg_mapper: props.msg_mapper,
            local_events,
            events: props.events,
            label: props.label,
            disabled: props.disabled,
            toggled: props.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetLabel(val) => self.label = Some(val),
            Msg::TrySetLabel(val) => self.label = val,
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

impl<PMsg: 'static> View for Radio<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.radio()(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Radio<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let radio = html::input()
            .set(att::class("radio"))
            .set(att::disabled(self.disabled))
            .set(att::checked(self.toggled))
            .set(att::Type::Radio)
            .set(&style.radio)
            .set(&self.local_events.radio)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.radio)
            .el_ref(&self.input_el_ref)
            // add button if the radio is toggled
            .config_if(self.is_toggled(), |conf| {
                let button = html::div()
                    .add(att::class("button"))
                    .set(&style.button)
                    .map_msg_with(&self.msg_mapper)
                    .add(&self.events.button);
                conf.add(button)
            });

        match self.label.as_ref() {
            None => radio,
            Some(lbl) => html::label()
                .add(att::class("label"))
                .set(&style.label)
                .set(&self.local_events.label)
                .map_msg_with(&self.msg_mapper)
                .add(&self.events.label)
                .add(radio)
                .add(lbl.clone())
                .el_ref(&self.label_el_ref),
        }
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init<GMsg: 'static>(self, orders: &mut impl Orders<PMsg, GMsg>) -> Radio<PMsg> {
        Radio::init(self, orders)
    }
}

impl<PMsg: 'static> Radio<PMsg> {
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

pub type Styler<PMsg> = theme::Styler<Radio<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<RadioLens<'a>, Style>;
