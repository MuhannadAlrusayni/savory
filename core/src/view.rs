//! Traits used to build views.
//!
//! # Views
//!
//! View is used to build HTML nodes, and since Savory is built on top of Seed,
//! views should return Seed [`Node`], the returned type can be anything
//! actually, but it's common to return single node (`Node<Msg>`) or collection
//! of nodes (`Vec<Node<Msg>>`) depending on the view type.
//!
//! Views can be any type that implement `View` trait, here is an example of
//! view type:
//!
//! Here is simple example on implementing [`View`] trait:
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
//! # View Functions
//!
//! Function and closure can be used where [`View`] is expected, Savory have
//! blanket implementation for `Fn() -> T`, so we can use closure as view type:
//!
//! ```rust
//! # use savory::prelude::*;
//! let greeting = "Hello";
//! let greeting = || html::h1().push(greeting);
//! let node: Node<()> = greeting.view();
//! ```
//!
//! # View types and View functions
//!
//! View functions are simple to write and read and suitable for application
//! code, while view types works well with reusable, complex elements.
//!
//! [`View`]: crate::prelude::View
//! [`Node`]: crate::prelude::Node

use crate::prelude::{html, Node};

/// Main trait used to render view.
pub trait View<Output> {
    /// view method that returns Seed `Node`
    fn view(&self) -> Output;
}

impl<T, F> View<T> for F
where
    F: Fn() -> T,
{
    fn view(&self) -> T {
        self()
    }
}

impl<Msg> View<Node<Msg>> for String {
    fn view(&self) -> Node<Msg> {
        html::text(self.clone())
    }
}

impl<Msg> View<Node<Msg>> for &'static str {
    fn view(&self) -> Node<Msg> {
        html::text(*self)
    }
}

impl<Msg> View<Node<Msg>> for std::borrow::Cow<'static, str> {
    fn view(&self) -> Node<Msg> {
        html::text(self.clone())
    }
}
