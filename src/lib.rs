#![feature(decl_macro)]
#![feature(never_type)]
#![feature(bool_to_option)]

pub mod el;
pub mod propertie;
// pub mod layout;
pub mod css;
pub mod events;
pub mod macros;
pub mod model;
pub mod render;
pub mod routable;
pub mod theme;

pub mod prelude {
    pub use crate::css::Style;
    pub use crate::model::Model;
    pub use crate::render::Render;
    pub use crate::routable::Routable;
    pub use crate::theme::Theme;
}

#[macro_use]
extern crate seed;

#[macro_use]
extern crate derive_more;
