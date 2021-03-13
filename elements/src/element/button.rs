//! Button element
//!
//! Buttons are core element in any app, Savory ships a powerfull button that
//! cover 4 kinds of button:
//! - Default Button
//! - Dashed Button
//! - Text Button
//! - Link Button
//!
//! Button also can have `Suggestion` or `Destructive` (aka `Primary` and
//! `Danger` in web UI World), button can also go ghost.
//!
//! See [`Button`] docs to find out more about its methods.
//!
//! # Usage
//! TODO
//!
//! [`Button`]: crate::prelude::Button

use crate::{id::Id, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_style::prelude::*;
use std::borrow::Cow;

pub enum Msg {
    Rerender,
    Focus(bool),
    MouseOver(bool),
    Disable(bool),
}

/// Button element
#[derive(Rich, Element)]
pub struct Button {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Option<Id>,
    env: Env,

    // button element properties
    #[rich(read)]
    #[element(config)]
    text: Option<Cow<'static, str>>,
    #[rich(read)]
    #[element(config)]
    icon: Option<Svg<Msg>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default), data_lens)]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(data_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(data_lens)]
    mouse_over: bool,

    #[rich(read(copy))]
    #[element(config, data_lens)]
    color: Option<palette::Hsl>,
    #[rich(read(copy))]
    #[element(config, data_lens)]
    text_color: Option<palette::Hsl>,
    #[rich(read(copy))]
    #[element(config(default = "ActionType::Default"), data_lens)]
    action_type: ActionType,
    #[rich(read(copy))]
    #[element(config(default = "Kind::Default"), data_lens)]
    kind: Kind,
    #[rich(read(copy, rename = is_ghost))]
    #[element(config(default, no_pub), data_lens)]
    ghost: bool,
}

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
pub enum ActionType {
    Default,
    Suggested,
    Destructive,
}

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
pub enum Kind {
    Default,
    Dashed,
    TextButton,
    LinkButton,
}

impl Element for Button {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>, env: Env) -> Self {
        orders.subscribe(|_: RerenderRequested| Msg::Rerender);

        Button {
            id: config.id,
            env,
            text: config.text,
            icon: config.icon,
            disabled: config.disabled,
            focused: false,
            mouse_over: false,
            color: config.color,
            text_color: config.text_color,
            action_type: config.action_type,
            kind: config.kind,
            ghost: config.ghost,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Rerender => {}
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::Focus(val) => self.focused = val,
            Msg::Disable(val) => self.disabled = val,
        }
    }
}

impl View<Node<Msg>> for Button {
    fn view(&self) -> Node<Msg> {
        let style = self.env.ds().button(self.data_lens());
        html::button()
            .class("button")
            .try_id(self.id.clone())
            .disabled(self.disabled)
            .style(style)
            .on_focus(|_| Msg::Focus(true))
            .on_blur(|_| Msg::Focus(false))
            .on_mouse_over(|_| Msg::MouseOver(true))
            .on_mouse_leave(|_| Msg::MouseOver(false))
            .try_push(self.icon.as_ref().map(|el| el.view()))
            .try_push(self.text.clone())
    }
}

impl Config {
    pub fn suggestion(mut self) -> Self {
        self.action_type = ActionType::Suggested;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.action_type = ActionType::Destructive;
        self
    }

    pub fn dashed(mut self) -> Self {
        self.kind = Kind::Dashed;
        self
    }

    pub fn text_button(mut self) -> Self {
        self.kind = Kind::TextButton;
        self
    }

    pub fn link_button(mut self) -> Self {
        self.kind = Kind::LinkButton;
        self
    }

    pub fn ghost(mut self) -> Self {
        self.ghost = true;
        self
    }
}
