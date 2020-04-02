use crate::prelude::*;
use derive_rich::Rich;

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Click,
}

#[derive(Rich, Element)]
pub struct Switch<PMsg> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,

    // switch element properties
    #[rich(
        read(copy, rename = is_disabled),
    )]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,
    #[rich(
        read(copy, rename = is_toggled),
    )]
    #[element(theme_lens)]
    toggled: bool,
}

impl<PMsg> Switch<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        todo!()
        // let local_events = Events::default().insert("switch", |conf| {
        //     conf.focus(|_| Msg::Focus)
        //         .blur(|_| Msg::Blur)
        //         .mouse_enter(|_| Msg::MouseEnter)
        //         .mouse_leave(|_| Msg::MouseLeave)
        //         .click(|_| Msg::Click)
        // });

        // Self {
        //     msg_mapper: msg_mapper.into(),
        //     local_events,
        //     events: Events::default(),
        //     user_style: Style::default(),
        //     disabled: false,
        //     focus: false,
        //     mouse_over: false,
        //     toggled: false,
        // }
    }

    fn handle_toggle_msg(&mut self) {
        self.toggled = !self.toggled;
    }
}

impl<PMsg: 'static> Model<PMsg> for Switch<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Click => self.handle_toggle_msg(),
        }
    }
}

impl<PMsg: 'static> Render for Switch<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.switch(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // let button = div!()
        //     .set(att::class("button"))
        //     .set(style["button"])
        //     .try_add(self.events.get("button"));

        // button!()
        //     .add(att::class("switch"))
        //     .add(att::disabled(self.disabled))
        //     .set(&self.local_events["switch"])
        //     .set(style["switch"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("switch"))
        //     .add(button)
    }
}
