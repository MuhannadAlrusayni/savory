use crate::{css, events::Events, model::Model, propertie::Size, render::Render, theme::Theme};
use derive_rich::Rich;
use seed::prelude::*;
use std::{borrow::Cow, rc::Rc};

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
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    #[rich(write(take))]
    pub label: Option<Cow<'static, str>>,
    #[rich(write(take, style = compose))]
    pub style: Style,
    #[rich(value_fns(take) = {
        small = Size::Small,
        medium = Size::Medium,
        large = Size::Large,
    })]
    pub size: Option<Size>,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns(take) = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
    #[rich(
        read(copy, rename = is_toggled),
        value_fns(take) = { toggle_on = true, toggle_off = false }
    )]
    toggle: bool,
}

impl<PMsg> Radio<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        Self {
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            events: Events::default(),
            label: None,
            style: Style::default(),
            size: None,
            disabled: false,
            focus: false,
            mouse_over: false,
            toggle: false,
        }
    }

    pub fn toggle(mut self) -> Self {
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
    #[rich(write(take, style = compose))]
    pub input: css::Style,
    #[rich(write(take, style = compose))]
    pub button: css::Style,
    #[rich(write(take, style = compose))]
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
        let events = Events::default()
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Click);
        let input = input![
            attrs![
                At::Disabled => self.disabled.as_at_value(),
                At::Checked => self.toggle.as_at_value(),
                At::Type => "radio",
            ],
            events.events,
            style.input,
            if self.is_toggled() {
                div![style.button]
            } else {
                empty![]
            },
        ];

        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut radio = if let Some(ref lbl) = self.label {
            let events = Events::default()
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave);
            label![style.label, input, lbl.to_string(), events.events]
        } else {
            input
        }
        .map_msg(move |msg| (msg_mapper.clone())(msg));

        for event in self.events.events.clone().into_iter() {
            radio.add_listener(event);
        }
        radio
    }
}
