use crate::{label::LabelLens, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

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
    style: Option<Style>,

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

crate::style_type! {
    button,
}

crate::events_type! {
    button,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Button<PMsg> {
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
        }
    }
}

impl<PMsg> Render for Button<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
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

impl<PMsg> Button<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        todo!()
        // let local_events = Events::default().insert("button", |conf| {
        //     conf.focus(|_| Msg::Focus)
        //         .blur(|_| Msg::Blur)
        //         .mouse_enter(|_| Msg::MouseEnter)
        //         .mouse_leave(|_| Msg::MouseLeave)
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
        //     style: None,
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
}

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}
