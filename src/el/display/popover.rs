use crate::{css, prelude::*};
use derive_rich::Rich;

// TODO: add placement property
#[derive(Clone, Rich)]
pub struct Popover<'a, PMsg, C, T> {
    #[rich(write(style = compose))]
    events: Events<PMsg>,
    #[rich(write)]
    child: &'a C,
    #[rich(write)]
    target: &'a T,
    #[rich(write(style = compose))]
    pub style: Style,
    #[rich(write, read(copy, rename = is_visible), value_fns = { popup = true, popdown = false })]
    pub visible: bool,
    #[rich(write)]
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
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
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

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.popover(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        div![
            style.container,
            self.events.events.clone(),
            self.target.render(theme),
            div![style.panel, self.child.render(theme)]
        ]
    }
}
