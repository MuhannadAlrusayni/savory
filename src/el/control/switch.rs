use crate::{css, prelude::*};
use derive_rich::Rich;

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Click,
}

#[derive(Rich)]
pub struct Switch<PMsg> {
    // general element properties
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

    // switch element properties
    #[rich(
        read(copy, rename = is_disabled),
    )]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
    #[rich(
        read(copy, rename = is_toggled),
    )]
    toggled: bool,
}

impl<PMsg> Switch<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let mut local_events = Events::default();
        local_events
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Click);

        Self {
            el_ref: ElRef::default(),
            msg_mapper: msg_mapper.into(),
            local_events,
            events: Events::default(),
            user_style: UserStyle::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
            toggled: false,
        }
    }

    pub fn disable(&mut self) -> &mut Self {
        self.el_ref.get_then(|el| el.set_disabled(true));
        self.disabled = true;
        self
    }

    pub fn enable(&mut self) -> &mut Self {
        self.el_ref.get_then(|el| el.set_disabled(false));
        self.disabled = true;
        self
    }

    pub fn set_disabled(&mut self, val: bool) -> &mut Self {
        self.el_ref.get_then(|el| el.set_disabled(val));
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

    fn handle_toggle_msg(&mut self) {
        self.toggle();
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Switch<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Click => self.handle_toggle_msg(),
        }
    }
}

#[derive(Clone, Debug, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub background: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub background: css::Style,
    #[rich(write(style = compose))]
    pub button: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for Switch<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.switch(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let mut button = div!();
        button
            .and_attributes(|conf| conf.set_class("switch-button"))
            .set_style(style.button);

        let mut switch = button!();
        switch
            .set_events(&self.local_events)
            .set_style(style.background)
            .and_attributes(|conf| conf.set_class("switch").set_disabled(self.disabled))
            .add_child(button);

        let mut switch = switch.map_msg_with(&self.msg_mapper);
        switch.add_events(&self.events);
        switch
    }
}
