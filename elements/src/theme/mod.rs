//! Traits and types used to generate elements/views styles.

pub mod ant;

use crate::prelude::*;
use std::{ops::Deref, rc::Rc};

/// Type that hold `ThemeImpl` trait
#[derive(Clone)]
pub struct Theme(Rc<dyn ThemeImpl>);

/// The defaut theme is Ant (for now :D)
impl Default for Theme {
    fn default() -> Self {
        Self(Rc::new(ant::Ant::default()))
    }
}

/// Subscribe to theme changes
#[derive(Clone)]
pub struct ThemeChanged(pub Theme);

impl Deref for Theme {
    type Target = dyn ThemeImpl;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// Trait used by theme types to generate elements/views styles.
pub trait ThemeImpl {
    // views
    fn flexbox<'a>(&self) -> flexbox::ThemeStyler<'a>;

    fn flexbox_item<'a>(&self) -> flexbox::item::ThemeStyler<'a>;

    fn svg_icon<'a>(&self) -> icon::svg::ThemeStyler<'a>;

    fn html_icon<'a>(&self) -> icon::html::ThemeStyler<'a>;

    fn url_icon<'a>(&self) -> icon::url::ThemeStyler<'a>;

    fn header_bar<'a>(&self) -> header_bar::ThemeStyler<'a>;

    fn label<'a>(&self) -> label::ThemeStyler<'a>;

    // elments
    fn popover<'a>(&self) -> popover::ThemeStyler<'a>;

    fn button<'a>(&self) -> button::ThemeStyler<'a>;

    fn switch<'a>(&self) -> switch::ThemeStyler<'a>;

    fn checkbox<'a>(&self) -> checkbox::ThemeStyler<'a>;

    fn radio<'a>(&self) -> radio::ThemeStyler<'a>;

    fn entry<'a>(&self) -> entry::ThemeStyler<'a>;

    fn spin_entry<'a>(&self) -> spin_entry::ThemeStyler<'a>;

    fn dialog<'a>(&self) -> dialog::ThemeStyler<'a>;

    fn progress_bar<'a>(&self) -> progress_bar::ThemeStyler<'a>;
}

/// Trait used to exteract element/view data that is needed by the theme to
/// generate style
pub trait ThemeLens<'a> {
    type Lens: 'a;

    fn theme_lens(&'a self) -> Self::Lens;
}

/// Type that hold function which takes `&E` and return `Style`, this function
/// is used every time when elemet/view get viewed.
///
/// Usally `&E` is reference to type `<Element as ThemeLens>::Lens` or element
/// type.
pub struct Styler<E, S>(Rc<dyn Fn(&E) -> S>);

impl<E, S> Styler<E, S> {
    pub fn new(styler: impl Fn(&E) -> S + 'static) -> Self {
        Styler(Rc::new(styler))
    }

    pub fn get(&self, e: &E) -> S {
        self.0(e)
    }
}

impl<E, S, T> From<T> for Styler<E, S>
where
    T: Fn(&E) -> S + 'static,
{
    fn from(val: T) -> Self {
        Self(Rc::new(val))
    }
}

impl<E, S> From<Rc<dyn Fn(&E) -> S>> for Styler<E, S> {
    fn from(val: Rc<dyn Fn(&E) -> S>) -> Self {
        Self(val)
    }
}

impl<E, S> From<Rc<Styler<E, S>>> for Styler<E, S> {
    fn from(val: Rc<Styler<E, S>>) -> Self {
        Self(Rc::clone(&val.0))
    }
}

impl<E, S> Clone for Styler<E, S> {
    fn clone(&self) -> Self {
        Styler(Rc::clone(&self.0))
    }
}

impl<E, S: Default> Default for Styler<E, S> {
    fn default() -> Self {
        Self(Rc::new(|_| S::default()))
    }
}
