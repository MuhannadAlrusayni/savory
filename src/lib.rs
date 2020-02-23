
pub mod attribute;
pub mod css;
pub mod el;
pub mod events;
pub mod model;
pub mod render;
pub mod routable;
pub mod theme;

pub mod prelude {
    pub use crate::css::Style;
    pub use crate::el::prelude::*;
    pub use crate::model::Model;
    pub use crate::render::Render;
    pub use crate::routable::Routable;
    pub use crate::theme::Theme;
}

#[macro_use]
extern crate seed;

#[macro_use]
extern crate derive_more;
