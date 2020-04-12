pub mod item;

use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::{
    css::{box_align::*, flexbox::*, values as val, Gap},
    prelude::*,
};

use item::Item;

#[derive(Element, Rich)]
#[element(style(flexbox), events(flexbox))]
pub struct Flexbox<'a, PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<'a, PMsg>>,
    #[rich(write(rename = theme))]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write(style = compose), write(rename = items))]
    pub items: Vec<Item<'a, PMsg>>,
    #[rich(value_fns = {
        row = val::Row,
        reversed_row = val::RowReverse,
        column = val::Column,
        reversed_column = val::ColumnReverse,
    })]
    #[element(theme_lens)]
    pub direction: Option<Direction>,
    #[rich(value_fns = {
        wrap = val::Wrap,
        no_wrap = val::Nowrap,
        reversed_wrap = val::WrapReverse,
    })]
    #[element(theme_lens)]
    pub wrap: Option<Wrap>,
    #[rich(write(rename = justify_content))]
    #[element(theme_lens)]
    pub justify_content: Option<JustifyContent>,
    #[rich(write(rename = align_items))]
    #[element(theme_lens)]
    pub align_items: Option<AlignItems>,
    #[rich(write(rename = align_content))]
    #[element(theme_lens)]
    pub align_content: Option<AlignContent>,
    #[rich(write(rename = gap))]
    #[element(theme_lens)]
    pub gap: Option<Gap>,
}

impl<'a, PMsg> Default for Flexbox<'a, PMsg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, PMsg> Flexbox<'a, PMsg> {
    pub fn new() -> Self {
        Self {
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            items: vec![],
            direction: None,
            wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            gap: None,
        }
    }

    pub fn item_with(content: &'a dyn View<Output = Node<PMsg>>) -> Item<'a, PMsg> {
        Item::new(content)
    }

    pub fn add(mut self, el: &'a dyn View<Output = Node<PMsg>>) -> Self {
        self.items.push(Item::new(el));
        self
    }

    pub fn try_add(self, item: Option<&'a dyn View<Output = Node<PMsg>>>) -> Self {
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
        content: &'a dyn View<Output = Node<PMsg>>,
        config_item: impl FnOnce(Item<'a, PMsg>) -> Item<'a, PMsg> + 'static,
    ) -> Self {
        self.items.push(config_item(Item::new(content)));
        self
    }

    pub fn try_add_item_and(
        self,
        node: Option<&'a dyn View<Output = Node<PMsg>>>,
        config_item: impl FnOnce(Item<'a, PMsg>) -> Item<'a, PMsg> + 'static,
    ) -> Self {
        if let Some(node) = node {
            self.add_item_and(node, config_item)
        } else {
            self
        }
    }

    pub fn add_items(mut self, items: Vec<&'a dyn View<Output = Node<PMsg>>>) -> Self {
        let items = items
            .into_iter()
            .map(|item| Item::new(item))
            .collect::<Vec<Item<'a, PMsg>>>();
        self.items.extend(items);
        self
    }

    pub fn normal(self) -> Self {
        self.justify_content(val::Normal)
            .align_content(val::Normal)
            .align_items(val::Normal)
    }

    pub fn stretch(self) -> Self {
        self.justify_content(val::Stretch)
            .align_content(val::Stretch)
            .align_items(val::Stretch)
    }

    pub fn center(self) -> Self {
        self.justify_content(val::Center)
            .align_content(val::Center)
            .align_items(val::Center)
    }

    pub fn start(self) -> Self {
        self.justify_content(val::Start)
            .align_content(val::Start)
            .align_items(val::Start)
    }

    pub fn end(self) -> Self {
        self.justify_content(val::End)
            .align_content(val::End)
            .align_items(val::End)
    }

    pub fn space_between(self) -> Self {
        self.justify_content(val::SpaceBetween)
            .align_content(val::SpaceBetween)
    }

    pub fn space_around(self) -> Self {
        self.justify_content(val::SpaceAround)
            .align_content(val::SpaceAround)
    }

    pub fn space_evenly(self) -> Self {
        self.justify_content(val::SpaceEvenly)
            .align_content(val::SpaceEvenly)
    }
}

impl<'a, PMsg> View for Flexbox<'a, PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.flexbox()(&self.theme_lens())),
        )
    }
}

impl<'a, PMsg> StyledView for Flexbox<'a, PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        html::div()
            .set(att::class("flexbox"))
            .set(&self.events.flexbox)
            .set(&style.flexbox)
            .add(
                self.items
                    .iter()
                    // FIXME: find a way to generate items styles and call
                    // styled_view() (if that make sense :D)
                    .map(|item| item.view())
                    .collect::<Vec<Node<PMsg>>>(),
            )
    }
}

pub type Styler<'a, PMsg> = theme::Styler<Flexbox<'a, PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<FlexboxLens<'a>, Style>;
