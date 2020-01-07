pub mod ant;
// pub mod simple;
// pub mod matieral;

use crate::{
    css::Style,
    el::{
        button::Button,
        icon::{HtmlIcon, SvgIcon, UrlIcon},
        layout::flexbox::{self, Flexbox},
    },
    render::Render,
};

pub trait Theme2<Msg: 'static, R: Render<Msg>> {
    fn style(&self, obj: R) -> R::StyleMap;
}

pub struct ThemeX;
pub struct ThemeY;

impl<Msg: 'static + Clone> Theme2<Msg, Flexbox<Msg>> for ThemeX {
    fn style(&self, obj: Flexbox<Msg>) -> <Flexbox<Msg> as Render<Msg>>::StyleMap {
        unimplemented!()
    }
}

pub trait Theme {
    // layout
    fn flexbox<Msg: 'static + Clone>(
        &self,
        _: &Flexbox<Msg>,
    ) -> <Flexbox<Msg> as Render<Msg>>::StyleMap;

    fn flexbox_item<Msg: 'static + Clone>(
        &self,
        _: &flexbox::Item<Msg>,
    ) -> <flexbox::Item<Msg> as Render<Msg>>::StyleMap;

    // fn grid(&self) -> Style;
    // icon

    fn svg_icon<Msg: 'static + Clone>(
        &self,
        _: &SvgIcon<Msg>,
    ) -> <SvgIcon<Msg> as Render<Msg>>::StyleMap;

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
