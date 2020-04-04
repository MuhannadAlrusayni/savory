use crate::{button::ButtonLens, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    // internal messages
    Button(button::Msg),
    TogglePopover,
    // public messages
    Close,
    Show,
}

#[derive(Rich, Element)]
pub struct MenuButton<PMsg, C> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,

    // menu button element properties
    #[rich(read, write(style = compose))]
    #[element(theme_lens(nested))]
    pub button: Button<Msg>,
    #[rich(read, write(style = compose))]
    pub child: C,
    #[rich(read(rename = is_popedup), write)]
    #[element(theme_lens)]
    popup: bool,
}

impl<PMsg, C> MenuButton<PMsg, C> {
    pub fn new(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        label: impl Into<Cow<'static, str>>,
        child: C,
    ) -> Self {
        todo!()
        // let button = Button::with_label(Msg::Button, label)
        //     .and_events(|conf| conf.click(|_| Msg::TogglePopover));

        // Self {
        //     msg_mapper: msg_mapper.into(),
        //     button,
        //     child,
        //     popup: false,
        // }
    }
}

impl<PMsg, GMsg, C> Element<PMsg, GMsg> for MenuButton<PMsg, C>
where
    PMsg: 'static,
    GMsg: 'static,
    C: Render<Output = Node<PMsg>>,
{
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy(self.msg_mapper.map_msg_once());
        match msg {
            Msg::Button(msg) => self.button.update(msg, &mut orders),
            Msg::TogglePopover => self.popup = !self.popup,
            Msg::Show => self.popup = true,
            Msg::Close => self.popup = false,
        }
    }
}

impl<PMsg, C> Render for MenuButton<PMsg, C>
where
    C: Render<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        todo!()
        // Popover::new(
        //     &self.button.render(theme).map_msg_with(&self.msg_mapper),
        //     &self.child,
        // )
        // .set_visible(self.popup)
        // .set_offset(4)
        // .render(theme)
    }
}
