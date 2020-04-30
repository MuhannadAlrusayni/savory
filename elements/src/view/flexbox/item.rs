use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::{
    css::{box_align::*, flexbox::*, values as val},
    prelude::*,
};
use std::default::Default;

#[derive(Rich, Element)]
#[element(style(item), events(item))]
pub struct Item<PMsg> {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<PMsg>>,
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

impl<PMsg> View for Item<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.flexbox_item().get(&self.theme_lens())),
        )
    }
}

impl<PMsg> StyledView for Item<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        if self.is_flatten() {
            self.content.clone()
        } else {
            html::div()
                .try_id(self.id.clone())
                .class("flexbox-item")
                .add(self.content.clone())
        }
        .add(&self.events.item)
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
            events: Events::default(),
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

impl<'a, PMsg> From<&'a dyn View<Output = Node<PMsg>>> for Item<PMsg> {
    fn from(source: &'a dyn View<Output = Node<PMsg>>) -> Self {
        Self::from(source.view())
    }
}

impl<T, PMsg> From<&T> for Item<PMsg>
where
    T: View<Output = Node<PMsg>>,
{
    fn from(view: &T) -> Self {
        Self::from(view.view())
    }
}

pub type Styler<PMsg> = theme::Styler<Item<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<ItemLens<'a>, Style>;
