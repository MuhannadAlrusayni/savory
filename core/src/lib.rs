//! Savory is library for building user interface.
//!
//!
//! [![master docs](https://img.shields.io/badge/docs-master-blue.svg)](https://malrusayni.gitlab.io/savory/savory/)
//! &middot;
//! [![crate info](https://img.shields.io/crates/v/savory.svg)](https://crates.io/crates/savory)
//! &middot;
//! [![pipeline](https://gitlab.com/MAlrusayni/savory/badges/master/pipeline.svg)](https://gitlab.com/MAlrusayni/savory/pipelines)
//! &middot;
//! [![rustc version](https://img.shields.io/badge/rustc-stable-green.svg)](https://crates.io/crates/savory)
//! &middot;
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//!
//! # Core Concept
//!
//! There are two main types in Savory, **Views** and **Elements**. View types
//! produce static HTML nodes, while Element types produce dynamic HTML nodes,
//! as simple as that.
//!
//! # View types
//!
//! View types implements [`View`] trait, this trait have one method that
//! generate the resulted HTML nodes, here is simple example:
//!
//! ```rust
//! # use savory::prelude::*;
//! struct HomePage;
//!
//! impl<Msg> View<Node<Msg>> for HomePage {
//!     fn view(&self) -> Node<Msg> {
//!         html::div().push("Home page")
//!     }
//! }
//! ```
//!
//! # Element types
//!
//! Element types implements [`Element`] trait, most of the time elements have
//! logic inside them, they commonly carry their state inside them, and since
//! Savory follow Elm style we need message type and lastly we need config type
//! that is used to config the element on it's initialization, here is simple
//! example:
//!
//! ```rust
//! # use savory::prelude::*;
//! // Element module
//! struct Counter(u32);
//!
//! enum Msg {
//!     Increase,
//!     Decrease,
//! }
//!
//! impl Element for Counter {
//!     type Config = u32;
//!     type Message = Msg;
//!
//!     fn init(config: u32, _orders: &mut impl Orders<Msg>, env: &Env) -> Self {
//!         Counter(config)
//!     }
//!
//!     fn update(&mut self, msg: Self::Message, _orders: &mut impl Orders<Self::Message>) {
//!         match msg {
//!             Msg::Increase => self.0 += 1,
//!             Msg::Decrease => self.0 -= 1,
//!         }
//!     }
//! }
//!
//! impl View<Node<Msg>> for Counter {
//!     fn view(&self) -> Node<Msg> {
//!         html::div()
//!         .push(
//!             html::button()
//!                 .push("+")
//!                 .on_click(|_| Msg::Increase)
//!         )
//!         .push(
//!             html::button()
//!                 .push("-")
//!                 .on_click(|_| Msg::Decrease)
//!         )
//!         .push(html::h1().push(self.0.to_string()))
//!     }
//! }
//! ```
//!
//! This example shows how to wirte counter element, so you can write your own
//! elements.
//!
//! ## Ecosystem
//!
//! - `savory` (this crate) - Core library for building user interface
//! - [`savory-router`] - Savory Router used to generate router for your app
//! - [`savory-style`] - Typed CSS style for Savory
//! - [`savory-elements`] - Collection of UI elements based on Savory
//! - [`savory-elements-derive`] - Crate that provide `Element` derive
//!
//! [`savory-router`]: https://malrusayni.gitlab.io/savory/savory_router/index.html
//! [`savory-style`]: https://malrusayni.gitlab.io/savory/savory_style/index.html
//! [`savory-elements`]: https://malrusayni.gitlab.io/savory/savory_elements/index.html
//! [`savory-elements-derive`]: https://malrusayni.gitlab.io/savory/savory_elements_derive/index.html
//! [`View`]: crate::view
//! [`Element`]: crate::prelude::Element

#![forbid(unsafe_code)]

#[macro_use]
pub extern crate seed;

pub mod element;
pub mod env;
pub mod events;
pub mod html;
pub mod node;
pub mod orders;
pub mod traits;
pub mod view;

pub use web_sys;

pub mod prelude {
    pub use crate::{
        element::{AppElementExt, Element},
        env::Env,
        events::*,
        html,
        node::*,
        orders::*,
        seed::prelude::{subs, wasm_bindgen, web_sys, ElRef, MessageMapper, Url},
        traits::*,
        view::View,
    };
}
