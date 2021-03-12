//! Traits used to handle model messages and update element state accordingly.
//!
//! # Elements
//!
//! Elements are an interactive views, they contain their own state and interact
//! with users via messages and update their state accordingly. Elements must
//! implement the `Element` and `View` traits.
//!
//! ## App Element
//!
//! App element are normal elements but have one main difference, the `Config`
//! type is `Url` type, so the app can do routing logic on the initialization.
//!
//! # TODO Examples
//! # TODO Helper types

use crate::prelude::*;
use seed::prelude::{Node, Orders, Url};

/// Trait used to create element and handle element messages and update element
/// state accordingly.
///
/// Element messages can be emitted by HTML events or by other elements,
/// handling these messages is done inside the method `update`, both `init`
/// function and `update` method receive `orders` argument which provide many
/// useful methods to interact with the runtime library [Seed], for example we
/// can subscribe to messages of some type, we can perform async blocks and many
/// more things (see [`Orders`]).
///
/// [Seed]: https://seed-rs.org
/// [`Orders`]: crate::prelude::Orders
pub trait Element {
    /// Element message
    type Message: 'static;
    /// Configuration used to initialize this element
    type Config;

    /// Create and initialize the element
    ///
    /// # Arguments
    /// - `config` configuration used to create the element.
    /// - `orders` used to interacte with Seed runtime.
    fn init(config: Self::Config, orders: &mut impl Orders<Self::Message>, env: Env) -> Self;

    /// update method that recive `Self::Message` and update the model state
    /// accordingly.
    fn update(&mut self, _: Self::Message, _: &mut impl Orders<Self::Message>);
}

/// Extension trait for `Element` when it's used on App element
///
/// This trait provides functions that mounts the app element on HTML node by
/// integrating app element with `seed::app::App`
pub trait AppElementExt
where
    Self: Element<Config = Url> + View<Node<<Self as Element>::Message>> + Sized,
{
    /// Start app element
    ///
    /// # Example
    /// ```
    /// use savory::prelude::*;
    ///
    /// pub struct MyApp;
    ///
    /// pub enum Msg {
    ///     FooMessage,
    /// }
    ///
    /// impl Element for MyApp {
    ///     type Message = Msg;
    ///     type Config = Url;
    ///
    ///     fn init(url: Url, orders: &mut impl Orders<Msg>, env: Env) -> Self {
    ///         // initialize the app goes here
    ///         todo!()
    ///     }
    ///
    ///     fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
    ///         // handling app messages goes here
    ///         todo!()
    ///     }
    /// }
    ///
    /// impl View<Node<Msg>> for MyApp {
    ///     fn view(&self) -> Node<Msg> {
    ///         // viewing the app goes here
    ///         todo!()
    ///     }
    /// }
    ///
    /// #[wasm_bindgen(start)]
    /// pub fn view() {
    ///     MyApp::start();
    /// }
    /// ```
    fn start() -> seed::app::App<Self::Message, Self, Node<Self::Message>> {
        Self::start_at("app")
    }

    /// Start app element at specifec element that matchs the `id` passed
    fn start_at(id: &str) -> seed::app::App<Self::Message, Self, Node<Self::Message>> {
        seed::app::App::start(
            id,
            |url, orders| Self::init(url, orders, Env::base_branch()),
            |msg, app, orders| app.update(msg, orders),
            |app| app.view(),
        )
    }
}

impl<T> AppElementExt for T where
    Self: Element<Config = Url> + View<Node<<Self as Element>::Message>> + Sized
{
}
