use crate::{
    css::Style,
    events::Events,
    model::Model,
    propertie::{Shape, Size},
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

#[derive(Rich)]
pub struct Switch<PMsg> {
    internal_events: Events<Msg>,
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    #[rich(write(take, style = compose))]
    pub style: Style,
    #[rich(value_fns(take) = {
        small = Size::Small,
        medium = Size::Medium,
        large = Size::Large,
    })]
    pub size: Option<Size>,
    #[rich(value_fns(take) = {
        circle = Shape::Circle,
        round = Shape::Round,
        rectangle = Shape::Rectangle
    })]
    pub shape: Option<Shape>,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns(take) = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(
        read(copy, rename = is_loading),
        value_fns(take) = { loading = true, loading_off = false }
    )]
    pub loading: bool,
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

impl<PMsg> Switch<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        Self {
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            internal_events: Events::default()
                .focus(|_| Msg::Focus)
                .blur(|_| Msg::Blur)
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave)
                .click(|_| Msg::Click),
            events: Events::default(),
            style: Style::default(),
            size: None,
            shape: None,
            disabled: false,
            loading: false,
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

impl<GMsg, PMsg: 'static> Model<Msg, PMsg, GMsg> for Switch<PMsg> {
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

impl<PMsg: 'static> Render<PMsg> for Switch<PMsg> {
    type View = Node<PMsg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let (bg_style, btn_style) = theme.switch(self);
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());

        button![
            self.internal_events.events.clone(),
            attrs![
                At::Disabled => self.disabled.as_at_value(),
            ],
            bg_style,
            div![btn_style],
        ]
        .map_msg(move |msg| (msg_mapper.clone())(msg))
    }
}

impl<PMsg> Themeable for Switch<PMsg> {
    type StyleMap = (Style, Style);
}
