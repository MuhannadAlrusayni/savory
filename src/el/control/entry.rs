use crate::{css, prelude::*};
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    UpdateText,
}

#[derive(Default, Rich)]
pub struct LocalEvents {
    #[rich(write(style = compose))]
    pub input: Events<Msg>,
    #[rich(write(style = compose))]
    pub container: Events<Msg>,
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(style = compose))]
    pub input: Events<PMsg>,
    #[rich(write(style = compose))]
    pub container: Events<PMsg>,
}

impl<PMsg> Default for ParentEvents<PMsg> {
    fn default() -> Self {
        Self {
            input: Events::default(),
            container: Events::default(),
        }
    }
}

#[derive(Rich)]
pub struct Entry<PMsg> {
    // general element properties
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: LocalEvents,
    #[rich(read, write(style = compose))]
    events: ParentEvents<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

    // entry element properties
    #[rich(read, write)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy), write)]
    max_length: Option<att::MaxLength>,
    #[rich(read, write)]
    placeholder: Option<Cow<'static, str>>,
    #[rich(write, read(
        /// Return `true` if entry element is disabled
        copy, rename = is_disabled
    ))]
    disabled: bool,
    #[rich(read(
        /// Return `true` if entry element is focused
        copy, rename = is_focused
    ))]
    focus: bool,
    #[rich(read(
        /// Return `true` when mouse over entry element
        copy, rename = is_mouse_over
    ))]
    mouse_over: bool,
}

impl<PMsg> Entry<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let local_events = LocalEvents::default().and_input(|conf| {
            conf.focus(|_| Msg::Focus)
                .blur(|_| Msg::Blur)
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave)
                .input(|_| Msg::UpdateText)
        });

        Self {
            el_ref: ElRef::default(),
            msg_mapper: msg_mapper.into(),
            local_events,
            events: ParentEvents::default(),
            user_style: UserStyle::default(),
            text: None,
            max_length: None,
            placeholder: None,
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    pub fn with_placeholder(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        val: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::new(msg_mapper).set_placeholder(val)
    }

    // pub fn set_text(self, text: impl Into<Cow<'static, str>>) -> Self {
    //     let text = text.into();
    //     if let Some(input) = self.el_ref.get() {
    //         input.set_value(text.as_ref());
    //     }
    //     self.text = Some(text);
    //     self
    // }

    // pub fn set_max_length(self, val: impl Into<att::MaxLength>) -> Self {
    //     let val = val.into();
    //     self.el_ref
    //         .get_then(|el| el.set_max_length(val.into_inner()));
    //     self.max_length = Some(val);
    //     self
    // }

    // pub fn set_placeholder(self, val: impl Into<Cow<'static, str>>) -> Self {
    //     let val = val.into();
    //     self.el_ref.get_then(|el| el.set_placeholder(&val));
    //     self.placeholder = Some(val);
    //     self
    // }

    // pub fn disable(self) -> Self {
    //     self.el_ref.get_then(|el| el.set_disabled(true));
    //     self.disabled = true;
    //     self
    // }

    // pub fn enable(self) -> Self {
    //     self.el_ref.get_then(|el| el.set_disabled(false));
    //     self.disabled = false;
    //     self
    // }

    // pub fn set_disabled(self, val: bool) -> Self {
    //     if val {
    //         self.enable()
    //     } else {
    //         self.disable()
    //     }
    // }

    fn handle_update_text(&mut self) {
        if let Some(input) = self.el_ref.get() {
            self.text = Some(input.value().into());
        }
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Entry<PMsg> {
    type Message = Msg;

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

#[derive(Clone, Debug, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
    pub input: css::Style,
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
    pub input: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Entry<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.entry(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let input = input!()
            .set(&self.local_events.input)
            .set(style.input)
            .and_attributes(|conf| {
                conf.set_disabled(self.disabled)
                    .try_set_value(self.text.clone())
                    .try_set_max_length(self.max_length)
                    .try_set_placeholder(self.placeholder.clone())
            })
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.input);

        div!()
            .set(style.container)
            .set(&self.local_events.container)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.container)
            .add(vec![input])
    }
}
