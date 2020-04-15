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
//! App element is the root element that contains other elements, app element
//! must implement `AppElement` and `View` traits.
//!
//! # TODO Examples
//! # TODO Helper types

use crate::prelude::*;
use seed::{app::UndefinedGMsg, prelude::IntoNodes};

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
pub trait Element<PMsg: 'static, GMsg = UndefinedGMsg>: View {
    /// Element message
    type Message;
    /// Properties used to initialize this element
    type Props;

    /// Create and initialize the element
    ///
    /// # Arguments
    /// - `props` properties used to create the element.
    /// - `orders` used to interacte with the runtime.
    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self;

    /// update method that recive `Self::Message` and update the model state accordingly.
    fn update(&mut self, _: Self::Message, _: &mut impl Orders<PMsg, GMsg>);
}

/// Similar to `Element` trait but for the root element (the app)
///
/// The `init` function takes `Url` insted if `Props` as it's first argument.
pub trait AppElement<GMsg = UndefinedGMsg>: View {
    /// App message
    type Message: 'static;

    /// Create and initialize the app element
    ///
    /// # Arguments
    /// - `url` the requested url when the app was loaded
    /// - `orders` used to interacte with the runtime, such as subscribing to
    ///   messages, or sending messages ..etc.
    fn init(url: Url, orders: &mut impl Orders<Self::Message, GMsg>) -> Self;

    /// update method that recive `Self::Message` and update the model state accordingly.
    fn update(&mut self, _: Self::Message, _: &mut impl Orders<Self::Message, GMsg>);
}

/// Extension trait for `AppElement`
///
/// This trait provides functions that mounts the app element on HTML node by
/// integrating `AppElement` with `seed::app::App`
pub trait AppElementExt<GMsg = UndefinedGMsg>: AppElement<GMsg>
where
    Self: Sized,
    GMsg: 'static,
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
    /// impl AppElement for MyApp {
    ///     type Message = Msg;
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
    fn start() -> seed::app::App<Self::Message, Self, Self::Output, GMsg> {
        Self::start_at("root")
    }

    /// Start app element at specifec node that matchs the `id` passed
    fn start_at(id: &str) -> seed::app::App<Self::Message, Self, Self::Output, GMsg> {
        seed::app::App::start(
            id,
            |url, orders| Self::init(url, orders),
            |msg, app, orders| app.update(msg, orders),
            |app| app.view(),
        )
    }
}

impl<T, GMsg> AppElementExt<GMsg> for T
where
    Self: AppElement<GMsg>,
    GMsg: 'static,
    Self::Output: IntoNodes<Self::Message> + 'static,
{
}
