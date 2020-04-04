use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

#[derive(Clone, Element, Rich)]
pub struct Svg<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    view_box: Option<att::ViewBox>,
    #[rich(read, write)]
    draw: Vec<Node<PMsg>>,
}

crate::style_type! {
    svg,
}

crate::events_type! {
    svg,
}

impl<PMsg> Svg<PMsg> {
    pub fn new(draw: impl IntoIterator<Item = Node<PMsg>>) -> Self {
        Self {
            events: Events::default(),
            style: None,
            view_box: None,
            draw: draw.into_iter().collect(),
        }
    }
}

impl<PMsg> Render for Svg<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        todo!()
        // svg!()
        //     .and_attributes(|conf| conf.set_class("svg-icon").try_set_view_box(self.view_box))
        //     .set(style["svg-icon"])
        //     .try_set_events(self.events.get("svg-icon"))
        //     .add(self.draw.clone())
    }
}
