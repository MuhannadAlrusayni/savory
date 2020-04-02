use crate::prelude::*;
use derive_rich::Rich;

#[derive(Rich, Element)]
pub struct HeaderBar<PMsg> {
    // general element properties
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,

    // dialog element properties
    #[rich(read, write)]
    #[element(theme_lens(nested))]
    title: Option<Label<PMsg>>,
    #[rich(read, write)]
    #[element(theme_lens(nested))]
    subtitle: Option<Label<PMsg>>,
    #[rich(read, write)]
    #[element(theme_lens(nested))]
    pub close_button: Option<Button<PMsg>>,
    #[rich(read(rename = is_hidden), write)]
    #[element(theme_lens)]
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
            user_style: Style::default(),
            title: None,
            subtitle: None,
            close_button: None,
            hidden: false,
        }
    }
}

impl<PMsg> Render for HeaderBar<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.header_bar(self.theme_lens())
    }

    fn render_with_style(&self, theme: &Theme, style: Style) -> Self::View {
        todo!()
        // let title =
        //     Flexbox::new()
        //         .try_add(
        //             self.title.as_ref().map(|t| {
        //                 t.render_with_style(theme, style.sub_style(vec![("label", "title")]))
        //             }),
        //         )
        //         .try_add(self.subtitle.as_ref().map(|s| {
        //             s.render_with_style(theme, style.sub_style(vec![("label", "subtitle")]))
        //         }))
        //         .render_with_style(theme, style.sub_style(vec![("flexbox", "title-container")]));

        // Flexbox::new()
        //     .try_add(self.close_button.as_ref().map(|btn| {
        //         btn.render_with_style(
        //             theme,
        //             style.sub_style(vec![
        //                 ("button", "close-button"),
        //                 ("common-container", "close-button-container"),
        //             ]),
        //         )
        //     }))
        //     .add(title)
        //     .render_with_style(theme, style.sub_style(vec![("flexbox", "container")]))
    }
}
