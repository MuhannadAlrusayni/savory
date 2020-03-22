use crate::{css, prelude::*};
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

#[derive(Default, Rich)]
pub struct LocalEvents {
    #[rich(write(style = compose))]
    pub background: Events<Msg>,
    #[rich(write(style = compose))]
    pub widget: Events<Msg>,
    #[rich(write(style = compose))]
    pub content: Events<Msg>,
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(style = compose))]
    pub background: Events<PMsg>,
    #[rich(write(style = compose))]
    pub widget: Events<PMsg>,
    #[rich(write(style = compose))]
    pub content: Events<PMsg>,
}

impl<PMsg> Default for ParentEvents<PMsg> {
    fn default() -> Self {
        Self {
            background: Events::default(),
            widget: Events::default(),
            content: Events::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
}

#[derive(Rich)]
pub struct Dialog<PMsg, C> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: LocalEvents,
    #[rich(read, write(style = compose))]
    events: ParentEvents<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

    // dialog element properties
    #[rich(read, write(style = compose))]
    header_bar: HeaderBar<Msg>,
    #[rich(read, write(style = compose))]
    pub child: C,
    #[rich(
        read(copy, rename = is_disabled),
    )]
    disabled: bool,
    // #[rich(read(copy, rename = is_visible))]
    // visible: bool,
    #[rich(read(copy, rename = is_mouse_on_widget))]
    mouse_on_widget: bool,
    // #[rich(read(copy, rename = is_display_none))]
    // display_none: bool,
    #[rich(read(copy), value_fns = {
        open = State::Opened,
        close = State::Closed,
    })]
    state: State,
}

impl<PMsg, C> Dialog<PMsg, C> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>, child: C) -> Self {
        let local_events = LocalEvents::default()
            .and_background(|conf| conf.click(|_| Msg::ClickedOutside))
            .and_widget(|conf| {
                conf.mouse_enter(|_| Msg::MouseEnterWidget)
                    .mouse_leave(|_| Msg::MouseLeaveWidget)
            });

        let header_bar = HeaderBar::new().set_close_button(
            Button::with_label(Msg::CloseButton, "X").and_events(|conf| conf.click(|_| Msg::Close)),
        );

        Self {
            msg_mapper: msg_mapper.into(),
            local_events: local_events,
            events: ParentEvents::default(),
            user_style: UserStyle::default(),
            header_bar,
            child,
            disabled: false,
            mouse_on_widget: false,
            // visible: false,
            // display_none: true,
            state: State::Closed,
        }
    }
}

impl<GMsg, PMsg, C> Model<PMsg, GMsg> for Dialog<PMsg, C>
where
    C: Render<PMsg, View = Node<PMsg>>,
    GMsg: 'static,
    PMsg: 'static,
{
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
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

#[derive(Clone, Debug, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub background: css::Style,
    #[rich(write(style = compose))]
    pub widget: flexbox::Style,
    #[rich(write(style = compose))]
    pub content: flexbox::Style,
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub background: css::Style,
    #[rich(write(style = compose))]
    pub widget: flexbox::Style,
    #[rich(write(style = compose))]
    pub content: flexbox::Style,
}

impl<PMsg, C> Render<PMsg> for Dialog<PMsg, C>
where
    PMsg: 'static,
    C: Render<PMsg, View = Node<PMsg>>,
{
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.dialog(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        let content = div!()
            .set_style(style.content)
            .set_events(&self.local_events.content)
            .map_msg_with(&self.msg_mapper)
            .add_children(vec![self.child.render(theme)])
            .add_events(&self.events.content);

        let widget = div!()
            .set_style(style.widget)
            .set_events(&self.local_events.widget)
            .map_msg_with(&self.msg_mapper)
            .add_children(vec![
                self.header_bar.render(theme).map_msg_with(&self.msg_mapper),
                content,
            ])
            .add_events(&self.events.widget);

        div!()
            .set_style(style.background)
            .set_events(&self.local_events.background)
            .map_msg_with(&self.msg_mapper)
            .add_children(vec![widget])
            .add_events(&self.events.background)
    }
}
