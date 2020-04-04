//! Traits used to generate elements styles.

pub mod ant;

use crate::prelude::*;
use std::{ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct Theme(Rc<dyn ThemeImpl>);

impl Default for Theme {
    fn default() -> Self {
        Self(Rc::new(ant::Ant::default()))
    }
}

/// Subscribe to theme changes
pub struct ThemeChanged(pub Theme);

impl Deref for Theme {
    type Target = dyn ThemeImpl;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub trait ThemeImpl {
    fn flexbox<'a>(&self, el: flexbox::FlexboxLens<'a>) -> flexbox::Style;

    fn flexbox_item<'a>(&self, _: flexbox::item::ItemLens<'a>) -> flexbox::item::Style;

    fn popover<'a>(&self, _: popover::PopoverLens<'a>) -> popover::Style;

    fn svg_icon<'a>(&self, _: icon::svg::SvgLens<'a>) -> icon::svg::Style;

    fn html_icon<'a>(&self, _: icon::html::HtmlLens<'a>) -> icon::html::Style;

    fn url_icon<'a>(&self, _: icon::url::UrlLens<'a>) -> icon::url::Style;

    fn button<'a>(&self, _: button::ButtonLens<'a>) -> button::Style;

    fn switch<'a>(&self, _: switch::SwitchLens<'a>) -> switch::Style;

    fn checkbox<'a>(&self, _: checkbox::CheckboxLens<'a>) -> checkbox::Style;

    fn radio<'a>(&self, _: radio::RadioLens<'a>) -> radio::Style;

    fn entry<'a>(&self, _: entry::EntryLens<'a>) -> entry::Style;

    fn spin_entry<'a>(&self, _: spin_entry::SpinEntryLens<'a>) -> spin_entry::Style;

    fn dialog<'a>(&self, _: dialog::DialogLens<'a>) -> dialog::Style;

    fn header_bar<'a>(&self, _: header_bar::HeaderBarLens<'a>) -> header_bar::Style;

    fn label<'a>(&self, _: label::LabelLens<'a>) -> label::Style;

    fn progress_bar<'a>(&self, _: progress_bar::ProgressBarLens<'a>) -> progress_bar::Style;
}

pub trait ThemeLens<'a> {
    type Lens: 'a;

    fn theme_lens(&'a self) -> Self::Lens;
}
