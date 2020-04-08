//! Traits used to handle model messages and update element state accordingly.
//!
//! TODO: intro
//!
//! # TODO Examples
//! # TODO Helper types

use crate::prelude::*;
use seed::{app::UndefinedGMsg, prelude::View as SeedView};

// This trait used to handle model messages and update the model state accordingly.

// Model messages is emitted when HTML events trigger, handling these messages
// is done inside the method `update`, we can decide to not render the element
// by calling `orders.skip()` if the update doesn't affect the appearance of
// the element, we can do other things using `orders` methods, you maight want
// to check it's docs.
pub trait Element<PMsg: 'static, GMsg = UndefinedGMsg>: View {
    /// Element message
    type Message;
    /// properties used to initialize this element
    type Props;

    /// Create and initialize the element
    ///
    /// # Arguments
    /// - `map_msg` used to map element message `Self::Message` to the parrent
    ///   Message
    /// - `orders` used to interacte with the runtime, such as subscribing to
    ///   messages, or sending messages ..etc.
    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self;

    fn updates(
        &mut self,
        msgs: impl IntoIterator<Item = Self::Message>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        for msg in msgs.into_iter() {
            self.update(msg, orders);
        }
    }

    /// update method that recive `Self::Message` and update the model state accordingly.
    fn update(&mut self, _: Self::Message, _: &mut impl Orders<PMsg, GMsg>);
}

/// Similar to `Element` trait but for the root element (the app)
///
/// The `init` function takes `Url` insted if `MsgMapper` as it's first
/// argument.
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

pub trait AppElementExt<GMsg = UndefinedGMsg>: AppElement<GMsg>
where
    Self: Sized,
    GMsg: 'static,
    Self::Output: SeedView<Self::Message> + 'static,
{
    fn start() -> seed::app::App<Self::Message, Self, Self::Output, GMsg> {
        Self::start_at("root")
    }

    fn start_at(root_el: &str) -> seed::app::App<Self::Message, Self, Self::Output, GMsg> {
        seed::app::App::start(
            root_el,
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
    Self::Output: SeedView<Self::Message> + 'static,
{
}
