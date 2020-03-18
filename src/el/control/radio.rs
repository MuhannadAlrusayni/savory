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
    #[rich(read)]
    label: Option<Cow<'static, str>>,
    #[rich(read(
        /// Return `true` if radio element is disabled
        copy, rename = is_disabled),
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
    #[rich(read(
        /// Return `true` if radio element is toggled
        copy, rename = is_toggled
    ))]
    toggled: bool,
}

impl<PMsg> Radio<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let mut local_events = LocalEvents::default();
        local_events
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
        let mut radio = Self::new(msg_mapper);
        radio.set_label(lbl);
        radio
    }

    pub fn set_label(&mut self, val: impl Into<Cow<'static, str>>) -> &mut Self {
        let val = val.into();
        self.label_el_ref
            .get_then(|el| el.set_inner_text(val.as_ref()));
        self.label = Some(val.into());
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.input_el_ref.get_then(|el| el.set_disabled(true));
        self.disabled = true;
        self
    }

    pub fn enable(&mut self) -> &mut Self {
        self.input_el_ref.get_then(|el| el.set_disabled(false));
        self.disabled = false;
        self
    }

    pub fn set_disabled(&mut self, val: bool) -> &mut Self {
        if let Some(input) = self.input_el_ref.get() {
            input.set_disabled(val);
        }
        self.disabled = val;
        self
    }

    pub fn toggle_on(&mut self) -> &mut Self {
        self.toggled = true;
        self
    }

    pub fn toggle_off(&mut self) -> &mut Self {
        self.toggled = false;
        self
    }

    pub fn set_toggle(&mut self, val: bool) -> &mut Self {
        if val {
            self.toggle_on()
        } else {
            self.toggle_off()
        }
    }

    pub fn toggle(&mut self) -> &mut Self {
        self.set_toggle(!self.toggled)
    }

    pub fn handle_toggle_msg(&mut self) {
        self.toggle();
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
        let mut input = input!();
        input
            .el_ref(&self.input_el_ref)
            .and_attributes(|conf| {
                conf.set_class("radio-input")
                    .set_disabled(self.disabled)
                    .set_checked(self.toggled)
                    .set_type(att::Type::Radio)
            })
            .set_style(style.input)
            .set_events(&self.local_events.input);

        // add button div if the radio is toggled
        if self.is_toggled() {
            let mut button = div!();
            button
                .and_attributes(|conf| conf.set_class("radio-button"))
                .set_style(style.button);
            input.add_child(button);
        }

        let mut input = input.map_msg_with(&self.msg_mapper);
        input.add_events(&self.events.input);

        match self.label.as_ref() {
            None => input,
            Some(lbl) => {
                let mut label = label!();
                label
                    .and_attributes(|conf| conf.set_class("radio-label"))
                    .el_ref(&self.label_el_ref)
                    .set_style(style.label)
                    .set_events(&self.local_events.label);

                let mut label = label.map_msg_with(&self.msg_mapper);
                label
                    .add_children(vec![input, plain![lbl.to_string()]])
                    .add_events(&self.events.label);

                label
            }
        }
    }
}
