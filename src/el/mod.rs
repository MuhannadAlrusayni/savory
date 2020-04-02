//! Reusable elements.

pub mod control;
pub mod display;

pub use self::prelude::*;

use crate::{css, events};
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Add, AddAssign},
};

pub mod prelude {
    pub use super::{
        control::{
            button::{self, Button, ButtonLens},
            checkbox::{self, Checkbox, CheckboxLens},
            dialog::{self, Dialog, DialogLens},
            entry::{self, Entry, EntryLens},
            menu_button::{self, MenuButton, MenuButtonLens},
            progress_bar::{self, ProgressBar, ProgressBarLens},
            radio::{self, Radio, RadioLens},
            spin_entry::{self, SpinEntry, SpinEntryLens},
            switch::{self, Switch, SwitchLens},
        },
        display::{
            flexbox::{self, Flexbox, FlexboxLens},
            header_bar::{self, HeaderBar, HeaderBarLens},
            icon::{
                self, HtmlIcon, HtmlIconLens, Icon, SvgIcon, SvgIconLens, UrlIcon, UrlIconLens,
            },
            label::{self, Label, LabelLens},
            popover::{self, Popover, PopoverLens},
        },
    };
    pub use seed::prelude::Node;
}

#[derive(Index, IndexMut, IntoIterator, Clone, From)]
pub struct Events<Msg>(HashMap<Cow<'static, str>, events::Events<Msg>>);

impl<Msg> Default for Events<Msg> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<Msg> Events<Msg> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        get_events: impl FnOnce(events::Events<Msg>) -> events::Events<Msg>,
    ) -> Option<events::Events<Msg>> {
        self.0
            .insert(name.into(), get_events(events::Events::default()))
    }

    pub fn get(&mut self, name: &str) -> Option<&events::Events<Msg>> {
        self.0.get(name.into())
    }

    pub fn remove(&mut self, name: impl AsRef<Cow<'static, str>>) -> Option<events::Events<Msg>> {
        self.0.remove(name.as_ref())
    }
}

#[derive(Default, Index, IndexMut, IntoIterator, Clone, Debug, From)]
pub struct Style(HashMap<Cow<'static, str>, css::Style>);

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        get_style: impl FnOnce(css::Style) -> css::Style,
    ) -> Option<css::Style> {
        self.0.insert(name.into(), get_style(css::Style::default()))
    }

    pub fn get(&mut self, name: &str) -> Option<&css::Style> {
        self.0.get(name.into())
    }

    pub fn remove(&mut self, name: impl AsRef<Cow<'static, str>>) -> Option<css::Style> {
        self.0.remove(name.as_ref())
    }

    pub fn sub_style(
        &mut self,
        names: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Style {
        todo!()
        // let mut style = Style::default();
        // for (dest, name) in names.into_iter() {
        //     let name = name.into();
        //     let dest = dest.into();
        //     style[&dest] = self[&name];
        // }
        // style
    }
}

impl Add for Style {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for Style {
    fn add_assign(&mut self, other: Self) {
        for (name, style) in other.0.into_iter() {
            match self.0.get_mut(&name) {
                Some(val) => *val += style,
                None => {
                    self.0.insert(name, style);
                }
            }
        }
    }
}
