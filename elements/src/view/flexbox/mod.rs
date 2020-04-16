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
pub struct Flexbox<PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<PMsg>>,
    #[rich(write(rename = theme))]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write(style = compose), write(rename = items))]
    pub items: Vec<Item<PMsg>>,
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

impl<PMsg> Default for Flexbox<PMsg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<PMsg> Flexbox<PMsg> {
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

    pub fn item(item: impl Into<Item<PMsg>>) -> Item<PMsg> {
        item.into()
    }

    pub fn add(mut self, item: impl Into<Item<PMsg>>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn try_add(self, item: Option<impl Into<Item<PMsg>>>) -> Self {
        if let Some(item) = item {
            self.add(item)
        } else {
            self
        }
    }

    pub fn add_item(mut self, item: impl Into<Item<PMsg>>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn try_add_item(self, item: Option<impl Into<Item<PMsg>>>) -> Self {
        if let Some(item) = item {
            self.add_item(item)
        } else {
            self
        }
    }

    pub fn add_item_and(
        mut self,
        content: impl Into<Item<PMsg>>,
        config_item: impl FnOnce(Item<PMsg>) -> Item<PMsg> + 'static,
    ) -> Self {
        self.items.push(config_item(content.into()));
        self
    }

    pub fn try_add_item_and(
        self,
        node: Option<impl Into<Item<PMsg>>>,
        config_item: impl FnOnce(Item<PMsg>) -> Item<PMsg> + 'static,
    ) -> Self {
        if let Some(node) = node {
            self.add_item_and(node, config_item)
        } else {
            self
        }
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

impl<PMsg> View for Flexbox<PMsg> {
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

impl<PMsg> StyledView for Flexbox<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        html::div()
            .set(att::class("flexbox"))
            .set(&self.events.flexbox)
            .set(&style.flexbox)
            .add(
                self.items
                    .iter()
                    .map(|item| item.view())
                    .collect::<Vec<Node<PMsg>>>(),
            )
    }
}

impl<PMsg> ExtendBuilder<Item<PMsg>> for Flexbox<PMsg> {
    fn extend<T>(mut self, iter: T) -> Self
    where
        T: IntoIterator<Item = Item<PMsg>>,
    {
        self.items.extend(iter);
        self
    }
}

impl<'a, V, PMsg> ExtendBuilder<&'a V> for Flexbox<PMsg>
where
    V: View<Output = Node<PMsg>>,
{
    fn extend<T>(mut self, iter: T) -> Self
    where
        T: IntoIterator<Item = &'a V>,
    {
        self.items
            .extend(iter.into_iter().map(|content| Item::from(content)));
        self
    }
}

impl<'a, PMsg> ExtendBuilder<&'a dyn View<Output = Node<PMsg>>> for Flexbox<PMsg> {
    fn extend<T>(mut self, iter: T) -> Self
    where
        T: IntoIterator<Item = &'a dyn View<Output = Node<PMsg>>>,
    {
        self.items
            .extend(iter.into_iter().map(|content| Item::from(content)));
        self
    }
}

pub type Styler<PMsg> = theme::Styler<Flexbox<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<FlexboxLens<'a>, Style>;