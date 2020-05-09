//! Traits used to build HTML nodes.
//!
//! # Views
//!
//! Views are types that return Seed [`Node`] when we view them. Views can be
//! created either by implementing [`View`] trait or by standalone function that
//! returns Seed [`Node`].
//!
//! # Examples
//! Here is simple example on implementing [`View`] trait:
//! ```
//! use savory_core::prelude::*;
//! use savory_html::{prelude::*, css::{Color, values as val}};
//! use std::borrow::Cow;
//!
//! pub struct UserInfo {
//!     pub username: Cow<'static, str>,
//!     pub email: Cow<'static, str>,
//! }
//!
//! impl<Msg> View<Node<Msg>> for UserInfo {
//!     fn view(&self) -> Node<Msg> {
//!         let style = css::Style::default()
//!             .and_size(|conf| conf.full())
//!             .display(val::Flex)
//!             .flex_direction(val::Column)
//!             .justify_content(val::Center)
//!             .align_content(val::Center)
//!             .align_items(val::Center);
//!
//!         let username = html::h3().add(format!("Username: {}", self.username));
//!         let email = html::h4().add(format!("Email: {}", self.email));
//!
//!         html::div()
//!             .set(style)
//!             .add(username)
//!             .add(email)
//!     }
//! }
//! ```
//!
//! As we can see this way is a pretty verbose for simple elements, we can have
//! the same element with standalone function:
//! ```
//! use savory_core::prelude::*;
//! use savory_html::{prelude::*, css::{Color, values as val}};
//! use std::borrow::Cow;
//!
//! pub fn user_info<PMsg>(
//!     username: Cow<'static, str>,
//!     email: Cow<'static, str>,
//! ) -> Node<PMsg> {
//!     // creating element style
//!     let style = css::Style::default()
//!         .and_size(|conf| conf.full())
//!         .display(val::Flex)
//!         .flex_direction(val::Column)
//!         .justify_content(val::Center)
//!         .align_content(val::Center)
//!         .align_items(val::Center);
//!
//!     let username = html::h3().add(format!("Username: {}", username));
//!     let email = html::h4().add(format!("Email: {}", email));
//!
//!     html::div()
//!         .set(style)
//!         .add(username)
//!         .add(email)
//! }
//! ```
//!
//! # `View` trait vs Standalone functions
//!
//! **Traits approach** is pretty verbose for simple elements, but it works well
//! with bigger elements that needs a lot of arguments, types that implement
//! `View` can be storde as trait object.
//!
//! **Standalone functions** are simple to write and read, but they become complex
//! and hard to wrok with when the element get bigger or the function start to
//! accept more and more arguments.
//!
//! So, for simple element that is used in a few context in the application, I
//! would suggest using standalone functions, otherwise I would suggest creating
//! type for the element and implement [`View`] trait for it.
//!
//! [`View`]: crate::prelude::View
//! [`Node`]: crate::prelude::Node

/// Main trait used to render view.
pub trait View<Output> {
    /// view method that returns Seed `Node`
    fn view(&self) -> Output;
}
