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
pub struct Item<'a, PMsg> {
    #[rich(write(style = compose))]
    pub events: Events<PMsg>,
    #[rich(write(style = compose))]
    pub styler: Option<Styler<'a, PMsg>>,
    #[rich(write(rename = theme))]
    #[element(theme_lens)]
    pub theme: Theme,

    #[rich(write(style = compose))]
    pub content: &'a dyn View<Output = Node<PMsg>>,
    #[rich(write(rename = order))]
    #[element(theme_lens)]
    pub order: Option<Order>,
    #[rich(write(rename = grow))]
    #[element(theme_lens)]
    pub grow: Option<Grow>,
    #[rich(write(rename = shrink))]
    #[element(theme_lens)]
    pub shrink: Option<Shrink>,
    #[rich(write(rename = basis))]
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

impl<'a, PMsg> View for Item<'a, PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.flexbox_item()(&self.theme_lens())),
        )
    }
}

impl<'a, PMsg> StyledView for Item<'a, PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        if self.is_flatten() {
            self.content.view()
        } else {
            html::div()
                .set(att::class("flexbox-item"))
                .add(self.content.view())
        }
        .add(&self.events.item)
        .add(&style.item)
    }
}

impl<'a, PMsg> Item<'a, PMsg> {
    pub fn new(content: &'a dyn View<Output = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            styler: None,
            theme: Theme::default(),
            content: content,
            order: None,
            grow: None,
            shrink: None,
            basis: None,
            align_self: None,
            flatten: true,
        }
    }

    pub fn group(self, group_id: impl Into<Order>) -> Self {
        self.order(group_id)
    }
}

pub type Styler<'a, PMsg> = theme::Styler<Item<'a, PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<ItemLens<'a>, Style>;
