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
pub struct Radio<PMsg> {
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

impl<PMsg> Radio<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        Self {
            msg_mapper: msg_mapper.into(),
            events: Events::default(),
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

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Radio<PMsg> {
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

impl<PMsg: 'static> Render<PMsg> for Radio<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.radio(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        // TODO: create these event in the `new` function
        let mut events = Events::default();
        events
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Click);
        let input = input![
            att::disabled(self.disabled),
            att::checked(self.toggle),
            att::type_(att::Type::Radio),
            events.events,
            style.input,
            if self.is_toggled() {
                div![style.button]
            } else {
                empty![]
            },
        ];

        let msg_mapper = self.msg_mapper.map_msg_once();
        let mut radio = if let Some(ref lbl) = self.label {
            let mut events = Events::default();
            events
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave);
            label![style.label, input, lbl.to_string(), events.events]
        } else {
            input
        }
        .map_msg(msg_mapper);

        for event in self.events.events.clone().into_iter() {
            radio.add_listener(event);
        }
        radio
    }
}
