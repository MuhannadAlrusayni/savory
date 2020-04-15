//! UI Elements built using Savory
//!
//! Elements in this crate are built using `savory_core` crate.
//!
//! # Features
//!
//! - Theme: Elements are fully styled, either by [`Theme`] type or by custom
//!   `Styler`.
//! - Reusability: Elements are highly reusable/composable, [`Modifier`] type can
//!   be used to add common styles to any `View`/`Element`, styles and events
//!   can be replaced entirely, very useful when you want to make your own
//!   elements.
//! - Decoupled Developemnt: Themes can be developed in seprate crate without
//!   even touching elements code, and the same for elements developemnt, the
//!   are developed sepratedly from the theme, thanks to [`ThemeImpl`] trait this
//!   is possible.
//!
//! # Views
//!
//! - [Flexbox](prelude::Flexbox)
//!   - [Item](prelude::flexbox::Item)
//! - [Label](prelude::Label)
//! - [HeaderBar](prelude::HeaderBar)
//! - [Icon](prelude::Icon)
//!   - [Html](prelude::icon::Html)
//!   - [Svg](prelude::icon::Svg)
//!   - [Url](prelude::icon::Url)
//! - [Modifier](prelude::Modifier)
//!
//! # Elements
//!
//! - [Button](prelude::Button)
//! - [Checkbox](prelude::Checkbox)
//! - [Dialog](prelude::Dialog)
//! - [Entry](prelude::Entry)
//! - [Popover](prelude::Popover)
//! - [ProgressBar](prelude::ProgressBar)
//! - [Radio](prelude::Radio)
//! - [SpinEntry](prelude::SpinEntry)
//! - [Switch](prelude::Switch)
//!
//!
//! [`Theme`]: prelude::Theme
//! [`ThemeImpl`]: theme::ThemeImpl
//! [`Modifier`]: prelude::Modifier

#![forbid(unsafe_code)]

#[macro_use]
extern crate derive_more;

pub mod element;
pub mod theme;
pub mod view;

pub use self::prelude::*;

/// savory_elements prelude
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
