//! Khalas is library that provides you a set of themeable elements.
//!
//! Elements have been built using a set of core traits [`Render`], [`Model`] and
//! [`Theme`], usully users of the library will only need to implement [`Render`]
//! and [`Model`] traits to build thier own custom elements.
//!
//! # Core Concept
//!
//! Khalas have be designed in a way that decouples themes development from
//! elements development, this is achieved using two traits [`Theme`] and
//! [`Render`].
//!
//! [`Theme`] trait is responseable on styling khalas elements, this trait
//! basiclly have methods for every elements in khalas, these methods take
//! element and return style for the element, the returned style is used when
//! rendering the element.
//!
//! [`Render`] trait has three methods `render`, `render_with_style` and
//! `style`, `style` method used to construct element style, while
//! `render_with_style` method used to build the HTML structure, `render` has
//! default implementation.
//!
//! There is another core trait which is [`Model`], this trait is responseable
//! on handling the model messages and updating the model state accordingly.
//!
//! # Elements
//! There are two element types we can create in khalas:
//!
//! - **Statful elements** those who implement [`Render`] and [`Model`].
//! - **Stateless elements** those who implement [`Render`] only.
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
//! [`Render`]: crate::prelude::Render
//! [`Model`]: crate::prelude::Model

pub mod attribute;
pub mod css;
pub mod el;
pub mod events;
pub mod model;
pub mod msg_mapper;
pub mod render;
pub mod routable;
pub mod seed_ext;
pub mod theme;

/// khalas prelude.
pub mod prelude {
    pub use crate::attribute as att;
    pub use crate::css::Style;
    pub use crate::el::prelude::*;
    pub use crate::events::Events;
    pub use crate::model::Model;
    pub use crate::msg_mapper::{MessageMapperExt, MsgMapper};
    pub use crate::render::Render;
    pub use crate::routable::Routable;
    pub use crate::seed_ext::{ElExt, ElRefExt, NodeExt};
    pub use crate::theme::Theme;
    pub use seed::prelude::{
        AfterMount, App, BeforeMount, El, ElRef, MessageMapper, MountType, Orders,
        RenderTimestampDelta, UpdateEl, UpdateElForIterator, UrlHandling, View,
    };
    pub use wasm_bindgen::prelude::*;
}

#[macro_use]
extern crate seed;

#[macro_use]
extern crate derive_more;
