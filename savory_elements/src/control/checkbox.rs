use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
pub struct Checkbox<PMsg> {
    // general element properties
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,

    // checkbox element properties
    #[rich(read, write)]
    label: Option<Cow<'static, str>>,
    #[rich(
        write,
        value_fns = { enable = false, disable = true },
        read(
            /// Return `true` if checkbox element is disabled
            copy, rename = is_disabled
        )
    )]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(
        /// Return `true` if checkbox element is focused
        copy, rename = is_focused
    ))]
    #[element(theme_lens)]
    focus: bool,
    #[rich(read(
        /// Return `true` when mouse over checkbox element
        copy, rename = is_mouse_over
    ))]
    #[element(theme_lens)]
    mouse_over: bool,
    #[rich(
        read(
            /// Return `true` if checkbox element is toggled
            copy, rename = is_toggled
        ),
        value_fns = { toggled = true, toggle_off = false }
    )]
    #[element(theme_lens)]
    toggled: bool,
}

crate::style_type! {
    checkbox,
    button,
    label,
}

crate::events_type! {
    checkbox,
    button,
    label,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Toggle,
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Checkbox<PMsg> {
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
            Msg::Toggle => self.handle_toggle_msg(),
        }
    }
}

impl<PMsg> Render for Checkbox<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        todo!()
        // let input = input!()
        //     .set(att::class("checbox"))
        //     .set(att::disabled(self.disabled))
        //     .set(att::checked(self.toggled))
        //     .set(att::Type::Checkbox)
        //     .set(style["checkbox"])
        //     .set(&self.local_events["checkbox"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("input"))
        //     .el_ref(&self.input_el_ref)
        //     // add button if the checkbox is toggled
        //     .config_if(self.is_toggled(), |conf| {
        //         let button = div!()
        //             .add(att::class("checkbox-button"))
        //             .set(style["checkbox-button"])
        //             .map_msg_with(&self.msg_mapper)
        //             .try_add(self.events.get("checkbox-button"));
        //         conf.add(button)
        //     });

        // match self.label.as_ref() {
        //     None => input,
        //     Some(lbl) => label!()
        //         .add(att::class("checkbox-label"))
        //         .set(style["label"])
        //         .set(&self.local_events["label"])
        //         .map_msg_with(&self.msg_mapper)
        //         .try_add(self.events.get("label"))
        //         .add(vec![input, plain![lbl.to_string()]])
        //         .el_ref(&self.label_el_ref),
        // }
    }
}

impl<PMsg> Checkbox<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        todo!()
        // let local_events = Events::default()
        //     .insert("input", |conf| {
        //         conf.focus(|_| Msg::Focus)
        //             .blur(|_| Msg::Blur)
        //             .mouse_enter(|_| Msg::MouseEnter)
        //             .mouse_leave(|_| Msg::MouseLeave)
        //             .click(|_| Msg::Toggle)
        //     })
        //     .insert("label", |conf| {
        //         conf.mouse_enter(|_| Msg::MouseEnter)
        //             .mouse_leave(|_| Msg::MouseLeave)
        //     });

        // Self {
        //     input_el_ref: ElRef::default(),
        //     label_el_ref: ElRef::default(),
        //     msg_mapper: msg_mapper.into(),
        //     local_events,
        //     events: Events::default(),
        //     label: None,
        //     style: None,
        //     disabled: false,
        //     focus: false,
        //     mouse_over: false,
        //     toggled: false,
        // }
    }

    pub fn with_label(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        lbl: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::new(msg_mapper).set_label(lbl)
    }

    fn handle_toggle_msg(&mut self) {
        self.toggled = !self.toggled;
    }
}
