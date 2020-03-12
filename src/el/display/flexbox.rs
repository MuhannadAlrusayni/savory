use crate::{
    css::{
        self, background::Background, border::Border, box_align::*, flexbox::*, gap::Gap,
        margin::Margin, padding::Padding, size::Size, values as val,
    },
    prelude::*,
};
use derive_rich::Rich;
use std::default::Default;

#[derive(Clone, Rich, Default)]
pub struct Flexbox<PMsg: 'static> {
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    pub items: Vec<Item<PMsg>>,
    // properties
    #[rich(value_fns = {
        row = val::Row,
        reversed_row = val::RowReverse,
        column = val::Column,
        reversed_column = val::ColumnReverse,
    })]
    pub direction: Option<Direction>,
    #[rich(value_fns = {
        wrap = val::Wrap,
        no_wrap = val::Nowrap,
        reversed_wrap = val::WrapReverse,
    })]
    pub wrap: Option<Wrap>,
    #[rich(write)]
    pub justify_content: Option<JustifyContent>,
    #[rich(write)]
    pub align_items: Option<AlignItems>,
    #[rich(write)]
    pub align_content: Option<AlignContent>,
    #[rich(write)]
    pub gap: Option<Gap>,
    #[rich(write(style = compose))]
    pub size: Size,
    // TODO: remove this in favor for style
    #[rich(write(style = compose))]
    pub border: Border,
    // TODO: remove this in favor for style
    #[rich(write(style = compose))]
    pub background: Background,
    #[rich(write(style = compose))]
    pub margin: Margin,
    #[rich(write(style = compose))]
    pub padding: Padding,
    #[rich(write(style = compose))]
    pub style: css::Style,
}

impl<PMsg: 'static> Flexbox<PMsg> {
    pub fn new() -> Self {
        Self {
            events: Events::default(),
            items: vec![],
            direction: None,
            wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            gap: None,
            size: Size::default(),
            border: Border::default(),
            background: Background::default(),
            margin: Margin::default(),
            padding: Padding::default(),
            style: css::Style::default(),
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

    pub fn add_and(
        &mut self,
        config_item: impl FnOnce(&mut Item<PMsg>) -> &mut Item<PMsg> + 'static,
    ) -> &mut Self {
        let mut item = Self::item();
        config_item(&mut item);
        self.items.push(item);
        self
    }

    pub fn items(&mut self, items: impl IntoIterator<Item = Item<PMsg>>) -> &mut Self {
        self.items.extend(items);
        self
    }

    pub fn normal(&mut self) -> &mut Self {
        self.justify_content(val::Normal)
            .align_content(val::Normal)
            .align_items(val::Normal)
    }

    pub fn stretch(&mut self) -> &mut Self {
        self.justify_content(val::Stretch)
            .align_content(val::Stretch)
            .align_items(val::Stretch)
    }

    pub fn center(&mut self) -> &mut Self {
        self.justify_content(val::Center)
            .align_content(val::Center)
            .align_items(val::Center)
    }

    pub fn start(&mut self) -> &mut Self {
        self.justify_content(val::Start)
            .align_content(val::Start)
            .align_items(val::Start)
    }

    pub fn end(&mut self) -> &mut Self {
        self.justify_content(val::End)
            .align_content(val::End)
            .align_items(val::End)
    }

    pub fn space_between(&mut self) -> &mut Self {
        self.justify_content(val::SpaceBetween)
            .align_content(val::SpaceBetween)
    }

    pub fn space_around(&mut self) -> &mut Self {
        self.justify_content(val::SpaceAround)
            .align_content(val::SpaceAround)
    }

    pub fn space_evenly(&mut self) -> &mut Self {
        self.justify_content(val::SpaceEvenly)
            .align_content(val::SpaceEvenly)
    }

    pub fn full_size(&mut self) -> &mut Self {
        self.size(|size| size.full())
    }
}

pub type Style = css::Style;

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

// TODO: add collapse propertie
#[derive(Clone, Rich, Default)]
pub struct Item<PMsg: 'static> {
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    pub content: Vec<Node<PMsg>>,
    // propertie
    #[rich(write)]
    pub order: Option<Order>,
    #[rich(write)]
    pub grow: Option<Grow>,
    #[rich(write)]
    pub shrink: Option<Shrink>,
    #[rich(write)]
    pub basis: Option<Basis>,
    #[rich(value_fns = {
        auto = val::Auto,
        normal = val::Normal,
        stretch = val::Stretch,
        center = val::Center,
        start = val::Start,
        end = val::End,
    })]
    pub align_self: Option<AlignSelf>,
    #[rich(write(style = compose))]
    pub size: Size,
    #[rich(write(style = compose))]
    pub border: Border,
    #[rich(write(style = compose))]
    pub background: Background,
    #[rich(write(style = compose))]
    pub margin: Margin,
    #[rich(write(style = compose))]
    pub padding: Padding,
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
            content: vec![],
            order: None,
            grow: None,
            shrink: None,
            basis: None,
            align_self: None,
            size: Size::default(),
            border: Border::default(),
            background: Background::default(),
            margin: Margin::default(),
            padding: Padding::default(),
            flatten: true,
        }
    }

    pub fn with_content(arg: impl Into<ContentArg<PMsg>>) -> Self {
        let mut item = Self::new();
        item.content(arg);
        item
    }

    pub fn content(&mut self, arg: impl Into<ContentArg<PMsg>>) -> &mut Self {
        self.content = arg.into().0;
        self
    }

    pub fn auto_margin(&mut self) -> &mut Self {
        self.margin(|margin| margin.auto())
    }

    pub fn group(&mut self, group_id: impl Into<Order>) -> &mut Self {
        self.order(group_id)
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
