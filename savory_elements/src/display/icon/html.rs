use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Clone, Element, Rich)]
pub struct Html<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    view_box: Option<att::ViewBox>,
    #[rich(read, write)]
    html: Cow<'static, str>,
}

crate::style_type! {
    html,
}

crate::events_type! {
    html,
}

impl<PMsg> Html<PMsg> {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            style: None,
            view_box: None,
            html: html.into(),
        }
    }
}

impl<PMsg> Render for Html<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        todo!()
        // svg!()
        //     .set(style["html-icon"])
        //     .and_attributes(|conf| conf.set_class("html-icon").try_set_view_box(self.view_box))
        //     .try_set_events(self.events.get("html-icon"))
        //     .add(raw![self.html.as_ref()])
    }
}
