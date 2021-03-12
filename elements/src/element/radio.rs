//! Radio element
//!
//! Radios are common elements in most apps, Savory ships radio element that
//! covers common usages.
//!
//! Radio accept these values on it's initialization:
//! - toggled: initial value
//! - text: text descripe the radio
//! - color: color used when the radio toggled
//!
//! See [`Radio`] docs to find out more about its methods.
//!
//! # Usage
//! TODO
//!
//! [`Radio`]: crate::prelude::Radio

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
#[element(style_map(radio, check_sign, text))]
pub struct Radio {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Option<Id>,
    env: Env,

    // radio element properties
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
    color: Option<style::Color>,
}

impl Element for Radio {
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
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
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

impl View<Node<Msg>> for Radio {
    fn view(&self) -> Node<Msg> {
        let style_map = self.env.ds().radio(self.data_lens());
        let radio = html::button()
            .class("radio")
            .style(style_map.radio)
            .disabled(self.disabled)
            // .checked(self.toggled)
            // .type_("radio")
            .on_focus(|_| Msg::Focus(true))
            .on_blur(|_| Msg::Focus(false))
            .on_mouse_enter(|_| Msg::MouseOver(true))
            .on_mouse_leave(|_| Msg::MouseOver(false))
            .on_click(|_| Msg::Toggle)
            // push check-sign node
            .set(html::div().class("check-sign").style(style_map.check_sign));

        match self.text.as_ref() {
            None => radio.try_id(self.id.clone()),
            Some(lbl) => html::label()
                .try_id(self.id.clone())
                .class("text")
                .style(style_map.text)
                .push(radio)
                .push(lbl.clone())
                .on_mouse_enter(|_| Msg::MouseOver(true))
                .on_mouse_leave(|_| Msg::MouseOver(false)),
        }
    }
}
