use crate::{css, prelude::*};
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Toggle,
}

#[derive(Rich, Element)]
pub struct Radio<PMsg> {
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
    user_style: Style,

    // radio element properties
    #[rich(read, write)]
    label: Option<Cow<'static, str>>,
    #[rich(
        read(
            /// Return `true` if radio element is disabled
            copy, rename = is_disabled
        ),
        write,
        value_fns = { enable = false, disable = true }
    )]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(
        /// Return `true` if radio element is focused
        copy, rename = is_focused
    ))]
    #[element(theme_lens)]
    focus: bool,
    #[rich(read(
        /// Return `true` when mouse over radio element
        copy, rename = is_mouse_over
    ))]
    #[element(theme_lens)]
    mouse_over: bool,
    #[rich(
        read(
            /// Return `true` if radio element is toggled
            copy, rename = is_toggled
        ),
        write,
        value_fns = { toggled = true, toggle_off = false }
    )]
    #[element(theme_lens)]
    toggled: bool,
}

impl<PMsg> Radio<PMsg> {
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
        //     user_style: Style::default(),
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

    // pub fn set_label(self, val: impl Into<Cow<'static, str>>) -> Self {
    //     let val = val.into();
    //     self.label_el_ref
    //         .get_then(|el| el.set_inner_text(val.as_ref()));
    //     self.label = Some(val.into());
    //     self
    // }

    // pub fn disable(self) -> Self {
    //     self.input_el_ref.get_then(|el| el.set_disabled(true));
    //     self.disabled = true;
    //     self
    // }

    // pub fn enable(self) -> Self {
    //     self.input_el_ref.get_then(|el| el.set_disabled(false));
    //     self.disabled = false;
    //     self
    // }

    // pub fn set_disabled(self, val: bool) -> Self {
    //     if let Some(input) = self.input_el_ref.get() {
    //         input.set_disabled(val);
    //     }
    //     self.disabled = val;
    //     self
    // }

    // pub fn toggle_on(self) -> Self {
    //     self.toggled = true;
    //     self
    // }

    // pub fn toggle_off(self) -> Self {
    //     self.toggled = false;
    //     self
    // }

    // pub fn set_toggle(self, val: bool) -> Self {
    //     if val {
    //         self.toggle_on()
    //     } else {
    //         self.toggle_off()
    //     }
    // }

    // pub fn toggle(self) -> Self {
    //     self.set_toggle(!self.toggled)
    // }

    pub fn handle_toggle_msg(&mut self) {
        self.toggled = !self.toggled;
    }
}

impl<PMsg: 'static> Model<PMsg> for Radio<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Toggle => self.handle_toggle_msg(),
        }
    }
}

impl<PMsg> Render for Radio<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.radio(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // let mut input = input!()
        //     .set(att::class("radio"))
        //     .set(att::disabled(self.disabled))
        //     .set(att::checked(self.toggled))
        //     .set(att::Type::Radio)
        //     .set(style["radio"])
        //     .set(&self.local_events["radio"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("radio"))
        //     .el_ref(&self.input_el_ref)
        //     // add button div if the radio is toggled
        //     .config_if(self.is_toggled(), |conf| {
        //         let button = div!()
        //             .add(att::class("radio-button"))
        //             .set(style["radio-button"])
        //             .map_msg_with(&self.msg_mapper)
        //             .try_add(self.events.get("radio-button"));
        //         conf.add(button)
        //     });

        // match self.label.as_ref() {
        //     None => input,
        //     Some(lbl) => label!()
        //         .add(att::class("radio-label"))
        //         .set(style["label"])
        //         .set(&self.local_events["label"])
        //         .map_msg_with(&self.msg_mapper)
        //         .try_add(self.events.get("label"))
        //         .add(vec![input, plain![lbl.to_string()]])
        //         .el_ref(&self.label_el_ref),
        // }
    }
}
