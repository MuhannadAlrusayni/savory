use crate::{
    css::Style,
    el::prelude::*,
    model::Model,
    propertie::{Shape, Size},
    render::Render,
    theme::{Theme, Themeable},
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Clear,
    UpdateText(String),
}

#[derive(Debug, Rich)]
pub struct Entry {
    #[rich(write(take))]
    pub text: Option<String>,
    #[rich(
        read(copy, rename = is_readonly),
        value_fns(take) = { readonly = true, readonly_off = false }
    )]
    pub readonly: bool,
    #[rich(write(take))]
    pub max_length: Option<usize>,
    #[rich(write(take))]
    pub placeholder: Option<String>,
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
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}

impl Entry {
    pub fn new() -> Self {
        Self {
            text: None,
            readonly: false,
            max_length: None,
            placeholder: None,
            style: Style::default(),
            size: None,
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    fn handle_clear_msg(&mut self) {
        self.text = None;
    }
}

impl<GMsg: 'static> Model<Msg, GMsg> for Entry {
    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg, GMsg>) {
        match msg {
            Msg::UpdateText(text) => self.text = Some(text),
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Clear => self.handle_clear_msg(),
        }
    }
}

impl<ParentMsg: 'static> Render<Msg, ParentMsg> for Entry {
    type View = Node<ParentMsg>;

    fn render(
        &self,
        theme: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        let style = theme.entry(self);
        div![
            style.container,
            input![
                style.input,
                attrs![
                    At::Disabled => self.disabled.as_at_value(),
                    At::Value => self.text.as_ref().map(|v| AtValue::Some(v.clone())).unwrap_or(AtValue::Ignored),
                    // At::MaxLength => self.max_length,
                    // At::Placeholder => self.placeholder,
                ],
                simple_ev(Ev::Focus, Msg::Focus),
                simple_ev(Ev::Blur, Msg::Blur),
                simple_ev(Ev::MouseEnter, Msg::MouseEnter),
                simple_ev(Ev::MouseLeave, Msg::MouseLeave),
                input_ev(Ev::Input, Msg::UpdateText)
            ],
        ].map_msg(map_msg)
    }
}

pub struct StyleMap {
    pub container: Style,
    pub input: Style,
}

impl Themeable for Entry {
    type StyleMap = StyleMap;
}
