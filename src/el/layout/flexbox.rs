use crate::{
    macros::*,
    properties::{
        background::Background, border::Border, box_align::*, gap::Gap, margin::Margin,
        padding::Padding, size::Size, unit::*,
    },
    theme::Theme,
    view::View,
};
use seed::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Display)]
pub enum Direction {
    #[display(fmt = "row")]
    Row,
    #[display(fmt = "row-reverse")]
    RowReverse,
    #[display(fmt = "column")]
    Column,
    #[display(fmt = "column-reverse")]
    ColumnReverse,
}

#[derive(Clone, Copy, PartialEq, Eq, Display)]
pub enum Wrap {
    #[display(fmt = "wrap")]
    Wrap,
    #[display(fmt = "nowrap")]
    NoWrap,
    #[display(fmt = "wrap-reverse")]
    WrapReverse,
}

#[derive(Clone)]
pub struct Flexbox<Msg: 'static + Clone> {
    items: Vec<Item<Msg>>,
    // properties
    direction: Option<Direction>,
    wrap: Option<Wrap>,
    justify_content: Option<JustifyContent>,
    align_items: Option<AlignItems>,
    align_content: Option<AlignContent>,
    gap: Option<Gap>,
    size: Size,
    border: Border,
    background: Background,
    margin: Margin,
    padding: Padding,
}

impl<Msg: 'static + Clone> Flexbox<Msg> {
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

    pub fn item(child: impl Into<Node<Msg>>) -> Item<Msg> {
        Item::new(child)
    }

    pub fn add(mut self, get_child: impl FnOnce(Item<Msg>) -> Item<Msg>) -> Self {
        self.items.push(get_child(Item::empty()));
        self
    }

    pub fn adds(mut self, children: impl IntoIterator<Item = Item<Msg>>) -> Self {
        self.items.extend(children);
        self
    }

    pub fn normal(self) -> Self {
        self.justify_content(JustifyContent::Normal)
            .align_content(AlignContent::Normal)
            .align_items(AlignItems::Normal)
    }

    pub fn stretch(self) -> Self {
        self.justify_content(JustifyContent::Stretch)
            .align_content(AlignContent::Stretch)
            .align_items(AlignItems::Stretch)
    }

    pub fn center(self) -> Self {
        self.justify_content(JustifyContent::Center)
            .align_content(AlignContent::Center)
            .align_items(AlignItems::Center)
    }

    pub fn start(self) -> Self {
        self.justify_content(JustifyContent::Start)
            .align_content(AlignContent::Start)
            .align_items(AlignItems::Start)
    }

    pub fn end(self) -> Self {
        self.justify_content(JustifyContent::End)
            .align_content(AlignContent::End)
            .align_items(AlignItems::End)
    }

    pub fn space_between(self) -> Self {
        self.justify_content(JustifyContent::SpaceBetween)
            .align_content(AlignContent::SpaceBetween)
    }

    pub fn space_around(self) -> Self {
        self.justify_content(JustifyContent::SpaceAround)
            .align_content(AlignContent::SpaceAround)
    }

    pub fn space_evenly(self) -> Self {
        self.justify_content(JustifyContent::SpaceEvenly)
            .align_content(AlignContent::SpaceEvenly)
    }

    builder_enum_functions! {
        direction {
            row() => Direction::Row,
            reversed_row() => Direction::RowReverse,
            column() => Direction::Column,
            reversed_column() => Direction::ColumnReverse,
        }
        wrap {
            wrap() => Wrap::Wrap,
            no_wrap() => Wrap::NoWrap,
            reversed_wrap() => Wrap::WrapReverse,
        }
    }

    builder_functions! {
        gap(Gap),
        align_content(AlignContent),
        justify_content(JustifyContent),
        align_items(AlignItems),
    }

    composition_functions! {
        size: Size,
        border: Border,
        background: Background,
        margin: Margin,
        padding: Padding,
    }
}

impl<Msg: 'static + Clone> View<Msg> for Flexbox<Msg> {
    fn view(&self, theme: &impl Theme) -> Node<Msg> {
        // flex container style
        let mut style = style![
            St::Display => "flex",
            St::FlexDirection => self.direction,
            St::FlexWrap => self.wrap,
            St::JustifyContent => self.justify_content,
            St::AlignItems => self.align_items,
            St::AlignContent => self.align_content,
            St::Gap => self.gap,
        ];
        style.merge((&self.size).into());
        style.merge((&self.border).into());
        style.merge((&self.background).into());
        style.merge((&self.margin).into());
        style.merge((&self.padding).into());

        div![
            style,
            // items
            // self.items.iter().map(|item| item.view(theme))
        ]
    }
}

// ---- Flexbox Item ----

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Basis {
    #[display(fmt = "content")]
    Content,
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    #[from]
    Em(Em),
    #[from]
    Ex(Ex),
    #[from]
    Cap(Cap),
    #[from]
    Ch(Ch),
    #[from]
    Ic(Ic),
    #[from]
    Rem(Rem),
    #[from]
    Rlh(Rlh),
    #[from]
    Vm(Vm),
    #[from]
    Vh(Vh),
    #[from]
    Vi(Vi),
    #[from]
    Vb(Vb),
    #[from]
    Vmin(Vmin),
    #[from]
    Vmax(Vmax),
    #[from]
    Cm(Cm),
    #[from]
    Mm(Mm),
    #[from]
    Q(Q),
    #[from]
    In(In),
    #[from]
    Pc(Pc),
    #[from]
    Pt(Pt),
    #[from]
    Px(Px),
    #[from(forward)]
    Percent(Percent),
}

// TODO: add collapse propertie
#[derive(Clone)]
pub struct Item<Msg: 'static + Clone> {
    child: Option<Node<Msg>>,
    // propertie
    order: Option<i32>,
    grow: Option<f32>,
    shrink: Option<f32>,
    basis: Option<Basis>,
    align_self: Option<AlignSelf>,
    size: Size,
    border: Border,
    background: Background,
    margin: Margin,
    padding: Padding,
}

impl<Msg: 'static + Clone> Item<Msg> {
    pub fn new(child: impl Into<Node<Msg>>) -> Self {
        Self::empty().child(child)
    }

    pub fn empty() -> Self {
        Self {
            child: None,
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

    pub fn auto_margin(self) -> Self {
        self.margin(|margin| margin.auto())
    }

    pub fn group(self, group_id: impl Into<i32>) -> Self {
        self.order(group_id)
    }

    builder_functions! {
        child(Node<Msg>),
        order(i32),
        grow(f32),
        shrink(f32),
        basis(Basis),
        align_self(AlignSelf),
    }

    builder_enum_functions! {
        align_self {
            auto() => AlignSelf::Auto,
            normal() => AlignSelf::Normal,
            stretch() => AlignSelf::Stretch,
            center() => AlignSelf::Center,
            start() => AlignSelf::Start,
            end() => AlignSelf::End,
        }
    }

    composition_functions! {
        size: Size,
        border: Border,
        background: Background,
        margin: Margin,
        padding: Padding,
    }
}

impl<Msg: 'static + Clone> View<Msg> for Item<Msg> {
    fn view(&self, _: &impl Theme) -> Node<Msg> {
        // flex item style
        let mut style = style![
            St::Order => self.order,
            St::FlexGrow => self.grow,
            St::FlexShrink => self.shrink,
            St::FlexBasis => self.basis,
            St::AlignSelf => self.align_self,
            // St::Visibility => self.collapse,
        ];
        style.merge((&self.size).into());
        style.merge((&self.border).into());
        style.merge((&self.background).into());
        style.merge((&self.margin).into());
        style.merge((&self.padding).into());

        div![
            style,
            // child
            self.child.as_ref().map_or(empty![], |item| item.clone()),
        ]
    }
}
