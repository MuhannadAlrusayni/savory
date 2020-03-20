use crate::{css, prelude::*};
use derive_rich::Rich;

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
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

    // switch element properties
    #[rich(
        read(copy, rename = is_disabled),
    )]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
    #[rich(
        read(copy, rename = is_toggled),
    )]
    toggled: bool,
}

impl<PMsg> Switch<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let local_events = Events::default()
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Click);

        Self {
            msg_mapper: msg_mapper.into(),
            local_events,
            events: Events::default(),
            user_style: UserStyle::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
            toggled: false,
        }
    }

    fn handle_toggle_msg(&mut self) {
        self.toggled = !self.toggled;
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Switch<PMsg> {
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
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub background: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub background: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Switch<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.switch(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let button = div!()
            .and_attributes(|conf| conf.set_class("switch-button"))
            .set_style(style.button);

        button!()
            .set_events(&self.local_events)
            .set_style(style.background)
            .and_attributes(|conf| conf.set_class("switch").set_disabled(self.disabled))
            .add_children(vec![button])
            .map_msg_with(&self.msg_mapper)
            .add_events(&self.events)
    }
}
