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
    fn screen_info(&self, width: u32, height: u32) -> ScreenInfo;

    // NOTE: generice views uses ThemeStyler<'a> to keep this trait as trait
    // object safe. This is needed so we can store ThemeImpl in the Theme
    // struct.

    // views
    fn flexbox<'a>(&self) -> flexbox::ThemeStyler<'a>;

    fn flexbox_item<'a>(&self) -> flexbox::item::ThemeStyler<'a>;

    fn svg_icon<'a>(&self) -> icon::svg::ThemeStyler<'a>;

    fn html_icon(&self) -> <icon::Html as Stylable>::Styler;

    fn url_icon(&self) -> <icon::Url as Stylable>::Styler;

    fn label(&self) -> <Label as Stylable>::Styler;

    // elments
    fn header_bar(&self) -> <HeaderBar as Stylable>::Styler;

    fn popover(&self) -> <Popover as Stylable>::Styler;

    fn button(&self) -> <Button as Stylable>::Styler;

    fn switch(&self) -> <Switch as Stylable>::Styler;

    fn checkbox(&self) -> <Checkbox as Stylable>::Styler;

    fn radio(&self) -> <Radio as Stylable>::Styler;

    fn entry(&self) -> <Entry as Stylable>::Styler;

    fn spin_entry(&self) -> <SpinEntry as Stylable>::Styler;

    fn dialog(&self) -> <Dialog as Stylable>::Styler;

    fn progress_bar(&self) -> <ProgressBar as Stylable>::Styler;
}

/// Trait used to exteract element/view data that is needed by the theme to
/// generate style
pub trait ThemeLens<'a> {
    type Lens: 'a;

    fn theme_lens(&'a self) -> Self::Lens;
}
