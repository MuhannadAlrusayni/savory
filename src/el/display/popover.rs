use crate::prelude::*;
use derive_rich::Rich;

// TODO: add placement property
#[derive(Clone, Rich, Element)]
pub struct Popover<'a, PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    #[rich(read, write)]
    child: &'a dyn Render<View = Node<PMsg>>,
    #[rich(read, write)]
    target: &'a dyn Render<View = Node<PMsg>>,
    #[rich(write, read(copy, rename = is_visible), value_fns = { popup = true, popdown = false })]
    #[element(theme_lens)]
    visible: bool,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    offset: i8,
}

impl<'a, PMsg> Popover<'a, PMsg> {
    pub fn new(
        target: &'a impl Render<View = Node<PMsg>>,
        child: &'a impl Render<View = Node<PMsg>>,
    ) -> Self {
        Self {
            events: Events::default(),
            user_style: Style::default(),
            child,
            target,
            visible: false,
            offset: 0,
        }
    }
}

impl<'a, PMsg> Render for Popover<'a, PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.popover(self.theme_lens())
    }

    fn render_with_style(&self, theme: &Theme, style: Style) -> Self::View {
        todo!()
        // let panel = div!()
        //     .set(att::class("panel"))
        //     .set(style["panel"])
        //     .try_set(self.events.get("panel"))
        //     .add(self.child.render(theme));

        // div!()
        //     .set(att::class("popover"))
        //     .set(style["popover"])
        //     .try_set(self.events.get("popover"))
        //     .add(vec![self.target.render(theme), panel])
    }
}
