//! Savory is library that provides you a set of themeable elements.
//!
//! Elements have been built using a set of core traits [`View`], [`Model`] and
//! [`Theme`], usully users of the library will only need to implement [`View`]
//! and [`Model`] traits to build thier own custom elements.
//!
//! # Core Concept
//!
//! Savory have be designed in a way that decouples themes development from
//! elements development, this is achieved using two traits [`Theme`] and
//! [`View`].
//!
//! [`Theme`] trait is responseable on styling savory elements, this trait
//! basiclly have methods for every elements in savory, these methods take
//! element and return style for the element, the returned style is used when
//! rendering the element.
//!
//! [`View`] trait has three methods `render`, `render_with_style` and
//! `style`, `style` method used to construct element style, while
//! `render_with_style` method used to build the HTML structure, `render` has
//! default implementation.
//!
//! There is another core trait which is [`Model`], this trait is responseable
//! on handling the model messages and updating the model state accordingly.
//!
//! # Elements
//! There are two element types we can create in savory:
//!
//! - **Statful elements** those who implement [`View`] and [`Model`].
//! - **Stateless elements** those who implement [`View`] only.
//!
//! ## Avaliable Elements
//! Here is a list of the currently avaliable elements:
//! - Statful
//!   - [Button](prelude::Button)
//!   - [Checkbox](prelude::Checkbox)
//!   - [Radio](prelude::Radio)
//!   - [Switch](prelude::Switch)
//!   - [Entry](prelude::Entry)
//!   - [SpinEntry](prelude::SpinEntry)
//!   - [Dialog](prelude::Dialog)
//!   - [MenuButton](prelude::MenuButton)
//! - Statless
//!   - [Icon](prelude::Icon)
//!     - [HtmlIcon](prelude::HtmlIcon)
//!     - [SvgIcon](prelude::SvgIcon)
//!     - [UrlIcon](prelude::UrlIcon)
//!   - [Popover](prelude::Popover)
//!   - [Flexbox](prelude::Flexbox)
//!     - [Item](prelude::flexbox::Item)
//!   - [Label](prelude::Label)
//!   - [HeaderBar](prelude::HeaderBar)
//!
//!
//! [`Theme`]: crate::prelude::Theme
//! [`View`]: crate::prelude::View
//! [`Model`]: crate::prelude::Model

pub mod element;
pub mod msg_mapper;
pub mod orders_ext;
pub mod view;

/// savory prelude.
pub mod prelude {
    pub use crate::{
        element::{AppElement, AppElementExt, Element},
        msg_mapper::{MessageMapperExt, MsgMapper},
        orders_ext::OrdersExt,
        view::View,
    };
    pub use seed::prelude::{MessageMapper, Node, Orders, Url};
}
