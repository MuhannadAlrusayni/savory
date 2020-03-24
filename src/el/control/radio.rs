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

#[derive(Default, Rich)]
pub struct LocalEvents {
    #[rich(write(style = compose))]
    pub label: Events<Msg>,
    #[rich(write(style = compose))]
    pub input: Events<Msg>,
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(style = compose))]
    pub label: Events<PMsg>,
    #[rich(write(style = compose))]
    pub input: Events<PMsg>,
}

impl<PMsg> Default for ParentEvents<PMsg> {
    fn default() -> Self {
        Self {
            label: Events::default(),
            input: Events::default(),
        }
    }
}

#[derive(Rich)]
pub struct Radio<PMsg> {
    // general element properties
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: LocalEvents,
    #[rich(read, write(style = compose))]
    events: ParentEvents<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

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
    disabled: bool,
    #[rich(read(
        /// Return `true` if radio element is focused
        copy, rename = is_focused
    ))]
    focus: bool,
    #[rich(read(
        /// Return `true` when mouse over radio element
        copy, rename = is_mouse_over
    ))]
    mouse_over: bool,
    #[rich(
        read(
            /// Return `true` if radio element is toggled
            copy, rename = is_toggled
        ),
        write,
        value_fns = { toggled = true, toggle_off = false }
    )]
    toggled: bool,
}

impl<PMsg> Radio<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let local_events = LocalEvents::default()
            .and_input(|conf| {
                conf.focus(|_| Msg::Focus)
                    .blur(|_| Msg::Blur)
                    .mouse_enter(|_| Msg::MouseEnter)
                    .mouse_leave(|_| Msg::MouseLeave)
                    .click(|_| Msg::Toggle)
            })
            .and_label(|conf| {
                conf.mouse_enter(|_| Msg::MouseEnter)
                    .mouse_leave(|_| Msg::MouseLeave)
            });

        Self {
            input_el_ref: ElRef::default(),
            label_el_ref: ElRef::default(),
            msg_mapper: msg_mapper.into(),
            local_events,
            events: ParentEvents::default(),
            label: None,
            user_style: UserStyle::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
            toggled: false,
        }
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

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Radio<PMsg> {
    type Message = Msg;

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

#[derive(Clone, Debug, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub input: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
    #[rich(write(style = compose))]
    pub label: css::Style,
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub input: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
    #[rich(write(style = compose))]
    pub label: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Radio<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.radio(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let mut input = input!()
            .el_ref(&self.input_el_ref)
            .set(style.input)
            .set(&self.local_events.input)
            .and_attributes(|conf| {
                conf.set_class("radio-input")
                    .set_disabled(self.disabled)
                    .set_checked(self.toggled)
                    .set_type(att::Type::Radio)
            });

        // add button div if the radio is toggled
        if self.is_toggled() {
            let button = div!().set(style.button).add(att::class("radio-button"));
            input.add_child(button);
        }

        let input = input.map_msg_with(&self.msg_mapper).add(&self.events.input);

        match self.label.as_ref() {
            None => input,
            Some(lbl) => label!()
                .el_ref(&self.label_el_ref)
                .add(att::class("radio-label"))
                .set(style.label)
                .set(&self.local_events.label)
                .map_msg_with(&self.msg_mapper)
                .add(vec![input, plain![lbl.to_string()]])
                .add(&self.events.label),
        }
    }
}
