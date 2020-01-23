use crate::{
    css::{self, unit::px, values as val, St, Style},
    events::Events,
    macros::*,
    model::Model,
    propertie::Size,
    render::Render,
    theme::{Theme, Themeable},
};
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
    map_msg: Rc<dyn Fn(Msg) -> PMsg>,
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
    pub fn new(map_msg: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        Self {
            map_msg: Rc::new(move |msg| (map_msg.clone())(msg)),
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

impl<GMsg, PMsg: 'static> Model<Msg, PMsg, GMsg> for Radio<PMsg> {
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

impl<PMsg: 'static> Render<PMsg> for Radio<PMsg> {
    type View = Node<PMsg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let (input_style, btn_style, lbl_style) = theme.radio(self);

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
            input_style,
            if self.is_toggled() {
                div![btn_style]
            } else {
                empty![]
            },
        ];

        let msg_mapper = Rc::clone(&self.map_msg.clone());
        if let Some(ref lbl) = self.label {
            let events = Events::default()
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave);
            label![lbl_style, input, lbl.to_string(), events.events]
        } else {
            input
        }
        .map_msg(move |msg| (msg_mapper.clone())(msg))
    }
}

impl<PMsg> Themeable for Radio<PMsg> {
    type StyleMap = (Style, Style, Style);
}
