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
//! pub struct UserInfo<PMsg> {
//!     pub username: Cow<'static, str>,
//!     pub email: Cow<'static, str>,
//!     // store the parent message type, so we can use it in `View` impl
//!     phantom: std::marker::PhantomData<*const PMsg>,
//! }
//!
//! impl<PMsg> View for UserInfo<PMsg> {
//!     type Output = Node<PMsg>;
//!
//!     fn view(&self) -> Self::Output {
//!         let style = css::Style::default()
//!             .and_size(|conf| conf.full())
//!             .display(val::Flex)
//!             .flex_direction(val::Column)
//!             .justify_content(val::Center)
//!             .align_content(val::Center)
//!             .align_items(val::Center);
//!
//!         let username = html::h3().add(html::text(format!("Username: {}", self.username)));
//!         let email = html::h4().add(html::text(format!("Email: {}", self.email)));
//!
//!         html::div()
//!             .set(&style)
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
//!     let username = html::h3().add(html::text(format!("Username: {}", username)));
//!     let email = html::h4().add(html::text(format!("Email: {}", email)));
//!
//!     html::div()
//!         .set(&style)
//!         .add(username)
//!         .add(email)
//! }
//! ```
//!
//! # `View` & `StyledView` traits vs Standalone functions
//!
//! **Traits approach** is pretty verbose for simple elements, but it works well
//! with bigger elements, types that implemente `View` can be storde as trait
//! object, views that implemente `StyledView` can be viewed with diffrent style
//! thus we can reuse it in many other context.
//!
//! **Standalone functions** are simple to write and read, but they become complex
//! and hard to wrok with when the element get bigger or the function start to
//! accept more and more parameters.
//!
//! So, for simple element that is used in a few context in the application, I
//! would suggest using standalone functions, otherwise I would suggest creating
//! type for the element and implement [`View`] trait for it.
//!
//! [`View`]: crate::prelude::View
//! [`Node`]: crate::prelude::Node

/// Main trait used to render view.
pub trait View {
    /// The returne type from `render` function
    type Output;

    /// view method that returns Seed `Node`
    fn view(&self) -> Self::Output;
}

/// Trait that makes the view resuable by accepting `Self::Style`
///
/// Reusable view should implement this trait.
pub trait StyledView: View {
    /// Stye accepted by `styled_view` method
    type Style;

    /// view with the passed styled
    fn styled_view(&self, style: Self::Style) -> Self::Output;
}
