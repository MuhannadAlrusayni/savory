use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element, Clone)]
pub struct Url<PMsg> {
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,
    #[rich(read, write)]
    url: Cow<'static, str>,
}

crate::style_type! {
    url,
}

crate::events_type! {
    url,
}

impl<PMsg, T: ToString> From<T> for Url<PMsg> {
    fn from(url: T) -> Self {
        Self::new(url.to_string())
    }
}

impl<PMsg> Url<PMsg> {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            style: None,
            url: url.into(),
        }
    }
}

impl<PMsg> View for Url<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        todo!()
        // img!()
        //     .set(att::class("url-icon"))
        //     .set(att::src(self.url.clone()))
        //     .set(style["url-icon"])
        //     .try_set_events(self.events.get("url-icon"))
    }
}
