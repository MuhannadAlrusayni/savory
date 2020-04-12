//! Reusable elements.

#![forbid(unsafe_code)]

#[macro_use]
extern crate derive_more;

pub mod element;
pub mod theme;
pub mod view;

pub use self::prelude::*;

pub mod prelude {
    pub use super::{
        element::{
            button::{self, Button},
            checkbox::{self, Checkbox},
            dialog::{self, Dialog},
            entry::{self, Entry},
            popover::{self, Popover},
            progress_bar::{self, ProgressBar},
            radio::{self, Radio},
            spin_entry::{self, SpinEntry},
            switch::{self, Switch},
        },
        theme::{self, Theme, ThemeChanged, ThemeLens},
        view::{
            flexbox::{self, Flexbox},
            header_bar::{self, HeaderBar},
            icon::{self, Icon},
            label::{self, Label},
            modifier::{IntoModifier, Modifier},
        },
    };
    pub use savory_derive::Element;
}
