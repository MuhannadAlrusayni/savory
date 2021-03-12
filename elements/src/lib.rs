//! UI Elements built using Savory
//!
//! Elements in this crate are built using `savory` ecosystem crates.
//!
//! # Features
//!
//!
//! - **Design System**: Elements are fully styled using [`DesignSystem`].
//! - **Reusability**: Elements are highly reusable/composable.
//! - **Decoupled Developemnt**: Design Systems can be developed in seprate crate
//!   without even touching elements code, and the same for elements developemnt,
//!   the are developed sepratedly from the design system, thanks to
//!   [`DesignSystemImpl`] trait.
//! - **Clean View**: build your view in a clean and declarative way, no more macros at
//!   all.
//! - **Trait Based**: embrace Rust trait system, all savory elements implments
//!   `Element` and/or `View` trait.
//! - **Typed HTML**: Use typed CSS and HTML attributes, Savory try hard not to rely
//!   on strings when creating CSS and HTML attributes since these can produce hard
//!   to debug bugs.
//! - **Collection of UI elements**: Savory ships with collection of resuable and
//!   themeable UI elements.
//! - **Enhance Seed API**: Enhancement on Seed API that makes working with `Node`,
//!   `Orders` fun.
//!
//! Savory tries to make writing UI elements fun and boilerplate free.
//!
//! # Layouts
//!
//! - [Flex](prelude::Flex)
//!
//! # Elements & Views
//!
//! - [Button](prelude::Button)
//! - [Switch](prelude::Switch)
//! - [Radio](prelude::Radio)
//! - [Text](prelude::Text)
//! - [TextInput](prelude::TextInput)
//! - [ProgressBar](prelude::ProgressBar)
//! - [Image](prelude::Image)
//! - [Svg](prelude::Svg)
//!
//! [`DesignSystem`]: prelude::DesignSystem
//! [`DesignSystemImpl`]: design_system::DesignSystemImpl
//! [`Modifier`]: prelude::Modifier

#![forbid(unsafe_code)]

#[macro_use]
extern crate derive_more;

// pub mod animator;
pub mod design_system;
pub mod element;
pub mod id;
pub mod layout;
// pub mod screen_info_notifier;
pub mod env;
pub mod rerender;
pub mod traits;
pub mod view;

pub use self::prelude::*;

// TODO: add:
// - ScrollBox layout element
// - Slider element

/// savory_elements prelude
pub mod prelude {
    pub use super::{
        design_system::{self, DesignSystem},
        // animator::{self, Animator},
        // screen_info_notifier::{self, NewScreenInfo, ScreenInfo, ScreenInfoNotifier},
        element::{
            button::{self, Button},
            progress_bar::{self, ProgressBar},
            radio::{self, Radio},
            switch::{self, Switch},
            text_input::{self, TextInput},
        },
        env::EnvExt,
        // id::Id,
        layout::flex::{self, Flex},
        rerender::RerenderRequested,
        traits::ExtendBuilder,
        view::{
            image::{self, Image},
            svg::{self, Svg},
            text::{self, Text},
        },
    };
    pub use derive_rich::Rich;
    pub use savory_elements_derive::Element;
}
