use crate::{css, prelude::*};
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Click,
}

#[derive(Clone, Rich)]
pub struct Checkbox<PMsg> {
    local_events: Events<Msg>,
    lbl_events: Events<Msg>,
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(write)]
    pub label: Option<Cow<'static, str>>,
    #[rich(write(style = compose))]
    pub style: Style,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
    #[rich(
        read(copy, rename = is_toggled),
        value_fns = { toggle_on = true, toggle_off = false }
    )]
    toggle: bool,
}

impl<PMsg> Checkbox<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let mut local_events = Events::default();
        local_events
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Click);

        let mut lbl_events = Events::default();
        lbl_events
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave);

        Self {
            msg_mapper: msg_mapper.into(),
            events: Events::default(),
            local_events,
            lbl_events,
            label: None,
            style: Style::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
            toggle: false,
        }
    }

    pub fn toggle(&mut self) -> &mut Self {
        self.toggle = !self.toggle;
        self
    }

    pub fn handle_toggle_msg(&mut self) {
        self.toggle = !self.toggle;
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Checkbox<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Click => self.handle_toggle_msg(),
        }
    }
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub input: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
    #[rich(write(style = compose))]
    pub label: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Checkbox<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.checkbox(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let input = input![
            att::disabled(self.disabled),
            att::checked(self.toggle),
            att::ty(att::Type::Checkbox),
            style.input,
            self.local_events.events.clone(),
            if self.is_toggled() {
                div![style.button]
            } else {
                empty![]
            },
        ];

        let msg_mapper = self.msg_mapper.map_msg_once();
        let mut checkbox = if let Some(ref lbl) = self.label {
            label![
                style.label,
                input,
                lbl.to_string(),
                self.lbl_events.events.clone(),
            ]
        } else {
            input
        }
        .map_msg(msg_mapper);
        for event in self.events.events.clone().into_iter() {
            checkbox.add_listener(event);
        }
        checkbox
    }
}
