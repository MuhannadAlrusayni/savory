use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

// TODO: add placement property
#[derive(Clone, Rich, Element)]
pub struct Popover<'a, PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,
    #[rich(read, write)]
    child: &'a dyn Render<Output = Node<PMsg>>,
    #[rich(read, write)]
    target: &'a dyn Render<Output = Node<PMsg>>,
    #[rich(write, read(copy, rename = is_visible), value_fns = { popup = true, popdown = false })]
    #[element(theme_lens)]
    visible: bool,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    offset: i8,
}

crate::style_type! {
    panel,
    popover,
}

crate::events_type! {
    panel,
    popover,
}

impl<'a, PMsg> Popover<'a, PMsg> {
    pub fn new(
        target: &'a impl Render<Output = Node<PMsg>>,
        child: &'a impl Render<Output = Node<PMsg>>,
    ) -> Self {
        Self {
            events: Events::default(),
            style: None,
            child,
            target,
            visible: false,
            offset: 0,
        }
    }
}

impl<'a, PMsg> Render for Popover<'a, PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
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
