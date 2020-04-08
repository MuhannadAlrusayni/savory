use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
pub struct Checkbox<PMsg> {
    // general element properties
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: Events<Msg>,
    #[rich(read)]
    events: Events<PMsg>,
    #[rich(read)]
    style: Option<Style>,
    #[rich(read)]
    theme: Theme,

    // checkbox element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens)]
    toggled: bool,
    #[rich(read)]
    label: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,
}

crate::style_type! {
    checkbox,
    button,
    label,
}

crate::events_type! {
    checkbox,
    button,
    label,
}

pub enum Msg {
    SetTheme(Theme),
    SetStyle(Box<dyn FnOnce(Style) -> Style>),
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

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Checkbox<PMsg> {
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        let msg_mapper = msg_mapper.into();
        let mut orders = orders.proxy_with(&msg_mapper);
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
            theme: Theme::default(),
            input_el_ref: ElRef::default(),
            label_el_ref: ElRef::default(),
            msg_mapper: msg_mapper,
            local_events,
            events: Events::default(),
            label: None,
            style: None,
            disabled: false,
            focused: false,
            mouse_over: false,
            toggled: false,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetTheme(val) => self.set_theme(val, &mut orders),
            Msg::SetStyle(val) => self.set_style(val, &mut orders),
            Msg::SetLabel(val) => self.try_set_label(Some(val), &mut orders),
            Msg::TrySetLabel(val) => self.try_set_label(val, &mut orders),
            Msg::SetToggled(val) => self.set_toggled(val, &mut orders),
            Msg::Toggle => self.set_toggled(!self.toggled, &mut orders),
            Msg::ToggleOn => self.set_toggled(true, &mut orders),
            Msg::ToggleOff => self.set_toggled(false, &mut orders),
            Msg::SetDisabled(val) => self.set_disabled(val, &mut orders),
            Msg::Disable => self.set_disabled(true, &mut orders),
            Msg::Enable => self.set_disabled(false, &mut orders),
            Msg::SetFocus(val) => self.set_focused(val, &mut orders),
            Msg::SetMouseOver(val) => self.set_mouse_over(val, &mut orders),
        }
    }
}

impl<PMsg: 'static> View for Checkbox<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        let view = |style: &Style| {
            let checkbox = html::input()
                .set(att::class("checbox"))
                .set(att::class("checbox"))
                .set(att::disabled(self.disabled))
                .set(att::checked(self.toggled))
                .set(att::Type::Checkbox)
                .set(&style.checkbox)
                .set(&self.local_events.checkbox)
                .map_msg_with(&self.msg_mapper)
                .add(&self.events.checkbox)
                .el_ref(&self.input_el_ref)
                // add button if the checkbox is toggled
                .config_if(self.is_toggled(), |conf| {
                    let button = html::div()
                        .add(att::class("button"))
                        .set(&style.button)
                        .map_msg_with(&self.msg_mapper)
                        .add(&self.events.button);
                    conf.add(button)
                });

            match self.label.as_ref() {
                None => checkbox,
                Some(lbl) => html::label()
                    .add(att::class("label"))
                    .set(&style.label)
                    .set(&self.local_events.label)
                    .map_msg_with(&self.msg_mapper)
                    .add(&self.events.label)
                    .add(checkbox)
                    .add(html::text(lbl.clone()))
                    .el_ref(&self.label_el_ref),
            }
        };

        match self.style {
            Some(ref style) => view(&style),
            None => view(&self.theme.checkbox(self.theme_lens())),
        }
    }
}

impl<PMsg: 'static> Checkbox<PMsg> {
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    fn set_theme<GMsg: 'static>(&mut self, val: Theme, _orders: &mut impl Orders<Msg, GMsg>) {
        self.theme = val;
    }

    fn set_style<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Style) -> Style,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        // FIXME: finder better way, that doesn't need to clone the style
        self.style = match self.style {
            Some(ref style) => Some(get_val(style.clone())),
            None => Some(get_val(self.theme.checkbox(self.theme_lens()))),
        };
    }

    fn try_set_label<GMsg: 'static>(
        &mut self,
        val: Option<Cow<'static, str>>,
        orders: &mut impl Orders<Msg, GMsg>,
    ) {
        if self.label != val {
            self.label = val;
        } else {
            orders.skip();
        }
    }

    fn set_toggled<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.toggled != val {
            self.toggled = val;
        } else {
            orders.skip();
        }
    }

    fn set_disabled<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.disabled != val {
            self.disabled = val;
        } else {
            orders.skip();
        }
    }

    fn set_focused<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.focused != val {
            self.focused = val;
        } else {
            orders.skip();
        }
    }

    fn set_mouse_over<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.mouse_over != val {
            self.mouse_over = val;
        } else {
            orders.skip();
        }
    }
}
