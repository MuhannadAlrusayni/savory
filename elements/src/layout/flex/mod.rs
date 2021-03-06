pub mod item;

use crate::{id::Id, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_style::{box_align::*, flexbox::*, prelude::*, values as val, Gap};

use item::Item;

#[derive(Rich)]
pub struct Flex<Msg> {
    #[rich(write)]
    pub id: Option<Id>,

    #[rich(write(style = compose), write(rename = items))]
    pub items: Vec<Item<Msg>>,
    #[rich(value_fns = {
        reversed_row = val::RowReverse,
        reversed_column = val::ColumnReverse,
    })]
    pub direction: Option<Direction>,
    #[rich(value_fns = {
        wrap = val::Wrap,
        no_wrap = val::Nowrap,
        reversed_wrap = val::WrapReverse,
    })]
    pub wrap: Option<Wrap>,
    #[rich(write(rename = justify_content))]
    pub justify_content: Option<JustifyContent>,
    #[rich(write(rename = align_items))]
    pub align_items: Option<AlignItems>,
    #[rich(write(rename = align_content))]
    pub align_content: Option<AlignContent>,
    #[rich(write(rename = gap))]
    pub gap: Option<Gap>,
    #[rich(value_fns = { inline = true })]
    pub inline: bool,
}

impl<Msg> Flex<Msg> {
    pub fn column() -> Self {
        Self {
            id: None,
            items: vec![],
            direction: Some(val::Column.into()),
            wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            gap: None,
            inline: false,
        }
    }

    pub fn row() -> Self {
        Self {
            id: None,
            items: vec![],
            direction: Some(val::Row.into()),
            wrap: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            gap: None,
            inline: false,
        }
    }

    pub fn item(item: impl Into<Item<Msg>>) -> Item<Msg> {
        item.into()
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

impl<Msg> View<Node<Msg>> for Flex<Msg> {
    fn view(&self) -> Node<Msg> {
        html::div()
            .try_id(self.id.clone())
            .and_style(|s| {
                s.config_if_else(
                    self.inline,
                    |c| c.display(val::InlineFlex),
                    |c| c.display(val::Flex),
                )
                .try_flex_direction(self.direction)
                .try_flex_wrap(self.wrap)
                .try_justify_content(self.justify_content)
                .try_align_items(self.align_items)
                .try_align_content(self.align_content)
                .try_gap(self.gap.clone())
            })
            .class("flex")
            .push(
                self.items
                    .iter()
                    .map(|item| item.view())
                    .collect::<Vec<Node<Msg>>>(),
            )
    }
}

impl<T, Msg> PushOwned<T> for Flex<Msg>
where
    T: Into<Item<Msg>>,
{
    fn push(mut self, val: T) -> Self {
        self.items.push(Self::item(val));
        self
    }
}

impl<U, Msg> ExtendBuilder<U> for Flex<Msg>
where
    U: Into<Item<Msg>>,
{
    fn extend<T>(mut self, iter: T) -> Self
    where
        T: IntoIterator<Item = U>,
    {
        self.items.extend(iter.into_iter().map(|i| i.into()));
        self
    }
}
