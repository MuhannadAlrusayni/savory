use crate::{icon::IconLens, label::LabelLens, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(button), events(button))]
pub struct Button<PMsg> {
    // general element properties
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: Events<Msg>,
    #[rich(read)]
    events: Events<PMsg>,
    #[rich(read)]
    style: Option<Style>,
    #[rich(read)]
    theme: Theme,

    // button element properties
    #[rich(read)]
    #[element(theme_lens(nested))]
    label: Option<Label<Msg>>,
    #[rich(read)]
    #[element(theme_lens(nested))]
    icon: Option<Icon<Msg>>,
    #[rich(read(copy))]
    #[element(theme_lens)]
    kind: Option<Kind>,
    #[rich(read(copy))]
    #[element(theme_lens)]
    ghost: bool,
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

pub enum Msg {
    SetTheme(Theme),
    SetStyleFn(Box<dyn FnOnce(Style) -> Style>),
    SetStyle(Style),
    SetLabel(Label<Msg>),
    TrySetLabel(Option<Label<Msg>>),
    SetIcon(Icon<Msg>),
    TrySetIcon(Option<Icon<Msg>>),
    SetKind(Kind),
    TrySetKind(Option<Kind>),
    SetGhost(bool),
    SetGhostOn,
    SetGhostOff,
    SetDisabled(bool),
    Disable,
    Enable,
    SetFocus(bool),
    SetMouseOver(bool),
}

impl Msg {
    pub fn set_style_fn(get_style: impl FnOnce(Style) -> Style + 'static) -> Self {
        Msg::SetStyleFn(Box::new(get_style))
    }
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Button<PMsg> {
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        let msg_mapper = msg_mapper.into();
        let mut orders = orders.proxy_with(&msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default().and_button(|conf| {
            conf.focus(|_| Msg::SetFocus(true))
                .blur(|_| Msg::SetFocus(false))
                .mouse_enter(|_| Msg::SetMouseOver(true))
                .mouse_leave(|_| Msg::SetMouseOver(false))
        });

        Button {
            theme: Theme::default(),
            el_ref: ElRef::default(),
            msg_mapper: msg_mapper,
            local_events,
            events: Events::default(),
            label: None,
            icon: None,
            kind: None,
            ghost: false,
            style: None,
            disabled: false,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetTheme(val) => self.set_theme(val, &mut orders),
            Msg::SetStyleFn(val) => self.set_style_fn(val, &mut orders),
            Msg::SetStyle(val) => self.set_style(val, &mut orders),
            Msg::SetLabel(val) => self.try_set_label(Some(val), &mut orders),
            Msg::TrySetLabel(val) => self.try_set_label(val, &mut orders),
            Msg::SetIcon(val) => self.try_set_icon(Some(val), &mut orders),
            Msg::TrySetIcon(val) => self.try_set_icon(val, &mut orders),
            Msg::SetKind(val) => self.try_set_kind(Some(val), &mut orders),
            Msg::TrySetKind(val) => self.try_set_kind(val, &mut orders),
            Msg::SetGhost(val) => self.set_ghost(val, &mut orders),
            Msg::SetGhostOn => self.set_ghost(true, &mut orders),
            Msg::SetGhostOff => self.set_ghost(false, &mut orders),
            Msg::SetDisabled(val) => self.set_disabled(val, &mut orders),
            Msg::Disable => self.set_disabled(true, &mut orders),
            Msg::Enable => self.set_disabled(false, &mut orders),
            Msg::SetFocus(val) => self.set_focused(val, &mut orders),
            Msg::SetMouseOver(val) => self.set_mouse_over(val, &mut orders),
        }
    }
}

impl<PMsg: 'static> View for Button<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        let view = |style: &Style| {
            html::button()
                .add(att::class("button"))
                .add(att::disabled(self.disabled))
                .set(&self.local_events.button)
                .set(&style.button)
                .try_add(self.icon.as_ref().map(|icon| icon.view()))
                .try_add(self.label.as_ref().map(|lbl| lbl.view()))
                .map_msg_with(&self.msg_mapper)
                .add(&self.events.button)
        };

        match self.style {
            Some(ref style) => view(&style),
            None => view(&self.theme.button(self.theme_lens())),
        }
    }
}

impl<PMsg: 'static> Button<PMsg> {
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    fn set_theme<GMsg: 'static>(&mut self, val: Theme, orders: &mut impl Orders<Msg, GMsg>) {
        self.theme = val;
    }

    fn set_style_fn<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Style) -> Style,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        // FIXME: finder better way, that doesn't need to clone the style
        self.style = match self.style {
            Some(ref style) => Some(get_val(style.clone())),
            None => Some(get_val(self.theme.button(self.theme_lens()))),
        };
    }

    fn set_style<GMsg: 'static>(&mut self, val: Style, _orders: &mut impl Orders<Msg, GMsg>) {
        self.style = Some(val);
    }

    fn try_set_label<GMsg: 'static>(
        &mut self,
        val: Option<Label<Msg>>,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        self.label = val;
    }

    fn try_set_kind<GMsg: 'static>(
        &mut self,
        val: Option<Kind>,
        orders: &mut impl Orders<Msg, GMsg>,
    ) {
        if self.kind != val {
            self.kind = val;
        } else {
            orders.skip();
        }
    }

    fn try_set_icon<GMsg: 'static>(
        &mut self,
        val: Option<Icon<Msg>>,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        self.icon = val;
    }

    fn set_ghost<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.ghost != val {
            self.ghost = val;
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

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}
