//! Reusable elements.

#[macro_use]
extern crate derive_more;

pub mod control;
pub mod display;
pub mod theme;

pub use self::prelude::*;

pub mod prelude {
    pub use super::{
        control::{
            button::{self, Button},
            checkbox::{self, Checkbox},
            dialog::{self, Dialog},
            entry::{self, Entry},
            menu_button::{self, MenuButton},
            progress_bar::{self, ProgressBar},
            radio::{self, Radio},
            spin_entry::{self, SpinEntry},
            switch::{self, Switch},
        },
        display::{
            flexbox::{self, Flexbox},
            header_bar::{self, HeaderBar},
            icon::{self, Icon},
            label::{self, Label},
            popover::{self, Popover},
        },
        theme::{Theme, ThemeChanged, ThemeLens},
    };
    pub use savory_macros::Element;
}

/// create style struct for element, and impl `From` for the created style and
/// it's substyle if there is any
#[macro_export]
macro_rules! style_type {
    ( $( $field:ident $(,)? )+ $( { $( $fn_name:ident() -> $substyle:path { $( $substyle_field_name:ident: $field_name:ident $(,)? )+ } $(,)? )+ } )? ) => {
        #[derive(Clone, Default, PartialEq, Rich)]
        pub struct Style {
            $(
                #[rich(write(style = compose), write)]
                pub $field: savory_html::css::Style,
            )+
        }

        $(
            impl Style {
                $(
                    pub fn $fn_name(&self) -> $substyle {
                        $substyle {
                            $(
                                $substyle_field_name: self.$field_name.clone(),
                            )+
                        }
                    }
                )+
            }
        )?
    }
}

/// create events struct for element, and impl `From` for the created struct and
/// it's subevents if there is any
#[macro_export]
macro_rules! events_type {
    ( $( $field:ident $(,)? )+ $( { $( $fn_name:ident() -> $subevents:path { $( $subevents_field_name:ident: $field_name:ident $(,)? )+ } )+ } )? ) => {
        #[derive(Rich)]
        pub struct Events<Msg> {
            $(
                #[rich(write(style = compose), write)]
                pub $field: savory_html::events::Events<Msg>,
            )+
        }

        impl<Msg> Clone for Events<Msg> {
            fn clone(&self) -> Self {
                Self {
                    $(
                        $field: self.$field.clone(),
                    )+
                }
            }
        }

        impl<Msg> Default for Events<Msg> {
            fn default() -> Self {
                Self {
                    $(
                        $field: savory_html::events::Events::default(),
                    )+
                }
            }
        }

        $(
            impl<Msg> Events<Msg> {
                $(
                    pub fn $fn_name(&self) -> $subevents {
                        $subevents {
                            $(
                                $subevents_field_name: self.$field_name.clone(),
                            )+
                        }
                    }
                )+
            }
        )?
    }
}
