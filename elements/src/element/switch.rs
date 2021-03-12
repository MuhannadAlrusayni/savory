//! Switch element
//!
//! Switchs are common elements in most apps, Savory ships switch element that covers
//! common usages.
//!
//! Switch accept these values on it's initialization:
//! - toggled: initial value
//! - text: text descripe the switch
//! - color: color used when the switch toggled
//! - checkbox like: this tells the switch element to look like checkbox instead
//!
//! See [`Switch`] docs to find out more about its methods.
//!
//! # Usage
//! TODO
//!
//! [`Switch`]: crate::prelude::Switch

use crate::{id::Id, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_style::{self as style, prelude::*};
use std::borrow::Cow;

pub enum Msg {
    Rerender,
    Focus(bool),
    MouseOver(bool),
    Disable(bool),
    Toggled(bool),
    Toggle,
}

#[derive(Rich, Element)]
#[element(style_map(switch, check_sign, text))]
pub struct Switch {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Option<Id>,
    env: Env,

    // switch element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(config(default), data_lens(copy))]
    toggled: bool,
    #[rich(read)]
    #[element(config)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default), data_lens(copy))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(data_lens(copy))]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(data_lens(copy))]
    mouse_over: bool,

    #[rich(read(copy))]
    #[element(config, data_lens(copy))]
    pub color: Option<style::Color>,
    #[rich(write)]
    #[element(config(default), data_lens(copy))]
    pub checkbox_like: bool,
}

impl Element for Switch {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>, env: Env) -> Self {
        orders.subscribe(|_: RerenderRequested| Msg::Rerender);

        Self {
            id: config.id,
            env,
            text: config.text,
            toggled: config.toggled,
            disabled: config.disabled,
            focused: false,
            mouse_over: false,
            color: config.color,
            checkbox_like: config.checkbox_like,
        }
    }

    fn update(&mut self, msg: Msg, _orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Rerender => {}
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::Focus(val) => self.focused = val,
            Msg::Disable(val) => self.disabled = val,
            Msg::Toggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
        }
    }
}

impl View<Node<Msg>> for Switch {
    fn view(&self) -> Node<Msg> {
        let style_map = self.env.ds().switch(self.data_lens());
        let switch = html::button()
            .class("switch")
            .style(style_map.switch)
            .disabled(self.disabled)
            // .checked(self.toggled)
            // .type_("switch")
            .on_focus(|_| Msg::Focus(true))
            .on_blur(|_| Msg::Focus(false))
            .on_mouse_enter(|_| Msg::MouseOver(true))
            .on_mouse_leave(|_| Msg::MouseOver(false))
            .on_click(|_| Msg::Toggle)
            // add `check-sign` node
            .set(html::div().class("check-sign").style(style_map.check_sign));

        match self.text.as_ref() {
            None => switch.try_id(self.id.clone()),
            Some(lbl) => html::label()
                .try_id(self.id.clone())
                .class("text")
                .style(style_map.text)
                .push(switch)
                .push(lbl.clone())
                .on_mouse_enter(|_| Msg::MouseOver(true))
                .on_mouse_leave(|_| Msg::MouseOver(false)),
        }
    }
}
