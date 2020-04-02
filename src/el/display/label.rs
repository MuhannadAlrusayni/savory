use crate::prelude::*;
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Rich, Element)]
pub struct Label<PMsg> {
    // general element properties
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    // label element properties
    #[rich(read, write)]
    text: Cow<'static, str>,
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
            user_style: Style::default(),
            text: text.into(),
        }
    }
}

impl<PMsg> Render for Label<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.label(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // span!()
        //     .set(att::class("label"))
        //     .add_children(vec![plain![self.text.to_string()]])
        //     .set_style(style["label"])
        //     .try_set_events(self.events.get("label"))
    }
}
