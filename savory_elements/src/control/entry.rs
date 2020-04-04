use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
pub struct Entry<PMsg> {
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

    // entry element properties
    #[rich(read, write)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    max_length: Option<att::MaxLength>,
    #[rich(read, write)]
    #[element(theme_lens)]
    placeholder: Option<Cow<'static, str>>,
    #[rich(write, read(
        /// Return `true` if entry element is disabled
        copy, rename = is_disabled
    ))]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(
        /// Return `true` if entry element is focused
        copy, rename = is_focused
    ))]
    #[element(theme_lens)]
    focus: bool,
    #[rich(read(
        /// Return `true` when mouse over entry element
        copy, rename = is_mouse_over
    ))]
    #[element(theme_lens)]
    mouse_over: bool,
}

crate::style_type! {
    input,
    container,
}

crate::events_type! {
    input,
    container,
}

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    UpdateText,
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Entry<PMsg> {
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::UpdateText => self.handle_update_text(),
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
        }
    }
}

impl<PMsg> Render for Entry<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        todo!()
        // let input = input!()
        //     .set(&self.local_events["input"])
        //     .set(style["input"])
        //     .and_attributes(|conf| {
        //         conf.set_disabled(self.disabled)
        //             .try_set_value(self.text.clone())
        //             .try_set_max_length(self.max_length)
        //             .try_set_placeholder(self.placeholder.clone())
        //     })
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("input"));

        // div!()
        //     .set(style["container"])
        //     .set(&self.local_events["container"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("container"))
        //     .add(vec![input])
    }
}

impl<PMsg> Entry<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        todo!()
        // let local_events = Events::default().insert("input", |conf| {
        //     conf.focus(|_| Msg::Focus)
        //         .blur(|_| Msg::Blur)
        //         .mouse_enter(|_| Msg::MouseEnter)
        //         .mouse_leave(|_| Msg::MouseLeave)
        //         .input(|_| Msg::UpdateText)
        // });

        // Self {
        //     el_ref: ElRef::default(),
        //     msg_mapper: msg_mapper.into(),
        //     local_events,
        //     events: Events::default(),
        //     style: None,
        //     text: None,
        //     max_length: None,
        //     placeholder: None,
        //     disabled: false,
        //     focus: false,
        //     mouse_over: false,
        // }
    }

    pub fn with_placeholder(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        val: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::new(msg_mapper).set_placeholder(val)
    }

    fn handle_update_text(&mut self) {
        if let Some(input) = self.el_ref.get() {
            self.text = Some(input.value().into());
        }
    }
}
