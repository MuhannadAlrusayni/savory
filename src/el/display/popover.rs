use crate::{
    css::{self, unit::px, values as val, St},
    events::Events,
    macros::*,
    render::Render,
    theme::{Theme, Themeable},
};
use derive_rich::Rich;
use seed::prelude::*;
use std::{borrow::Cow, rc::Rc};

#[derive(Clone, Rich)]
pub struct Popover<PMsg: 'static> {
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    #[rich(write(take))]
    child: Node<PMsg>,
    #[rich(write(take))]
    target: Node<PMsg>,
    #[rich(write(take, style = compose))]
    pub style: css::Style,
    #[rich(write(take), read(copy, rename = is_visible), value_fns(take) = { popup = true, popdown = false })]
    pub visible: bool,
}

impl<PMsg> Popover<PMsg> {
    pub fn new(target: Node<PMsg>, child: Node<PMsg>) -> Self {
        Self {
            child,
            target,
            style: css::Style::default(),
            events: Events::default(),
            visible: false,
        }
    }
}

impl<PMsg: 'static> Render<PMsg> for Popover<PMsg> {
    type View = Node<PMsg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let style = theme.popover(self);
        div![
            style.container,
            self.events.events.clone(),
            self.target.clone(),
            div![style.panel, self.child.clone(),]
        ]
    }
}

pub struct Style {
    pub container: css::Style,
    pub panel: css::Style,
}

impl<PMsg: 'static> Themeable for Popover<PMsg> {
    type StyleMap = Style;
}
