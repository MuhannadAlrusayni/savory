use crate::id::Id;
use derive_rich::Rich;
use savory::prelude::*;
use savory_style::{box_align::*, flexbox::*, prelude::*, values as val};

#[derive(Rich)]
pub struct Item<Msg> {
    #[rich(write)]
    pub id: Option<Id>,

    #[rich(write(style = compose))]
    pub content: Node<Msg>,
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
    #[rich(read(copy, rename = is_flatten), value_fns = { flatten = true, wrapped = false })]
    pub flatten: bool,
}

impl<Msg> View<Node<Msg>> for Item<Msg> {
    fn view(&self) -> Node<Msg> {
        let styler = |s: Style| {
            s.try_flex_order(self.order)
                .try_flex_grow(self.grow)
                .try_flex_shrink(self.shrink)
                .try_flex_basis(self.basis.clone())
                .try_align_self(self.align_self)
        };

        if self.is_flatten() {
            self.content.clone().and_style(styler)
        } else {
            html::div()
                .try_id(self.id.clone())
                .class("flex-item")
                .and_style(styler)
                .push(self.content.clone())
        }
    }
}

impl<Msg> Item<Msg> {
    pub fn group(self, group_id: impl Into<Order>) -> Self {
        self.order(group_id)
    }
}

impl<Msg: 'static, OtherMsg: 'static> MessageMapper<Msg, OtherMsg> for Item<Msg> {
    type SelfWithOtherMs = Item<OtherMsg>;

    fn map_msg(self, f: impl FnOnce(Msg) -> OtherMsg + 'static + Clone) -> Self::SelfWithOtherMs {
        Item {
            content: self.content.map_msg(f),
            id: self.id,
            order: self.order,
            grow: self.grow,
            shrink: self.shrink,
            basis: self.basis,
            align_self: self.align_self,
            flatten: self.flatten,
        }
    }
}

impl<Msg> From<Node<Msg>> for Item<Msg> {
    fn from(node: Node<Msg>) -> Self {
        Self {
            id: None,
            content: node,
            order: None,
            grow: None,
            shrink: None,
            basis: None,
            align_self: None,
            flatten: false,
        }
    }
}

impl<'a, Msg> From<&'a dyn View<Node<Msg>>> for Item<Msg> {
    fn from(source: &'a dyn View<Node<Msg>>) -> Self {
        Self::from(source.view())
    }
}

impl<T, Msg> From<&T> for Item<Msg>
where
    T: View<Node<Msg>>,
{
    fn from(view: &T) -> Self {
        Self::from(view.view())
    }
}
