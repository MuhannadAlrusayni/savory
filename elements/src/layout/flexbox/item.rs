use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::{
    css::{box_align::*, flexbox::*, values as val},
    prelude::*,
};
use std::default::Default;

#[derive(Rich, Element)]
#[element(style(item))]
pub struct Item<PMsg> {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<Self, Style>>,
    #[rich(write)]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write(style = compose))]
    pub content: Node<PMsg>,
    #[rich(write)]
    #[element(theme_lens)]
    pub order: Option<Order>,
    #[rich(write)]
    #[element(theme_lens)]
    pub grow: Option<Grow>,
    #[rich(write)]
    #[element(theme_lens)]
    pub shrink: Option<Shrink>,
    #[rich(write)]
    #[element(theme_lens)]
    pub basis: Option<Basis>,
    #[rich(value_fns = {
        auto = val::Auto,
        normal = val::Normal,
        stretch = val::Stretch,
        center = val::Center,
        start = val::Start,
        end = val::End,
    })]
    #[element(theme_lens)]
    pub align_self: Option<AlignSelf>,
    #[rich(read(copy, rename = is_flatten), value_fns = { flatten = true, wrapped = false })]
    #[element(theme_lens)]
    pub flatten: bool,
}

pub type ThemeStyler<'a> = Styler<ItemLens<'a>, Style>;

impl<PMsg> Stylable for Item<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.flexbox_item().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg> View<Node<PMsg>> for Item<PMsg> {
    fn view(&self) -> Node<PMsg> {
        self.styled_view(self.style())
    }
}

impl<PMsg> StyledView<Node<PMsg>> for Item<PMsg> {
    fn styled_view(&self, style: Style) -> Node<PMsg> {
        if self.is_flatten() {
            self.content.clone()
        } else {
            html::div()
                .try_id(self.id.clone())
                .class("flexbox-item")
                .add(self.content.clone())
        }
        .add(style.item)
    }
}

impl<PMsg> Item<PMsg> {
    pub fn group(self, group_id: impl Into<Order>) -> Self {
        self.order(group_id)
    }
}

impl<PMsg> From<Node<PMsg>> for Item<PMsg> {
    fn from(node: Node<PMsg>) -> Self {
        Self {
            id: None,
            styler: None,
            theme: Theme::default(),
            content: node,
            order: None,
            grow: None,
            shrink: None,
            basis: None,
            align_self: None,
            flatten: true,
        }
    }
}

impl<'a, PMsg> From<&'a dyn View<Node<PMsg>>> for Item<PMsg> {
    fn from(source: &'a dyn View<Node<PMsg>>) -> Self {
        Self::from(source.view())
    }
}

impl<T, PMsg> From<&T> for Item<PMsg>
where
    T: View<Node<PMsg>>,
{
    fn from(view: &T) -> Self {
        Self::from(view.view())
    }
}
