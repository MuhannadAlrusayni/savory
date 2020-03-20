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
//! use khalas::{prelude::*, css::Color};
//! use std::borrow::Cow;
//!
//! pub struct UserInfo {
//!     pub username: Cow<'static, str>,
//!     pub email: Cow<'static, str>,
//! };
//!
//! impl<PMsg: 'static> Render<PMsg> for UserInfo {
//!     type View = Node<PMsg>;
//!     type Style = Style;
//!
//!     fn style(&self, _: &impl Theme) -> Self::Style {
//!         Style::default()
//!             .and_background(|conf| conf.set_color(Color::Black))
//!             .and_text(|conf| conf.set_color(Color::White))
//!     }
//!
//!     fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
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
//! use khalas::{prelude::*, css::Color};
//! use std::borrow::Cow;
//!
//! pub fn user_info<PMsg: 'static>(
//!     username: Cow<'static, str>,
//!     email: Cow<'static, str>,
//!     theme: &impl Theme
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

use crate::prelude::{El, Node, Theme, View};

/// Main trait used to render elements.
pub trait Render<PMsg: 'static> {
    /// The returne type from `render` function
    type View: View<PMsg>;
    /// The style used to render this element
    type Style;

    /// Return style for the current state of the element
    fn style(&self, theme: &impl Theme) -> Self::Style;

    /// Users will call this method to render the element, this method basiclly
    /// will call `style` and pass the returned style to `render_with_style`.
    ///
    /// In most cases you don't need to implement this method yourself.
    fn render(&self, theme: &impl Theme) -> Self::View {
        self.render_with_style(theme, self.style(theme))
    }

    /// This is the main method used to render element with the passed style
    ///
    /// # Arguments
    ///
    /// - `Theme` is used here to provieds styles for other elements.
    /// - `Style` is used to style the element.
    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View;
}

impl<PMsg: 'static> Render<PMsg> for Node<PMsg> {
    type View = Node<PMsg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for Vec<Node<PMsg>> {
    type View = Vec<Node<PMsg>>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for El<PMsg> {
    type View = El<PMsg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for Vec<El<PMsg>> {
    type View = Vec<El<PMsg>>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

/// calls `render(theme)` on all passed elements.
#[macro_export]
macro_rules! renders {
    ( $theme:ident, $( $element:expr $(,)? )+ ) => {
        vec![
            $(
                $element.render($theme),
            )*
        ]
    }
}
