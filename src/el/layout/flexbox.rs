use crate::{
    macros::*,
    css::{
        self,
        flexbox::*,
        background::Background, border::Border, box_align::*, gap::Gap, margin::Margin,
        padding::Padding, size::Size, unit::*,
    },
    theme::Theme,
    view::View,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::default::Default;

#[derive(Clone, Rich, Default)]
pub struct Flexbox<ParentMsg: 'static> {
    pub items: Vec<Item<ParentMsg>>,
    // properties
    #[rich(value_fns(take) = {
        row = css::Row,
        reversed_row = css::RowReverse,
        column = css::Column,
        reversed_column = css::ColumnReverse,
    })]
    pub direction: Option<Direction>,
    #[rich(value_fns(take) = {
        wrap = css::Wrap,
        no_wrap = css::NoWrap,
        reversed_wrap = css::WrapReverse,
    })]
    pub wrap: Option<Wrap>,
    #[rich(write(take))]
    pub justify_content: Option<JustifyContent>,
    #[rich(write(take))]
    pub align_items: Option<AlignItems>,
    #[rich(write(take))]
    pub align_content: Option<AlignContent>,
    #[rich(write(take))]
    pub gap: Option<Gap>,
    #[rich(write(take, style = compose))]
    pub size: Size,
    #[rich(write(take, style = compose))]
    pub border: Border,
    #[rich(write(take, style = compose))]
    pub background: Background,
    #[rich(write(take, style = compose))]
    pub margin: Margin,
    #[rich(write(take, style = compose))]
    pub padding: Padding,
}

impl<ParentMsg: 'static> Flexbox<ParentMsg> {
    pub fn new() -> Self {
        Self {
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
        }
    }

    pub fn item(content: impl IntoIterator<Item = impl Into<Node<ParentMsg>>>) -> Item<ParentMsg> {
        Item::with_content(content)
    }

    pub fn add(mut self, get_child: impl FnOnce(Item<ParentMsg>) -> Item<ParentMsg>) -> Self {
        self.items.push(get_child(Item::new()));
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = Item<ParentMsg>>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn normal(self) -> Self {
        self.justify_content(css::Normal)
            .align_content(css::Normal)
            .align_items(css::Normal)
    }

    pub fn stretch(self) -> Self {
        self.justify_content(css::Stretch)
            .align_content(css::Stretch)
            .align_items(css::Stretch)
    }

    pub fn center(self) -> Self {
        self.justify_content(css::Center)
            .align_content(css::Center)
            .align_items(css::Center)
    }

    pub fn start(self) -> Self {
        self.justify_content(css::Start)
            .align_content(css::Start)
            .align_items(css::Start)
    }

    pub fn end(self) -> Self {
        self.justify_content(css::End)
            .align_content(css::End)
            .align_items(css::End)
    }

    pub fn space_between(self) -> Self {
        self.justify_content(css::SpaceBetween)
            .align_content(css::SpaceBetween)
    }

    pub fn space_around(self) -> Self {
        self.justify_content(css::SpaceAround)
            .align_content(css::SpaceAround)
    }

    pub fn space_evenly(self) -> Self {
        self.justify_content(css::SpaceEvenly)
            .align_content(css::SpaceEvenly)
    }
}

impl<ParentMsg: Clone + 'static> View<ParentMsg> for Flexbox<ParentMsg> {
    fn view(&self, theme: &impl Theme) -> Node<ParentMsg> {
        div![
            theme.flexbox(self),
            // items
            self.items.iter().map(|item| item.view(theme)),
        ]
    }
}

// ---- Flexbox Item ----

// TODO: add collapse propertie
#[derive(Clone, Rich, Default)]
pub struct Item<ParentMsg: 'static> {
    pub content: Vec<Node<ParentMsg>>,
    // propertie
    #[rich(write(take))]
    pub order: Option<i32>,
    #[rich(write(take))]
    pub grow: Option<f32>,
    #[rich(write(take))]
    pub shrink: Option<f32>,
    #[rich(write(take))]
    pub basis: Option<Basis>,
    #[rich(value_fns(take) = {
        auto = css::Auto,
        normal = css::Normal,
        stretch = css::Stretch,
        center = css::Center,
        start = css::Start,
        end = css::End,
    })]
    pub align_self: Option<AlignSelf>,
    #[rich(write(take, style = compose))]
    pub size: Size,
    #[rich(write(take, style = compose))]
    pub border: Border,
    #[rich(write(take, style = compose))]
    pub background: Background,
    #[rich(write(take, style = compose))]
    pub margin: Margin,
    #[rich(write(take, style = compose))]
    pub padding: Padding,
}

impl<ParentMsg: 'static> Item<ParentMsg> {
    pub fn new() -> Self {
        Self {
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
        }
    }

    pub fn with_content(content: impl IntoIterator<Item = impl Into<Node<ParentMsg>>>) -> Self {
        Self::new().content(content)
    }

    pub fn content(
        mut self,
        content: impl IntoIterator<Item = impl Into<Node<ParentMsg>>>,
    ) -> Self {
        self.content = content.into_iter().map(|c| c.into()).collect();
        self
    }

    pub fn auto_margin(self) -> Self {
        self.margin(|margin| margin.auto())
    }

    pub fn group(self, group_id: impl Into<i32>) -> Self {
        self.order(group_id)
    }
}

impl<ParentMsg: Clone + 'static> View<ParentMsg> for Item<ParentMsg> {
    fn view(&self, theme: &impl Theme) -> Node<ParentMsg> {
        div![
            theme.flexbox_item(self),
            // child
            self.content.clone()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum Msg {
        Home,
        AboutUs,
    }

    #[derive(Debug)]
    pub struct Flexbox {
        items: Vec<Node<Msg>>,
    }

    fn view_flexbox(flexbox: &Flexbox) -> Node<Msg> {
        div![flexbox.items.clone()]
    }

    #[test]
    fn test_listeners() {
        let items = vec![
            button![simple_ev(Ev::Click, Msg::Home), "Home"],
            button![simple_ev(Ev::Click, Msg::AboutUs), "About us"],
        ];

        let flexbox = Flexbox { items };
        //
        panic!("{:#?}", flexbox);
        let node = view_flexbox(&flexbox);
        panic!("{:#?}", node);
    }
}
