pub mod ant;
// pub mod simple;
// pub mod matieral;

use crate::el::prelude::*;

pub trait Theme {
    fn flexbox<Msg: 'static + Clone>(
        &self,
        _: &Flexbox<Msg>,
    ) -> <Flexbox<Msg> as Themeable>::StyleMap;

    fn flexbox_item<Msg: 'static + Clone>(
        &self,
        _: &flexbox::Item<Msg>,
    ) -> <flexbox::Item<Msg> as Themeable>::StyleMap;

    fn svg_icon<Msg: 'static + Clone>(
        &self,
        _: &SvgIcon<Msg>,
    ) -> <SvgIcon<Msg> as Themeable>::StyleMap;

    fn html_icon(&self, _: &HtmlIcon) -> <HtmlIcon as Themeable>::StyleMap;

    fn url_icon(&self, _: &UrlIcon) -> <UrlIcon as Themeable>::StyleMap;

    fn button(&self, _: &Button) -> <Button as Themeable>::StyleMap;

    fn switch(&self, _: &Switch) -> <Switch as Themeable>::StyleMap;

    fn checkbox(&self, _: &Checkbox) -> <Checkbox as Themeable>::StyleMap;

    fn radio(&self, _: &Radio) -> <Radio as Themeable>::StyleMap;
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
