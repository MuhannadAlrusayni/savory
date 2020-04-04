//! Traits used to render elements.
//!
//! We can create renderable element in two ways, either by implementing [`Render`]
//! trait or by using standalone function that returns type that implement
//! [`Render`] trait such as [`Node`].
//!
//! # Examples
//! Here is simple example on implementing [`Render`] trait:
//! ```
//! #[macro_use] extern crate seed;
//! use savory::{prelude::*, css::Color};
//! use std::borrow::Cow;
//!
//! pub struct UserInfo {
//!     pub username: Cow<'static, str>,
//!     pub email: Cow<'static, str>,
//! };
//!
//! impl<PMsg> Render for UserInfo {
//!     type Output = Node<PMsg>;
//!     type Style = Style;
//!
//!     fn style(&self, _: &Theme) -> Style {
//!         Style::default()
//!             .and_background(|conf| conf.set_color(Color::Black))
//!             .and_text(|conf| conf.set_color(Color::White))
//!     }
//!
//!     fn render(&self) -> Self::Output {
//!         Flexbox::new()
//!             .center()
//!             .column()
//!             .full_size()
//!             .add(h3![&format!("Username: {}", self.username)])
//!             .add(h4![&format!("Email: {}", self.email)])
//!             .render_with_style(theme, style)
//!     }
//! }
//! ```
//!
//! As we can see this way is a pretty verbose for simple elements, we can have
//! the same element with standalone function:
//! ```
//! #[macro_use] extern crate seed;
//! use savory::{prelude::*, css::Color};
//! use std::borrow::Cow;
//!
//! pub fn user_info<PMsg>(
//!     username: Cow<'static, str>,
//!     email: Cow<'static, str>,
//!     theme: &Theme
//! ) -> Node<PMsg> {
//!     // creating element style
//!     let style = Style::default()
//!         .and_background(|conf| conf.set_color(Color::Black))
//!         .and_text(|conf| conf.set_color(Color::White));
//!
//!     // rendering the element
//!     Flexbox::new()
//!         .center()
//!         .column()
//!         .full_size()
//!         .add(h3![&format!("Username: {}", username)])
//!         .add(h4![&format!("Email: {}", email)])
//!         .render_with_style(theme, style)
//! }
//! ```
//!
//! # `Render` trait vs Standalone functions
//!
//! **`Render` trait** is pretty verbose for simple elements, but it works well
//! with bigger elements, it provieds users of the element the ability to
//! override defult style for the element, so it's well suited for resuable
//! elements.
//!
//! **Standalone functions** are simple to write and read, but they become complex
//! and hard to wrok with when the element get bigger or the function start to
//! accept more and more parameters.
//!
//! So, for simple element that is used in a few context in the application, I
//! would suggest using standalone functions, otherwise I would suggest creating
//! type for the element and implement [`Render`] trait for it.
//!
//! [`Render`]: crate::prelude::Render
//! [`Node`]: crate::prelude::Node

/// Main trait used to render elements.
pub trait Render {
    /// The returne type from `render` function
    type Output;

    /// This is the main method used to render element
    ///
    /// Elements need to implement this method and return rendered element.
    fn render(&self) -> Self::Output;
}
