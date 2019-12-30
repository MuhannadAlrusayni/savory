pub mod ant;
// pub mod default;
// pub mod matieral;

use crate::{css::Style, el::{
    button::Button,
    icon::{SvgIcon, HtmlIcon, UrlIcon},
    layout::flexbox::{self, Flexbox},
}};

pub trait Theme {
    // layout
    fn flexbox<Msg: 'static>(&self, _: &Flexbox<Msg>) -> Style;
    fn flexbox_item<Msg: 'static>(&self, _: &flexbox::Item<Msg>) -> Style;
    // fn grid(&self) -> Style;
    // icon
    fn svg_icon<Msg: 'static>(&self, _: &SvgIcon<Msg>) -> Style;
    fn html_icon(&self, _: &HtmlIcon) -> Style;
    fn url_icon(&self, _: &UrlIcon) -> Style;
    // button
    fn button(&self, _: &Button) -> Style;
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn primary_test() {
    // }
}
