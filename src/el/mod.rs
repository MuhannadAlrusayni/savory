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
            radio::{self, Radio},
            switch::{self, Switch},
        },
        display::icon::{self, HtmlIcon, Icon, SvgIcon, UrlIcon},
        layout::flexbox::{self, Flexbox},
    };
}
