pub mod container;
pub mod control;
pub mod display;
pub mod layout;

pub use self::prelude::*;

pub mod prelude {
    pub use super::{
        control::{
            button::{self, Button},
            checkbox::{self, Checkbox},
            entry::{self, Entry},
            radio::{self, Radio},
            spin_entry::{self, SpinEntry},
            switch::{self, Switch},
        },
        display::{
            icon::{self, HtmlIcon, Icon, SvgIcon, UrlIcon},
            popover::{self, Popover},
        },
        layout::flexbox::{self, Flexbox},
    };
}
