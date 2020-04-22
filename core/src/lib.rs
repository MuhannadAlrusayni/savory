//! Savory is library for building user interface.
//!
//! [![master docs](https://img.shields.io/badge/docs-master-blue.svg)](https://malrusayni.gitlab.io/savory/savory_core/)
//! &middot;
//! [![crate info](https://img.shields.io/crates/v/savory.svg)](https://crates.io/crates/savory_core)
//! &middot;
//! [![pipeline](https://gitlab.com/MAlrusayni/savory/badges/master/pipeline.svg)](https://gitlab.com/MAlrusayni/savory/pipelines)
//! &middot;
//! [![rustc version](https://img.shields.io/badge/rustc-stable-green.svg)](https://crates.io/crates/savory)
//! &middot;
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//!
//!
//! # Features
//!
//! - **Views**: Views can be any type implement `View` trait or any standalone
//!   function that returns `Node`, views can be trait object which make them very
//!   composable.
//! - **Elements**: Savory uses elements as core building unit when building
//!   stateful UI. Elements owns thier state and handle user inputs via messages.
//! - **Collection of UI elements**: Savory ships with collection of resuable and
//!   themeable UI elements.
//! - **Theme**: UI elements can be themed by any type that implement `ThemeImpl`
//!   trait, themes have full control on the element appearance.
//! - **Typed HTML**: Use typed CSS and HTML attributes, Savory try hard not to rely
//!   on strings when creating CSS and HTML attributes since these can produce hard
//!   to debug bugs.
//! - **Enhance Seed API**: Enhancement on Seed API that makes working with `Node`,
//!   `Orders` fun.
//!
//! Savory tries to make writing UI elements fun and boilerplate free.
//!
//! Savory crates:
//! - `savory`: savory CLI
//! - [`savory_core`]: Library for building user interface (this crate)
//! - [`savory_html`]: Typed HTML for Savory
//! - [`savory_elements`]: UI Elements based on Savory
//! - [`savory_derive`]: Helper derives
//!
//! # Core Concept
//!
//! Savory have two main types **View** and **Element**, View types produce
//! static HTML, while Element types produce interactive HTML, as simple as
//! that.
//!
//! Elements types must implemente [`Element`] and [`View`] traits, which would
//! make them interactive.
//!
//! View types must implemente [`View`] trait, that would produce the static
//! HTML.
//!
//! # Counter Example
//!
//! Here is very simple counter, that doesn't use all Savory features, but it's
//! good as starting point for newcomers.
//!
//! ``` rust
//! use savory_core::prelude::*;
//! use savory_html::prelude::*;
//! use wasm_bindgen::prelude::*;
//!
//! // app element (the model)
//! pub struct Counter(i32);
//!
//! // app message
//! pub enum Msg {
//!     Increment,
//!     Decrement,
//! }
//!
//! impl Element<Msg> for Counter {
//!     type Message = Msg;
//!     type Props = Url;
//!
//!     // initialize the app in this function
//!     fn init(_: Url, _: &mut impl Orders<Msg>) -> Self {
//!         Self(0)
//!     }
//!
//!     // handle app messages
//!     fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
//!         match msg {
//!             Msg::Increment => self.0 += 1,
//!             Msg::Decrement => self.0 -= 1,
//!         }
//!     }
//! }
//!
//! impl View for Counter {
//!     type Output = Node<Msg>;
//!
//!     // view the app
//!     fn view(&self) -> Self::Output {
//!         let inc_btn = html::button()
//!             .add("Increment")
//!             .and_events(|events| events.click(|_| Msg::Increment));
//!
//!         let dec_btn = html::button()
//!             .add("Decrement")
//!             .and_events(|events| events.click(|_| Msg::Decrement));
//!
//!         html::div()
//!             .add(inc_btn)
//!             .add(self.0.to_string())
//!             .add(dec_btn)
//!     }
//! }
//!
//! #[wasm_bindgen(start)]
//! pub fn view() {
//!     // mount and start the app at `app` element
//!     Counter::start();
//! }
//! ```
//!
//! [`View`]: crate::prelude::View
//! [`Element`]: crate::prelude::Element
//! [`savory_core`]: https://gitlab.com/MAlrusayni/savory/tree/master/core
//! [`savory_html`]: https://gitlab.com/MAlrusayni/savory/tree/master/html
//! [`savory_elements`]: https://gitlab.com/MAlrusayni/savory/tree/master/elements
//! [`savory_derive`]: https://gitlab.com/MAlrusayni/savory/tree/master

#![forbid(unsafe_code)]

pub mod element;
pub mod msg_mapper;
pub mod orders_ext;
pub mod view;

/// savory prelude.
pub mod prelude {
    pub use crate::{
        element::{AppElementExt, Element},
        msg_mapper::{MessageMapperExt, MsgMapper},
        orders_ext::OrdersExt,
        view::{StyledView, View},
    };
    pub use seed::prelude::{MessageMapper, Node, Orders, Url};
}
