use crate::{css, prelude::*};
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Rich)]
pub struct Label<PMsg> {
    // general element properties
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,
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
            user_style: UserStyle::default(),
            text: text.into(),
        }
    }
}

pub type UserStyle = css::Style;
pub type Style = css::Style;

impl<PMsg: 'static> Render<PMsg> for Label<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.label(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        span!()
            .add_children(vec![plain![self.text.to_string()]])
            .set_style(style)
            .set_events(&self.events)
    }
}
