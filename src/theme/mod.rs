//! Traits used to generate elements styles.

pub mod ant;

use crate::prelude::*;
use std::{ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct Theme(Rc<dyn ThemeImpl>);

impl Deref for Theme {
    type Target = dyn ThemeImpl;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub trait ThemeLens<'a> {
    type Lens: 'a;

    fn theme_lens(&'a self) -> Self::Lens;
}

pub trait ThemeImpl {
    fn flexbox<'a>(&self, el: flexbox::FlexboxLens<'a>) -> Style;

    fn flexbox_item<'a>(&self, _: flexbox::ItemLens<'a>) -> Style;

    fn popover<'a>(&self, _: popover::PopoverLens<'a>) -> Style;

    fn svg_icon<'a>(&self, _: icon::SvgIconLens<'a>) -> Style;

    fn html_icon<'a>(&self, _: icon::HtmlIconLens<'a>) -> Style;

    fn url_icon<'a>(&self, _: icon::UrlIconLens<'a>) -> Style;

    fn button<'a>(&self, _: button::ButtonLens<'a>) -> Style;

    fn switch<'a>(&self, _: switch::SwitchLens<'a>) -> Style;

    fn checkbox<'a>(&self, _: checkbox::CheckboxLens<'a>) -> Style;

    fn radio<'a>(&self, _: radio::RadioLens<'a>) -> Style;

    fn entry<'a>(&self, _: entry::EntryLens<'a>) -> Style;

    fn spin_entry<'a>(&self, _: spin_entry::SpinEntryLens<'a>) -> Style;

    fn dialog<'a>(&self, _: dialog::DialogLens<'a>) -> Style;

    fn header_bar<'a>(&self, _: header_bar::HeaderBarLens<'a>) -> Style;

    fn label<'a>(&self, _: label::LabelLens<'a>) -> Style;

    fn progress_bar<'a>(&self, _: progress_bar::ProgressBarLens<'a>) -> Style;
}
