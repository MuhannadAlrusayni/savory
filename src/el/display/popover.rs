use crate::{
    css::{self, unit::px, values as val, St, Style},
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
    pub child: Node<PMsg>,
    #[rich(write(take, style = compose))]
    pub style: Style,
    pub state: State,
}

#[derive(Clone)]
pub enum State {
    VisibleAt(i32, i32),
    Hidden,
}

impl<PMsg> Popover<PMsg> {
    pub fn new(child: Node<PMsg>) -> Self {
        Self {
            child,
            style: Style::default(),
            events: Events::default(),
            state: State::Hidden,
        }
    }

    pub fn popup_at(mut self, x: i32, y: i32) -> Self {
        self.state = State::VisibleAt(x, y);
        self
    }

    pub fn popdown(mut self) -> Self {
        self.state = State::Hidden;
        self
    }

    pub fn is_visible(&self) -> bool {
        !matches!(self.state, State::Hidden)
    }

    pub fn position(&self) -> Option<(i32, i32)> {
        if let State::VisibleAt(x, y) = self.state {
            Some((x, y))
        } else {
            None
        }
    }
}

impl<PMsg: 'static> Render<PMsg> for Popover<PMsg> {
    type View = Node<PMsg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        div![
            theme.popover(self),
            self.events.events.clone(),
            self.child.clone()
        ]
    }
}

impl<PMsg: 'static> Themeable for Popover<PMsg> {
    type StyleMap = css::Style;
}
