//! Savory is library for building user interface.
//!
//! Savory crates:
//! - `savory`: savory CLI
//! - [`savory_core`]: Library for building user interface (this crate)
//! - [`savory_html`]: Typed HTML for Savory
//! - [`savory_elements`]: UI Elements based on Savory
//! - [`savory_derive`]: Helper derives
//!
//! # Core Concept
//!
//! Savory have two main types **View** and **Element**, View produce static HTML,
//! while Element produce interactive HTML, as simple as that.
//!
//! Elements types must implemente [`Element`] and [`View`] traits, which would
//! make them interactive.
//!
//! View types must implemente [`View`] trait, that would produce the static
//! HTML.
//!
//!
//! [`View`]: crate::prelude::View
//! [`Element`]: crate::prelude::Element
//! [`savory_core`]: https://gitlab.com/MAlrusayni/savory/tree/master/core
//! [`savory_html`]: https://gitlab.com/MAlrusayni/savory/tree/master/html
//! [`savory_elements`]: https://gitlab.com/MAlrusayni/savory/tree/master/elements
//! [`savory_derive`]: https://gitlab.com/MAlrusayni/savory/tree/master

#![forbid(unsafe_code)]

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
        view::{StyledView, View},
    };
    pub use seed::prelude::{MessageMapper, Node, Orders, Url};
}
