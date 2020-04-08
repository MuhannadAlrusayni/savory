use crate::{header_bar::HeaderBarLens, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
pub struct Dialog<PMsg, C> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,

    // dialog element properties
    #[rich(read, write(style = compose))]
    #[element(theme_lens(nested))]
    header_bar: HeaderBar<Msg>,
    #[rich(read, write(style = compose))]
    pub child: C,
    #[rich(
        read(copy, rename = is_disabled),
    )]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(copy, rename = is_mouse_on_widget))]
    #[element(theme_lens)]
    mouse_on_widget: bool,
    #[rich(read(copy), value_fns = {
        open = State::Opened,
        close = State::Closed,
    })]
    #[element(theme_lens)]
    state: State,
}

crate::style_type! {
    dialog,
    dialog_background,
    content,
}

crate::events_type! {
    dialog,
    dialog_background,
    content,
}

pub enum Msg {
    // internal messages
    MouseEnterWidget,
    MouseLeaveWidget,
    ClickedOutside,
    CloseButton(button::Msg),
    // public messages
    Close,
    Show,
}

impl<PMsg, GMsg, C> Element<PMsg, GMsg> for Dialog<PMsg, C>
where
    PMsg: 'static,
    GMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);
        match msg {
            Msg::CloseButton(msg) => {
                if let Some(ref mut btn) = self.header_bar.close_button {
                    btn.update(msg, &mut orders)
                }
            }
            Msg::MouseEnterWidget => {
                self.mouse_on_widget = true;
                orders.skip();
            }
            Msg::MouseLeaveWidget => {
                self.mouse_on_widget = false;
                orders.skip();
            }
            Msg::ClickedOutside => {
                if !self.mouse_on_widget {
                    orders.skip().send_msg(Msg::Close);
                }
            }
            Msg::Close => match self.state {
                State::Closed => {}
                State::Opened | State::Opening => {
                    self.state = State::Closing;
                    orders.perform_cmd_after(400, || Msg::Close);
                }
                State::Closing => {
                    self.state = State::Closed;
                }
            },
            Msg::Show => match self.state {
                State::Opened => {}
                State::Closed | State::Closing => {
                    self.state = State::Opening;
                    orders.after_next_render(|_| Msg::Show);
                }
                State::Opening => {
                    self.state = State::Opened;
                }
            },
        }
    }
}

impl<PMsg, C> View for Dialog<PMsg, C>
where
    C: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        todo!()
        // let content = div!()
        //     .add(att::class("content"))
        //     .set(style["content"])
        //     .set(&self.local_events["content"])
        //     .map_msg_with(&self.msg_mapper)
        //     .add(self.child.view(theme))
        //     .try_add(self.events.get("content"));

        // let dialog = div!()
        //     .add(att::class("dialog"))
        //     .set(style["dialog"])
        //     .set(&self.local_events["dialog"])
        //     .map_msg_with(&self.msg_mapper)
        //     .add(vec![
        //         self.header_bar.view(theme).map_msg_with(&self.msg_mapper),
        //         content,
        //     ])
        //     .try_add(self.events.get("dialog"));

        // div!()
        //     .add(att::class("dialog-background"))
        //     .set(style["dialog-background"])
        //     .set(&self.local_events["dialog-background"])
        //     .map_msg_with(&self.msg_mapper)
        //     .add(dialog)
        //     .try_add(self.events.get("dialog-background"))
    }
}

impl<PMsg, C> Dialog<PMsg, C> {
    pub fn new(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        title: impl Into<Label<Msg>>,
        child: C,
    ) -> Self {
        todo!()
        // let local_events = Events::default()
        //     .insert("background", |conf| conf.click(|_| Msg::ClickedOutside))
        //     .insert("widget", |conf| {
        //         conf.mouse_enter(|_| Msg::MouseEnterWidget)
        //             .mouse_leave(|_| Msg::MouseLeaveWidget)
        //     });

        // let header_bar = HeaderBar::new()
        //     .set_close_button(
        //         Button::with_label(Msg::CloseButton, "X")
        //             .and_events(|conf| conf.insert("button", |conf| conf.click(|_| Msg::Close))),
        //     )
        //     .set_title(title);

        // Self {
        //     msg_mapper: msg_mapper,
        //     local_events: local_events,
        //     events: Events::default(),
        //     style: None,
        //     header_bar,
        //     child,
        //     disabled: false,
        //     mouse_on_widget: false,
        //     state: State::Closed,
        // }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
}
