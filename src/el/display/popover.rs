use crate::{css, prelude::*};
use derive_rich::Rich;

// TODO: add placement property
#[derive(Clone, Rich)]
pub struct Popover<'a, PMsg, C, T> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,
    #[rich(read, write)]
    child: &'a C,
    #[rich(read, write)]
    target: &'a T,
    #[rich(write, read(copy, rename = is_visible), value_fns = { popup = true, popdown = false })]
    visible: bool,
    #[rich(read(copy), write)]
    offset: i8,
}

impl<'a, PMsg, C, T> Popover<'a, PMsg, C, T> {
    pub fn new(target: &'a T, child: &'a C) -> Self {
        Self {
            events: Events::default(),
            user_style: UserStyle::default(),
            child,
            target,
            visible: false,
            offset: 0,
        }
    }
}

#[derive(Clone, Debug, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
    pub panel: css::Style,
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
        let panel = div!()
            .set_style(style.panel)
            .add_children(vec![self.child.render(theme)]);

        div!()
            .set_style(style.container)
            .set_events(&self.events)
            .add_children(vec![self.target.render(theme), panel])
    }
}
