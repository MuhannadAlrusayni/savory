pub mod ant;
// pub mod simple;
// pub mod matieral;

use crate::el::prelude::*;

pub trait Theme {
    fn flexbox<PMsg: 'static>(&self, _: &Flexbox<PMsg>) -> <Flexbox<PMsg> as Themeable>::StyleMap;

    fn flexbox_item<PMsg: 'static>(
        &self,
        _: &flexbox::Item<PMsg>,
    ) -> <flexbox::Item<PMsg> as Themeable>::StyleMap;

    fn popover<'a, PMsg, C, T>(&self, _: &Popover<'a, PMsg, C, T>) -> popover::Style;

    fn svg_icon<PMsg: 'static>(&self, _: &SvgIcon<PMsg>) -> <SvgIcon<PMsg> as Themeable>::StyleMap;

    fn html_icon<PMsg>(&self, _: &HtmlIcon<PMsg>) -> <HtmlIcon<PMsg> as Themeable>::StyleMap;

    fn url_icon<PMsg>(&self, _: &UrlIcon<PMsg>) -> <UrlIcon<PMsg> as Themeable>::StyleMap;

    fn button<PMsg>(&self, _: &Button<PMsg>) -> <Button<PMsg> as Themeable>::StyleMap;

    fn switch<PMsg>(&self, _: &Switch<PMsg>) -> <Switch<PMsg> as Themeable>::StyleMap;

    fn checkbox<PMsg>(&self, _: &Checkbox<PMsg>) -> <Checkbox<PMsg> as Themeable>::StyleMap;

    fn radio<PMsg>(&self, _: &Radio<PMsg>) -> <Radio<PMsg> as Themeable>::StyleMap;

    fn entry<PMsg>(&self, _: &Entry<PMsg>) -> <Entry<PMsg> as Themeable>::StyleMap;
}

pub trait Themeable {
    type StyleMap;
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn primary_test() {
    // }
}
