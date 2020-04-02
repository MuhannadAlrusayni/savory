use crate::{
    css::{box_align::*, flexbox::*, gap::Gap, values as val},
    prelude::*,
};
use derive_rich::Rich;
use std::default::Default;

#[derive(Element, Rich, Default)]
pub struct Flexbox<'a, PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    #[rich(read, write(style = compose))]
    items: Vec<Item<'a, PMsg>>,
    #[rich(read(copy), value_fns = {
        row = val::Row,
        reversed_row = val::RowReverse,
        column = val::Column,
        reversed_column = val::ColumnReverse,
    })]
    #[element(theme_lens)]
    direction: Option<Direction>,
    #[rich(read(copy), value_fns = {
        wrap = val::Wrap,
        no_wrap = val::Nowrap,
        reversed_wrap = val::WrapReverse,
    })]
    #[element(theme_lens)]
    wrap: Option<Wrap>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    justify_content: Option<JustifyContent>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    align_items: Option<AlignItems>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    align_content: Option<AlignContent>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    gap: Option<Gap>,
}

impl<'a, PMsg> Flexbox<'a, PMsg> {
    pub fn new() -> Self {
        Self {
            events: Events::default(),
            user_style: Style::default(),
            items: vec![],
            direction: None,
            wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            gap: None,
        }
    }

    pub fn item_with(content: &'a dyn Render<View = Node<PMsg>>) -> Item<'a, PMsg> {
        Item::new(content)
    }

    pub fn add(mut self, el: &'a dyn Render<View = Node<PMsg>>) -> Self {
        self.items.push(Item::new(el));
        self
    }

    pub fn try_add(self, item: Option<&'a dyn Render<View = Node<PMsg>>>) -> Self {
        if let Some(item) = item {
            self.add(item)
        } else {
            self
        }
    }

    pub fn add_item(mut self, item: impl Into<Item<'a, PMsg>>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn try_add_item(self, item: Option<impl Into<Item<'a, PMsg>>>) -> Self {
        if let Some(item) = item {
            self.add_item(item)
        } else {
            self
        }
    }

    pub fn add_item_and(
        mut self,
        content: &'a dyn Render<View = Node<PMsg>>,
        config_item: impl FnOnce(Item<'a, PMsg>) -> Item<'a, PMsg> + 'static,
    ) -> Self {
        self.items.push(config_item(Item::new(content)));
        self
    }

    pub fn try_add_item_and(
        self,
        node: Option<&'a dyn Render<View = Node<PMsg>>>,
        config_item: impl FnOnce(Item<'a, PMsg>) -> Item<'a, PMsg> + 'static,
    ) -> Self {
        if let Some(node) = node {
            self.add_item_and(node, config_item)
        } else {
            self
        }
    }

    pub fn add_items(
        mut self,
        items: impl IntoIterator<Item = &'a dyn Render<View = Node<PMsg>>>,
    ) -> Self {
        self.items
            .extend(items.into_iter().map(|node| Item::new(node)));
        self
    }

    pub fn normal(self) -> Self {
        self.set_justify_content(val::Normal)
            .set_align_content(val::Normal)
            .set_align_items(val::Normal)
    }

    pub fn stretch(self) -> Self {
        self.set_justify_content(val::Stretch)
            .set_align_content(val::Stretch)
            .set_align_items(val::Stretch)
    }

    pub fn center(self) -> Self {
        self.set_justify_content(val::Center)
            .set_align_content(val::Center)
            .set_align_items(val::Center)
    }

    pub fn start(self) -> Self {
        self.set_justify_content(val::Start)
            .set_align_content(val::Start)
            .set_align_items(val::Start)
    }

    pub fn end(self) -> Self {
        self.set_justify_content(val::End)
            .set_align_content(val::End)
            .set_align_items(val::End)
    }

    pub fn space_between(self) -> Self {
        self.set_justify_content(val::SpaceBetween)
            .set_align_content(val::SpaceBetween)
    }

    pub fn space_around(self) -> Self {
        self.set_justify_content(val::SpaceAround)
            .set_align_content(val::SpaceAround)
    }

    pub fn space_evenly(self) -> Self {
        self.set_justify_content(val::SpaceEvenly)
            .set_align_content(val::SpaceEvenly)
    }

    pub fn full_size(self) -> Self {
        todo!()
        // self.and_user_style(|conf| conf.and_size(|size| size.full()))
    }
}

impl<'a, PMsg> Render for Flexbox<'a, PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.flexbox(self.theme_lens())
    }

    fn render_with_style(&self, theme: &Theme, style: Style) -> Self::View {
        todo!()
        // div!()
        //     .set(att::class("flexbox"))
        //     .set(&self.events["flexbox"])
        //     .set(style["flexbox"])
        //     .add(
        //         self.items
        //             .iter()
        //             .map(|item| item.render(theme))
        //             .collect::<Vec<Node<PMsg>>>(),
        //     )
    }
}

// ---- Flexbox Item ----

#[derive(Rich, Element)]
pub struct Item<'a, PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    #[rich(read, write(style = compose))]
    content: &'a dyn Render<View = Node<PMsg>>,
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

impl<'a, PMsg> Item<'a, PMsg> {
    pub fn new(content: &'a dyn Render<View = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            user_style: Style::default(),
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
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.flexbox_item(self.theme_lens())
    }

    fn render_with_style(&self, theme: &Theme, style: Style) -> Self::View {
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
