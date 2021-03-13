//! TextInput element
//!
//! Text input are core element in any app, Savory ships text input that covers
//! the common usages for text input.
//!
//! TextInput accept 3 values on it's initialization:
//! - text: initial value
//! - placeholder
//! - max length: this limits the numbers of characters in the input
//! - color: changes the text input color
//! - text color: changes the text color
//!
//! See [`TextInput`] docs to find out more about its methods.
//!
//! # Usage
//! TODO
//!
//! [`TextInput`]: crate::prelude::TextInput

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
    Text(Cow<'static, str>),
    Clear,
    ResyncText,
}

#[derive(Rich, Element)]
pub struct TextInput {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Option<Id>,
    el_ref: ElRef<web_sys::HtmlInputElement>,
    env: Env,

    // entry element properties
    #[rich(read)]
    #[element(config)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy))]
    #[element(config)]
    max_length: Option<i32>,
    #[rich(read)]
    #[element(config)]
    placeholder: Option<Cow<'static, str>>,
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
    color: Option<style::Color>,
    #[rich(read(copy))]
    #[element(config, data_lens)]
    text_color: Option<style::Color>,
}

impl Element for TextInput {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>, env: Env) -> Self {
        orders.subscribe(|_: RerenderRequested| Msg::Rerender);

        Self {
            id: config.id,
            el_ref: ElRef::default(),
            env,
            text: config.text,
            max_length: config.max_length,
            placeholder: config.placeholder,
            disabled: config.disabled,
            focused: false,
            mouse_over: false,
            color: config.color,
            text_color: config.text_color,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Rerender => {}
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::Focus(val) => self.focused = val,
            Msg::Disable(val) => self.disabled = val,
            Msg::Clear => self.text = None,
            Msg::Text(val) => self.text = Some(val),
            Msg::ResyncText => {
                if let Some(input) = self.el_ref.get() {
                    self.text = Some(input.value().into());
                }
            }
        }
    }
}

impl View<Node<Msg>> for TextInput {
    fn view(&self) -> Node<Msg> {
        let style = self.env.ds().text_input(self.data_lens());

        html::input()
            .class("text-input")
            .try_id(self.id.clone())
            .style(style)
            .class("input")
            .disabled(self.disabled)
            .try_value(self.text.clone())
            .try_max_length(self.max_length)
            .try_placeholder(self.placeholder.clone())
            .on_focus(|_| Msg::Focus(true))
            .on_blur(|_| Msg::Focus(false))
            .on_mouse_enter(|_| Msg::MouseOver(true))
            .on_mouse_leave(|_| Msg::MouseOver(false))
            .on_input(|_| Msg::ResyncText)
    }
}
