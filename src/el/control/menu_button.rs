use crate::{css, prelude::*};
use derive_rich::Rich;
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

// #[derive(Default, Rich)]
// pub struct LocalEvents {
//     #[rich(write(style = compose))]
//     pub background: Events<Msg>,
//     #[rich(write(style = compose))]
//     pub widget: Events<Msg>,
//     #[rich(write(style = compose))]
//     pub content: Events<Msg>,
// }

// #[derive(Rich)]
// pub struct ParentEvents<PMsg> {
//     #[rich(write(style = compose))]
//     pub background: Events<PMsg>,
//     #[rich(write(style = compose))]
//     pub widget: Events<PMsg>,
//     #[rich(write(style = compose))]
//     pub content: Events<PMsg>,
// }

// impl<PMsg> Default for ParentEvents<PMsg> {
//     fn default() -> Self {
//         Self {
//             background: Events::default(),
//             widget: Events::default(),
//             content: Events::default(),
//         }
//     }
// }

#[derive(Rich)]
pub struct MenuButton<PMsg, C> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    // #[rich(read, write(style = compose))]
    // local_events: LocalEvents,
    // #[rich(read, write(style = compose))]
    // events: ParentEvents<PMsg>,
    // #[rich(read, write(style = compose))]
    // user_style: UserStyle,

    // menu button element properties
    #[rich(read, write(style = compose))]
    pub button: Button<Msg>,
    #[rich(read, write(style = compose))]
    pub child: C,
    #[rich(read(rename = is_popedup), write)]
    popup: bool,
}

impl<PMsg, C> MenuButton<PMsg, C> {
    pub fn new(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        label: impl Into<Cow<'static, str>>,
        child: C,
    ) -> Self {
        let button = Button::with_label(Msg::Button, label)
            .and_events(|conf| conf.click(|_| Msg::TogglePopover));

        Self {
            msg_mapper: msg_mapper.into(),
            // local_events: local_events,
            // events: ParentEvents::default(),
            // user_style: UserStyle::default(),
            button,
            child,
            popup: false,
        }
    }
}

impl<GMsg, PMsg, C> Model<PMsg, GMsg> for MenuButton<PMsg, C>
where
    C: Render<PMsg, View = Node<PMsg>>,
    GMsg: 'static,
    PMsg: 'static,
{
    type Message = Msg;

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

impl<PMsg, C> Render<PMsg> for MenuButton<PMsg, C>
where
    PMsg: 'static,
    C: Render<PMsg, View = Node<PMsg>>,
{
    type View = Node<PMsg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, theme: &impl Theme, _: Self::Style) -> Self::View {
        Popover::new(
            &self.button.render(theme).map_msg_with(&self.msg_mapper),
            &self.child,
        )
        .set_visible(self.popup)
        .set_offset(4)
        .render(theme)
    }
}
