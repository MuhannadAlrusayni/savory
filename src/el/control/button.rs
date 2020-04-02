use crate::prelude::*;
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Route,
}

#[derive(Rich, Element)]
pub struct Button<PMsg> {
    // general element properties
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,

    // button element properties
    #[rich(write)]
    #[element(theme_lens(nested))]
    pub label: Option<Label<Msg>>,
    #[rich(write)]
    pub icon: Option<Icon<Msg>>,
    #[rich(read(copy), value_fns = {
        /// Change button kind to normal
        normal = Kind::Normal,
        /// Change button kind to suggestion
        suggestion = Kind::Suggestion,
        /// Change button kind to destructive
        destructive = Kind::Destructive,
        /// Change button kind to link
        link = Kind::Link,
        /// Change button kind to dashed
        dashed = Kind::Dashed,
    })]
    #[element(theme_lens)]
    kind: Option<Kind>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    block: bool,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    ghost: bool,
    #[rich(read, write)]
    #[element(theme_lens)]
    route: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled), write, value_fns = { disable = true, enable = false })]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(copy, rename = is_focused), write)]
    #[element(theme_lens)]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over), write)]
    #[element(theme_lens)]
    mouse_over: bool,
}

impl<PMsg> Button<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        todo!()
        // let local_events = Events::default().insert("button", |conf| {
        //     conf.focus(|_| Msg::Focus)
        //         .blur(|_| Msg::Blur)
        //         .mouse_enter(|_| Msg::MouseEnter)
        //         .mouse_leave(|_| Msg::MouseLeave)
        //         .click(|_| Msg::Route)
        // });

        // Button {
        //     el_ref: ElRef::default(),
        //     msg_mapper: msg_mapper.into(),
        //     local_events,
        //     events: Events::default(),
        //     label: None,
        //     icon: None,
        //     kind: None,
        //     block: false,
        //     ghost: false,
        //     user_style: Style::default(),
        //     route: None,
        //     disabled: false,
        //     focus: false,
        //     mouse_over: false,
        // }
    }

    pub fn with_label(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        label: impl Into<Label<Msg>>,
    ) -> Self {
        Button::new(msg_mapper).set_label(label)
    }

    pub fn with_icon(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        icon: impl Into<Icon<Msg>>,
    ) -> Self {
        Button::new(msg_mapper).set_icon(icon)
    }

    fn handle_route_msg(&mut self) {
        if let Some(ref route) = self.route {
            seed::browser::service::routing::push_route(
                route
                    .split('/')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }
}

impl<PMsg: 'static> Model<PMsg> for Button<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Route => self.handle_route_msg(),
        }
    }
}

impl<PMsg> Render for Button<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.button(self.theme_lens())
    }

    fn render_with_style(&self, theme: &Theme, style: Style) -> Self::View {
        todo!()
        // button!()
        //     .add(att::class("button"))
        //     .add(att::disabled(self.disabled))
        //     .set(&self.local_events["button"])
        //     .set(style["button"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("button"))
        //     .try_add(self.icon.as_ref().map(|icon| icon.render(theme)))
        //     .try_add(self.label.as_ref().map(|lbl| lbl.render(theme)))
    }
}
