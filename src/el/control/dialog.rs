use crate::prelude::*;
use derive_rich::Rich;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
}

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
    user_style: Style,

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
        //     msg_mapper: msg_mapper.into(),
        //     local_events: local_events,
        //     events: Events::default(),
        //     user_style: Style::default(),
        //     header_bar,
        //     child,
        //     disabled: false,
        //     mouse_on_widget: false,
        //     state: State::Closed,
        // }
    }
}

impl<PMsg, C> Model<PMsg> for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: Render<View = Node<PMsg>>,
{
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg>) {
        let mut orders = orders.proxy(self.msg_mapper.map_msg_once());
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
                    orders.perform_cmd(seed::prelude::cmds::timeout(400, || Msg::Close));
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

impl<PMsg, C> Render for Dialog<PMsg, C>
where
    C: Render<View = Node<PMsg>>,
{
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.dialog(self.theme_lens())
    }

    fn render_with_style(&self, theme: &Theme, style: Style) -> Self::View {
        todo!()
        // let content = div!()
        //     .add(att::class("content"))
        //     .set(style["content"])
        //     .set(&self.local_events["content"])
        //     .map_msg_with(&self.msg_mapper)
        //     .add(self.child.render(theme))
        //     .try_add(self.events.get("content"));

        // let dialog = div!()
        //     .add(att::class("dialog"))
        //     .set(style["dialog"])
        //     .set(&self.local_events["dialog"])
        //     .map_msg_with(&self.msg_mapper)
        //     .add(vec![
        //         self.header_bar.render(theme).map_msg_with(&self.msg_mapper),
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
