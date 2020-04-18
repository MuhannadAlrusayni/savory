use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(checkbox, button, label), events(checkbox, button, label))]
pub struct Checkbox<PMsg> {
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

    // checkbox element properties
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

impl<PMsg: 'static> Element<PMsg> for Checkbox<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default()
            .and_checkbox(|conf| {
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
            msg_mapper: props.msg_mapper,
            local_events,
            events: props.events,
            label: props.label,
            styler: props.styler,
            disabled: props.disabled,
            toggled: props.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _orders: &mut impl Orders<PMsg>) {
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

impl<PMsg: 'static> View for Checkbox<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.checkbox()(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Checkbox<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let Style {
            checkbox,
            button,
            label,
        } = style;

        let checkbox = html::input()
            .class("checbox")
            .set(att::disabled(self.disabled))
            .set(att::checked(self.toggled))
            .set(att::Type::Checkbox)
            .set(checkbox)
            .set(&self.local_events.checkbox)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.checkbox)
            .el_ref(&self.input_el_ref)
            // add button if the checkbox is toggled
            .config_if(self.is_toggled(), |conf| {
                let button = html::div()
                    .class("button")
                    .set(button)
                    .map_msg_with(&self.msg_mapper)
                    .add(&self.events.button);
                conf.add(button)
            });

        match self.label.as_ref() {
            None => checkbox,
            Some(lbl) => html::label()
                .class("label")
                .set(label)
                .set(&self.local_events.label)
                .map_msg_with(&self.msg_mapper)
                .add(&self.events.label)
                .add(checkbox)
                .add(lbl.clone())
                .el_ref(&self.label_el_ref),
        }
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Checkbox<PMsg> {
        Checkbox::init(self, orders)
    }
}

impl<PMsg: 'static> Checkbox<PMsg> {
    pub fn and_events(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn try_set_styler(
        &mut self,
        val: Option<impl Into<Styler<PMsg>>>,
        _: &mut impl Orders<PMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler(
        &mut self,
        val: impl Into<Styler<PMsg>>,
        orders: &mut impl Orders<PMsg>,
    ) {
        self.try_set_styler(Some(val), orders);
    }
}

pub type Styler<PMsg> = theme::Styler<Checkbox<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<CheckboxLens<'a>, Style>;
