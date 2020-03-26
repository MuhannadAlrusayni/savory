//! Traits used to generate elements styles.

pub mod ant;

use crate::el::prelude::*;

pub trait Theme {
    fn flexbox<PMsg: 'static>(&self, _: &Flexbox<PMsg>) -> flexbox::Style;

    fn flexbox_item<PMsg: 'static>(&self, _: &flexbox::Item<PMsg>) -> flexbox::ItemStyle;

    fn popover<'a, PMsg, C, T>(&self, _: &Popover<'a, PMsg, C, T>) -> popover::Style;

    fn svg_icon<PMsg: 'static>(&self, _: &SvgIcon<PMsg>) -> icon::SvgStyle;

    fn html_icon<PMsg>(&self, _: &HtmlIcon<PMsg>) -> icon::HtmlStyle;

    fn url_icon<PMsg>(&self, _: &UrlIcon<PMsg>) -> icon::UrlStyle;

    fn button<PMsg>(&self, _: &Button<PMsg>) -> button::Style;

    fn switch<PMsg>(&self, _: &Switch<PMsg>) -> switch::Style;

    fn checkbox<PMsg>(&self, _: &Checkbox<PMsg>) -> checkbox::Style;

    fn radio<PMsg>(&self, _: &Radio<PMsg>) -> radio::Style;

    fn entry<PMsg>(&self, _: &Entry<PMsg>) -> entry::Style;

    fn spin_entry<PMsg>(&self, _: &SpinEntry<PMsg>) -> spin_entry::Style;

    fn dialog<PMsg, C>(&self, _: &Dialog<PMsg, C>) -> dialog::Style;

    fn header_bar<PMsg>(&self, _: &HeaderBar<PMsg>) -> header_bar::Style;

    fn label<PMsg>(&self, _: &Label<PMsg>) -> label::Style;

    fn progress_bar<PMsg>(&self, _: &ProgressBar<PMsg>) -> progress_bar::Style;
}
