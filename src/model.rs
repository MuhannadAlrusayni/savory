//! Traits used to handle model messages and update element state accordingly.
//!
//! TODO: intro
//!
//! # TODO Examples
//! # TODO Helper types

use crate::prelude::{Orders, Render};

/// This trait used to handle model messages and update the model state accordingly.
///
/// Model messages is emitted when HTML events trigger, handling these messages
/// is done inside the method `update`, we can decide to not render the element
/// by calling `orders.skip()` if the update doesn't affect the appearance of
/// the element, we can do other things using `orders` methods, you maight want
/// to check it's docs.
pub trait Model<PMsg, GMsg>: Render<PMsg>
where
    PMsg: 'static,
{
    /// The model message type
    type Message;

    /// update method that recive `Self::Message` and update the model state accordingly.
    fn update(&mut self, _: Self::Message, _: &mut impl Orders<PMsg, GMsg>);
}
