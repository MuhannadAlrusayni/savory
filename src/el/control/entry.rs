use crate::{
    css,
    el::prelude::*,
    events::Events,
    model::Model,
    propertie::{Shape, Size},
    render::Render,
    theme::Theme,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::{borrow::Cow, rc::Rc};

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Clear,
    UpdateText(String),
}

#[derive(Rich)]
pub struct Entry<PMsg> {
    internal_events: Events<Msg>,
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    #[rich(write(take))]
    pub text: Option<String>,
    #[rich(write(take))]
    pub max_length: Option<usize>,
    #[rich(write(take))]
    pub placeholder: Option<String>,
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
}

impl<PMsg> Entry<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        Self {
            internal_events: Events::default()
                .focus(|_| Msg::Focus)
                .blur(|_| Msg::Blur)
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave)
                .input(Msg::UpdateText),
            events: Events::default(),
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            text: None,
            max_length: None,
            placeholder: None,
            style: Style::default(),
            size: None,
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    fn handle_clear_msg(&mut self) {
        self.text = None;
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Entry<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::UpdateText(text) => self.text = Some(text),
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Clear => self.handle_clear_msg(),
        }
    }
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(take, style = compose))]
    pub container: css::Style,
    #[rich(write(take, style = compose))]
    pub input: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Entry<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.entry(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut input = input![
            self.internal_events.events.clone(),
            style.input,
            attrs![
                At::Disabled => self.disabled.as_at_value(),
                At::Value => self.text.as_ref().map(|v| AtValue::Some(v.clone())).unwrap_or(AtValue::Ignored),
                // At::MaxLength => self.max_length,
                // At::Placeholder => self.placeholder,
            ],
        ].map_msg(move |msg| (msg_mapper.clone())(msg));

        for event in self.events.events.clone().into_iter() {
            input.add_listener(event);
        }

        div![style.container, input]
    }
}
