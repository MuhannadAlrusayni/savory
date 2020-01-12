use crate::{
    css::{self, unit::px, St, Style},
    macros::*,
    model::Model,
    propertie::Size,
    render::Render,
    theme::{Theme, Themeable},
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Click,
}

#[derive(Debug, Clone, Rich)]
pub struct Radio {
    #[rich(write(take))]
    pub label: Option<Cow<'static, str>>,
    #[rich(write(take, style = compose))]
    pub style: Style,
    #[rich(value_fns(take) = {
        small = Size::Small,
        medium = Size::Medium,
        large = Size::Large,
    })]
    pub size: Option<Size>,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns(take) = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
    #[rich(
        read(copy, rename = is_toggled),
        value_fns(take) = { toggle_on = true, toggle_off = false }
    )]
    toggle: bool,
}

impl Default for Radio {
    fn default() -> Self {
        Self::new()
    }
}

impl Radio {
    pub fn new() -> Self {
        Self {
            label: None,
            style: Style::default(),
            size: None,
            disabled: false,
            focus: false,
            mouse_over: false,
            toggle: false,
        }
    }

    pub fn toggle(mut self) -> Self {
        self.toggle = !self.toggle;
        self
    }

    pub fn handle_toggle_msg(&mut self) {
        self.toggle = !self.toggle;
    }
}

impl<GMsg: 'static> Model<Msg, GMsg> for Radio {
    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg, GMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Click => self.handle_toggle_msg(),
        }
    }
}

impl Render<Msg> for Radio {
    type View = Node<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let (input_style, btn_style, lbl_style) = theme.radio(self);

        let input = input![
            attrs![
                At::Disabled => self.disabled.as_at_value(),
                At::Checked => self.toggle.as_at_value(),
                At::Type => "radio",
            ],
            input_style,
            simple_ev(Ev::Focus, Msg::Focus),
            simple_ev(Ev::Blur, Msg::Blur),
            simple_ev(Ev::MouseEnter, Msg::MouseEnter),
            simple_ev(Ev::MouseLeave, Msg::MouseLeave),
            simple_ev(Ev::Click, Msg::Click),
            if self.is_toggled() {
                div![btn_style]
            } else {
                empty![]
            },
        ];

        if let Some(ref lbl) = self.label {
            label![
                lbl_style,
                input,
                lbl.to_string(),
                simple_ev(Ev::MouseEnter, Msg::MouseEnter),
                simple_ev(Ev::MouseLeave, Msg::MouseLeave),
            ]
        } else {
            input
        }
    }
}

impl Themeable for Radio {
    type StyleMap = (Style, Style, Style);
}
