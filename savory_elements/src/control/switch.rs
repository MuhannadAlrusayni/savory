use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

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
    style: Option<Style>,

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

crate::style_type! {
    button,
    switch,
}

crate::events_type! {
    button,
    switch,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Click,
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Switch<PMsg> {
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        todo!()
    }

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

impl<PMsg: 'static> Render for Switch<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
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
        //     style: None,
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
