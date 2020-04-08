use crate::prelude::*;

use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Clone, Element)]
pub struct Label<PMsg> {
    // general element properties
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    style: Option<Style>,
    // label element properties
    #[rich(read, write)]
    #[element(theme_lens)]
    text: Cow<'static, str>,
}

crate::style_type! {
    label,
}

crate::events_type! {
    label,
}

impl<PMsg> View for Label<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        todo!();
        // span!()
        //     .set(att::class("label"))
        //     .add_children(vec![plain![self.text.to_string()]])
        //     .set_style(style["label"])
        //     .try_set_events(self.events.get("label"))
    }
}

impl<PMsg> From<Cow<'static, str>> for Label<PMsg> {
    fn from(source: Cow<'static, str>) -> Self {
        Self::new(source)
    }
}

impl<PMsg> From<String> for Label<PMsg> {
    fn from(source: String) -> Self {
        Self::new(source)
    }
}

impl<PMsg> From<&'static str> for Label<PMsg> {
    fn from(source: &'static str) -> Self {
        Self::new(source)
    }
}

impl<PMsg> Label<PMsg> {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            events: Events::default(),
            style: None,
            text: text.into(),
        }
    }
}
