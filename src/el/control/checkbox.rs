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
pub struct Checkbox<PMsg> {
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
    disabled: bool,
    #[rich(read(
        /// Return `true` if checkbox element is focused
        copy, rename = is_focused
    ))]
    focus: bool,
    #[rich(read(
        /// Return `true` when mouse over checkbox element
        copy, rename = is_mouse_over
    ))]
    mouse_over: bool,
    #[rich(
        read(
            /// Return `true` if checkbox element is toggled
            copy, rename = is_toggled
        ),
        value_fns = { toggled = true, toggle_off = false }
    )]
    toggled: bool,
}

impl<PMsg> Checkbox<PMsg> {
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

    fn handle_toggle_msg(&mut self) {
        self.toggled = !self.toggled;
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Checkbox<PMsg> {
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

impl<PMsg: 'static> Render<PMsg> for Checkbox<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.checkbox(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let mut input = input!()
            .el_ref(&self.input_el_ref)
            .and_attributes(|conf| {
                conf.set_class("checkbox-input")
                    .set_disabled(self.disabled)
                    .set_checked(self.toggled)
                    .set_type(att::Type::Checkbox)
            })
            .set_style(style.input)
            .set_events(&self.local_events.input);

        // add button div if the checkbox is toggled
        if self.is_toggled() {
            let button = div!()
                .and_attributes(|conf| conf.set_class("checkbox-button"))
                .set_style(style.button);
            input.add_child(button);
        }

        let input = input
            .map_msg_with(&self.msg_mapper)
            .add_events(&self.events.input);

        match self.label.as_ref() {
            None => input,
            Some(lbl) => label!()
                .and_attributes(|conf| conf.set_class("checkbox-label"))
                .el_ref(&self.label_el_ref)
                .set_style(style.label)
                .set_events(&self.local_events.label)
                .map_msg_with(&self.msg_mapper)
                .add_children(vec![input, plain![lbl.to_string()]])
                .add_events(&self.events.label),
        }
    }
}
