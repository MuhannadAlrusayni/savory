use crate::{
    css::{self, unit::px, values as val, St},
    events::Events,
    macros::*,
    render::Render,
    theme::Theme,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::{borrow::Cow, rc::Rc};

// TODO: add placement property
#[derive(Clone, Rich)]
pub struct Popover<'a, PMsg, C, T> {
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    #[rich(write(take))]
    child: &'a C,
    #[rich(write(take))]
    target: &'a T,
    #[rich(write(take, style = compose))]
    pub style: Style,
    #[rich(write(take), read(copy, rename = is_visible), value_fns(take) = { popup = true, popdown = false })]
    pub visible: bool,
    #[rich(write(take))]
    pub offset: i8,
}

impl<'a, PMsg, C, T> Popover<'a, PMsg, C, T> {
    pub fn new(target: &'a T, child: &'a C) -> Self {
        Self {
            child,
            target,
            style: Style::default(),
            events: Events::default(),
            visible: false,
            offset: 0,
        }
    }
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(take, style = compose))]
    pub container: css::Style,
    #[rich(write(take, style = compose))]
    pub panel: css::Style,
}

impl<'a, PMsg: 'static, C, T> Render<PMsg> for Popover<'a, PMsg, C, T>
where
    PMsg: 'static,
    C: Render<PMsg, View = Node<PMsg>>,
    T: Render<PMsg, View = Node<PMsg>>,
{
    type View = Node<PMsg>;
    type Style = Style;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let style = theme.popover(self);
        div![
            style.container,
            self.events.events.clone(),
            self.target.render(theme),
            div![style.panel, self.child.render(theme)]
        ]
    }
}
