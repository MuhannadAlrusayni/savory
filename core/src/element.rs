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
//! App element are normal elements but two main difference, the `Props` type is
//! `Url` type, and the `PMsg` generic type have to be the same type as
//! `Message` type, and this actully make sense becuase `PMsg` ment to hold the
//! parent element message type and app elements doesn't have parent.
//!
//! # TODO Examples
//! # TODO Helper types

use crate::prelude::*;
use seed::prelude::IntoNodes;

/// Trait used to create element and handle element messages and update
/// element state accordingly.
///
/// Element messages can be emitted by HTML events or by other elements,
/// handling these messages is done inside the method `update`, both `init`
/// function and `update` method receive `orders` argument which provide many
/// useful methods to interact with the runtime library [Seed], for example we
/// can subscribe to messages of some type, we can send messages to other
/// elemenrs, we can perform async blocks and many more things (see [`Orders`]).
///
/// [Seed]: https://seed-rs.org
/// [`Orders`]: crate::prelude::Orders
pub trait Element<PMsg: 'static> {
    /// Element message
    type Message;
    /// Properties used to initialize this element
    type Props;

    /// Create and initialize the element
    ///
    /// # Arguments
    /// - `props` properties used to create the element.
    /// - `orders` used to interacte with the runtime.
    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self;

    /// update method that recive `Self::Message` and update the model state accordingly.
    fn update(&mut self, _: Self::Message, _: &mut impl Orders<PMsg>);
}

/// Extension trait for `Element` when it's used on App element
///
/// This trait provides functions that mounts the app element on HTML node by
/// integrating app element with `seed::app::App`
pub trait AppElementExt<Msg>: Element<Msg, Props = Url, Message = Msg>
where
    Msg: 'static,
    Self: Sized + View,
    Self::Output: IntoNodes<Self::Message> + 'static,
{
    /// Start app element
    ///
    /// # Example
    /// ```
    /// use savory_core::prelude::*;
    /// use wasm_bindgen::prelude::*;
    ///
    /// pub struct MyApp;
    ///
    /// pub enum Msg {
    ///     FooMessage,
    /// }
    ///
    /// impl Element<Msg> for MyApp {
    ///     type Message = Msg;
    ///     type Props = Url;
    ///
    ///     fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
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
    /// impl View for MyApp {
    ///     type Output = Node<Msg>;
    ///
    ///     fn view(&self) -> Self::Output {
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
    fn start() -> seed::app::App<Self::Message, Self, Self::Output> {
        Self::start_at("app")
    }

    /// Start app element at specifec element that matchs the `id` passed
    fn start_at(id: &str) -> seed::app::App<Self::Message, Self, Self::Output> {
        seed::app::App::start(
            id,
            |url, orders| Self::init(url, orders),
            |msg, app, orders| app.update(msg, orders),
            |app| app.view(),
        )
    }
}

impl<Msg, T> AppElementExt<Msg> for T
where
    Msg: 'static,
    Self: Element<Msg, Props = Url, Message = Msg> + View,
    Self::Output: IntoNodes<Self::Message> + 'static,
{
}
