//! Reusable elements.

pub mod control;
pub mod display;

pub use self::prelude::*;

pub mod prelude {
    pub use super::{
        control::{
            button::{self, Button},
            checkbox::{self, Checkbox},
            dialog::{self, Dialog},
            entry::{self, Entry},
            radio::{self, Radio},
            spin_entry::{self, SpinEntry},
            switch::{self, Switch},
        },
        display::{
            flexbox::{self, Flexbox},
            header_bar::{self, HeaderBar},
            icon::{self, HtmlIcon, Icon, SvgIcon, UrlIcon},
            label::{self, Label},
            popover::{self, Popover},
        },
    };
    pub use seed::prelude::Node;
}
