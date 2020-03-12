use crate::{css, prelude::*};
use derive_rich::Rich;

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Clear,
    UpdateText(web_sys::InputEvent),
}

#[derive(Default, Rich)]
pub struct LocalEvents {
    #[rich(write(style = compose))]
    pub input: Events<Msg>,
    #[rich(write(style = compose))]
    pub container: Events<Msg>,
}

impl LocalEvents {
    pub fn remove_events(self) -> Self {
        Self::default()
    }
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(style = compose))]
    pub input: Events<PMsg>,
    #[rich(write(style = compose))]
    pub container: Events<PMsg>,
}

impl<PMsg> Default for ParentEvents<PMsg> {
    fn default() -> Self {
        Self {
            input: Events::default(),
            container: Events::default(),
        }
    }
}

#[derive(Rich)]
pub struct Entry<PMsg> {
    #[rich(write(style = compose))]
    pub local_events: LocalEvents,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(write(style = compose))]
    pub events: ParentEvents<PMsg>,
    #[rich(write, write(rename = text_mut))]
    pub text: Option<String>,
    #[rich(write)]
    pub max_length: Option<usize>,
    #[rich(write)]
    pub placeholder: Option<String>,
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
}

impl<PMsg> Entry<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let mut local_events = LocalEvents::default();
        local_events.input(|conf| {
            conf.focus(|_| Msg::Focus)
                .blur(|_| Msg::Blur)
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave)
                .input(Msg::UpdateText)
        });

        Self {
            local_events,
            events: ParentEvents::default(),
            msg_mapper: msg_mapper.into(),
            text: None,
            max_length: None,
            placeholder: None,
            style: Style::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    fn handle_update_text(&mut self, event: web_sys::InputEvent) {
        if let Some(text) = event
            .target()
            .map(|t| seed::util::get_value(&t).ok())
            .flatten()
        {
            self.text = Some(text);
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
            Msg::UpdateText(event) => self.handle_update_text(event),
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
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
    pub input: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Entry<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.entry(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let msg_mapper = self.msg_mapper.map_msg_once();
        let mut input = input![
            self.local_events.input.clone(),
            style.input,
            att::disabled(self.disabled),
            self.text.clone().map(att::value),
            self.max_length.map(att::max_length),
            self.placeholder.clone().map(att::placeholder),
        ]
        .map_msg(msg_mapper.clone());

        for event in self.events.input.events.clone().into_iter() {
            input.add_listener(event);
        }

        let mut container =
            div![style.container, self.local_events.container.clone(),].map_msg(msg_mapper);

        for event in self.events.container.events.clone().into_iter() {
            container.add_listener(event);
        }

        container.add_child(input);
        container
    }
}
