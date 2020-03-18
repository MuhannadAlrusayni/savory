use crate::{
    css::{self, box_align::*, flexbox::*, gap::Gap, values as val},
    prelude::*,
};
use derive_rich::Rich;
use std::default::Default;

#[derive(Clone, Rich, Default)]
pub struct Flexbox<PMsg: 'static> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,
    #[rich(read, write(style = compose))]
    items: Vec<Item<PMsg>>,
    #[rich(read(copy), value_fns = {
        row = val::Row,
        reversed_row = val::RowReverse,
        column = val::Column,
        reversed_column = val::ColumnReverse,
    })]
    direction: Option<Direction>,
    #[rich(read(copy), value_fns = {
        wrap = val::Wrap,
        no_wrap = val::Nowrap,
        reversed_wrap = val::WrapReverse,
    })]
    wrap: Option<Wrap>,
    #[rich(read(copy), write)]
    justify_content: Option<JustifyContent>,
    #[rich(read(copy), write)]
    align_items: Option<AlignItems>,
    #[rich(read(copy), write)]
    align_content: Option<AlignContent>,
    #[rich(read(copy), write)]
    gap: Option<Gap>,
}

impl<PMsg: 'static> Flexbox<PMsg> {
    pub fn new() -> Self {
        Self {
            events: Events::default(),
            user_style: UserStyle::default(),
            items: vec![],
            direction: None,
            wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            gap: None,
        }
    }

    pub fn item() -> Item<PMsg> {
        Item::new()
    }

    pub fn item_with(content: impl Into<ContentArg<PMsg>>) -> Item<PMsg> {
        Item::with_content(content)
    }

    pub fn add(&mut self, item: impl Into<Item<PMsg>>) -> &mut Self {
        self.items.push(item.into());
        self
    }

    pub fn try_add(&mut self, item: Option<impl Into<Item<PMsg>>>) -> &mut Self {
        if let Some(item) = item {
            self.items.push(item.into())
        }
        self
    }

    pub fn add_and(
        &mut self,
        config_item: impl FnOnce(&mut Item<PMsg>) -> &mut Item<PMsg> + 'static,
    ) -> &mut Self {
        let mut item = Self::item();
        config_item(&mut item);
        self.items.push(item);
        self
    }

    pub fn try_add_and(
        &mut self,
        node: Option<Node<PMsg>>,
        config_item: impl FnOnce(&mut Item<PMsg>) -> &mut Item<PMsg> + 'static,
    ) -> &mut Self {
        if let Some(node) = node {
            let mut item = Self::item_with(node);
            config_item(&mut item);
            self.items.push(item);
        }
        self
    }

    pub fn add_items(&mut self, items: impl IntoIterator<Item = Node<PMsg>>) -> &mut Self {
        self.items
            .extend(items.into_iter().map(|node| Item::from(node)));
        self
    }

    pub fn normal(&mut self) -> &mut Self {
        self.set_justify_content(val::Normal)
            .set_align_content(val::Normal)
            .set_align_items(val::Normal)
    }

    pub fn stretch(&mut self) -> &mut Self {
        self.set_justify_content(val::Stretch)
            .set_align_content(val::Stretch)
            .set_align_items(val::Stretch)
    }

    pub fn center(&mut self) -> &mut Self {
        self.set_justify_content(val::Center)
            .set_align_content(val::Center)
            .set_align_items(val::Center)
    }

    pub fn start(&mut self) -> &mut Self {
        self.set_justify_content(val::Start)
            .set_align_content(val::Start)
            .set_align_items(val::Start)
    }

    pub fn end(&mut self) -> &mut Self {
        self.set_justify_content(val::End)
            .set_align_content(val::End)
            .set_align_items(val::End)
    }

    pub fn space_between(&mut self) -> &mut Self {
        self.set_justify_content(val::SpaceBetween)
            .set_align_content(val::SpaceBetween)
    }

    pub fn space_around(&mut self) -> &mut Self {
        self.set_justify_content(val::SpaceAround)
            .set_align_content(val::SpaceAround)
    }

    pub fn space_evenly(&mut self) -> &mut Self {
        self.set_justify_content(val::SpaceEvenly)
            .set_align_content(val::SpaceEvenly)
    }

    pub fn full_size(&mut self) -> &mut Self {
        self.and_user_style(|conf| conf.and_size(|size| size.full()))
    }
}

pub type Style = css::Style;
pub type UserStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for Flexbox<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.flexbox(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        div![
            self.events.events.clone(),
            style,
            // items
            self.items.iter().map(|item| item.render(theme)),
        ]
    }
}

// ---- Flexbox Item ----

#[derive(Clone, Rich, Default)]
pub struct Item<PMsg: 'static> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: ItemUserStyle,
    #[rich(read, write(style = compose))]
    content: Vec<Node<PMsg>>,
    #[rich(read(copy), write)]
    order: Option<Order>,
    #[rich(read(copy), write)]
    grow: Option<Grow>,
    #[rich(read(copy), write)]
    shrink: Option<Shrink>,
    #[rich(read(copy), write)]
    basis: Option<Basis>,
    #[rich(read(copy), value_fns = {
        auto = val::Auto,
        normal = val::Normal,
        stretch = val::Stretch,
        center = val::Center,
        start = val::Start,
        end = val::End,
    })]
    align_self: Option<AlignSelf>,
    #[rich(read(copy, rename = is_flatten), value_fns = { flatten = true, wrapped = false })]
    flatten: bool,
}

impl<PMsg: 'static> From<Vec<Node<PMsg>>> for Item<PMsg> {
    fn from(source: Vec<Node<PMsg>>) -> Self {
        Item::with_content(source)
    }
}

impl<PMsg: 'static> From<Node<PMsg>> for Item<PMsg> {
    fn from(source: Node<PMsg>) -> Self {
        Item::with_content(source)
    }
}

impl<PMsg: 'static> Item<PMsg> {
    pub fn new() -> Self {
        Self {
            events: Events::default(),
            user_style: ItemUserStyle::default(),
            content: vec![],
            order: None,
            grow: None,
            shrink: None,
            basis: None,
            align_self: None,
            flatten: true,
        }
    }

    pub fn with_content(arg: impl Into<ContentArg<PMsg>>) -> Self {
        let mut item = Self::new();
        item.set_content(arg);
        item
    }

    pub fn auto_margin(&mut self) -> &mut Self {
        self.and_user_style(|conf| conf.and_margin(|margin| margin.auto()))
    }

    pub fn set_content(&mut self, arg: impl Into<ContentArg<PMsg>>) -> &mut Self {
        self.content = arg.into().0;
        self
    }

    pub fn group(&mut self, group_id: impl Into<Order>) -> &mut Self {
        self.set_order(group_id)
    }
}

pub struct ContentArg<PMsg: 'static>(Vec<Node<PMsg>>);

impl<PMsg: 'static> From<Node<PMsg>> for ContentArg<PMsg> {
    fn from(source: Node<PMsg>) -> Self {
        Self(vec![source])
    }
}

impl<PMsg: 'static> From<Vec<Node<PMsg>>> for ContentArg<PMsg> {
    fn from(source: Vec<Node<PMsg>>) -> Self {
        Self(source)
    }
}

pub type ItemStyle = css::Style;
pub type ItemUserStyle = css::Style;

impl<PMsg: 'static> Render<PMsg> for Item<PMsg> {
    type View = Vec<Node<PMsg>>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.flexbox_item(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        if self.is_flatten() {
            self.content
                .clone()
                .into_iter()
                .map(|mut node| {
                    // add self.events to every node
                    for event in self.events.events.clone().into_iter() {
                        node.add_listener(event);
                    }
                    // add style to every node
                    if let Some(style) = style.to_seed_style() {
                        for (key, value) in style.vals.into_iter() {
                            node.add_style(key, value);
                        }
                    }
                    node
                })
                .collect()
        } else {
            vec![div![
                self.events.events.clone(),
                style,
                // child
                self.content.clone()
            ]]
        }
    }
}
