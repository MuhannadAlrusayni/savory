use crate::prelude::*;
use derive_rich::Rich;

#[derive(Rich)]
pub struct HeaderBar<PMsg: 'static> {
    // general element properties
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

    // dialog element properties
    #[rich(read, write)]
    title: Option<Label<PMsg>>,
    #[rich(read, write)]
    subtitle: Option<Label<PMsg>>,
    #[rich(read, write)]
    pub close_button: Option<Button<PMsg>>,
    #[rich(read(rename = is_hidden), write)]
    hidden: bool,
}

impl<PMsg> Default for HeaderBar<PMsg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<PMsg> HeaderBar<PMsg> {
    pub fn new() -> Self {
        Self {
            events: Events::default(),
            user_style: UserStyle::default(),
            title: None,
            subtitle: None,
            close_button: None,
            hidden: false,
        }
    }
}

#[derive(Clone, Debug, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub title_container: flexbox::Style,
    #[rich(write(style = compose))]
    pub container: flexbox::Style,
    #[rich(write(style = compose))]
    pub title: label::Style,
    #[rich(write(style = compose))]
    pub subtitle: label::Style,
    #[rich(write(style = compose))]
    pub close_button: button::Style,
}

#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub title_container: flexbox::Style,
    #[rich(write(style = compose))]
    pub container: flexbox::Style,
    #[rich(write(style = compose))]
    pub title: label::Style,
    #[rich(write(style = compose))]
    pub subtitle: label::Style,
    #[rich(write(style = compose))]
    pub close_button: button::Style,
}

impl<PMsg: 'static> Render<PMsg> for HeaderBar<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.header_bar(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        let Style {
            title_container,
            container,
            title,
            subtitle,
            close_button,
        } = style;
        let title_style = title;
        let subtitle_style = subtitle;
        let title = Flexbox::new()
            .try_add(
                self.title
                    .as_ref()
                    .map(|t| t.render_with_style(theme, title_style)),
            )
            .try_add(
                self.subtitle
                    .as_ref()
                    .map(|s| s.render_with_style(theme, subtitle_style)),
            )
            .render_with_style(theme, title_container);

        Flexbox::new()
            .try_add(
                self.close_button
                    .as_ref()
                    .map(|btn| btn.render_with_style(theme, close_button)),
            )
            .add(title)
            .render_with_style(theme, container)
    }
}
