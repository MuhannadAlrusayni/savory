pub mod attribute;
pub mod css;
pub mod el;
pub mod events;
pub mod seed_ext;

#[macro_use]
extern crate seed;

#[macro_use]
extern crate derive_more;

pub mod prelude {
    pub use crate::{
        attribute as att, css, el as html,
        seed_ext::{AddForEl, ElExt, ElRefExt, NodeExt, SetForEl, TryAddForEl, TrySetForEl},
    };
    pub use seed::prelude::{El, ElRef, Node, UpdateEl, UpdateElForIterator};
}
// pub use seed::prelude::{
//     AfterMount, App, BeforeMount, El, ElRef, GetElement, MessageMapper, MountType, Node,
//     Orders, RenderTimestampDelta, UpdateEl, UpdateElForIterator, Url, UrlHandling, View,
// };
