use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::{
    css::{box_align::*, flexbox::*, values as val},
    prelude::*,
};
use std::default::Default;

#[derive(Rich, Element)]
pub struct Item<'a, PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,
    #[rich(read, write(style = compose))]
    content: &'a dyn Render<Output = Node<PMsg>>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    order: Option<Order>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    grow: Option<Grow>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    shrink: Option<Shrink>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    basis: Option<Basis>,
    #[rich(read(copy), value_fns = {
        auto = val::Auto,
        normal = val::Normal,
        stretch = val::Stretch,
        center = val::Center,
        start = val::Start,
        end = val::End,
    })]
    #[element(theme_lens)]
    align_self: Option<AlignSelf>,
    #[rich(read(copy, rename = is_flatten), value_fns = { flatten = true, wrapped = false })]
    #[element(theme_lens)]
    flatten: bool,
}

crate::style_type! {
    item,
}

crate::events_type! {
    item,
}

impl<'a, PMsg> Item<'a, PMsg> {
    pub fn new(content: &'a dyn Render<Output = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            style: None,
            content: content,
            order: None,
            grow: None,
            shrink: None,
            basis: None,
            align_self: None,
            flatten: true,
        }
    }

    pub fn auto_margin(self) -> Self {
        todo!()
        // self.and_user_style(|conf| conf.and_margin(|margin| margin.auto()))
    }

    pub fn group(self, group_id: impl Into<Order>) -> Self {
        self.set_order(group_id)
    }
}

impl<'a, PMsg> Render for Item<'a, PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        todo!()
        // if self.is_flatten() {
        //     self.content.render(theme)
        // } else {
        //     div!()
        //         .add(self.content.render(theme))
        //         .set(att::class("flexbox-item"))
        // }
        // .set(&self.events["flexbox-item"])
        // .set(style["flexbox-item"])
    }
}

// style_type! {
//     flexbox,
// }
// pub struct Style {
//     flexbox: css::Style,
// }

// pub struct Events<PMsg> {
//     flexbox: events::Events<PMsg>,
// }
