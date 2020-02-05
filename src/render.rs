use crate::theme::Theme;
use seed::prelude::*;

/// This is the main trait used to render elments
pub trait Render<PMsg: 'static> {
    /// the returne type from `render` function
    type View: View<PMsg>;
    type Style;

    /// Return style for the current state of the element
    fn style(&self, theme: &impl Theme) -> Self::Style;

    /// This is the function used to render element and returne `Self::View`
    ///
    /// # Arguments
    /// `Theme` used to get the current style for the element
    fn render(&self, theme: &impl Theme) -> Self::View {
        self.render_with_style(theme, self.style(theme))
    }

    /// This is the main function used to render element with the passed style
    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View;
}

impl<PMsg: 'static> Render<PMsg> for Node<PMsg> {
    type View = Node<PMsg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for Vec<Node<PMsg>> {
    type View = Vec<Node<PMsg>>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for El<PMsg> {
    type View = El<PMsg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for Vec<El<PMsg>> {
    type View = Vec<El<PMsg>>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, _: &impl Theme, _: Self::Style) -> Self::View {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
