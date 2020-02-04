use crate::theme::Theme;
use seed::prelude::*;

pub trait Render<PMsg: 'static> {
    type View: View<PMsg>;
    type Style;

    fn render(&self, theme: &impl Theme) -> Self::View;
}

impl<PMsg: 'static> Render<PMsg> for Node<PMsg> {
    type View = Node<PMsg>;
    type Style = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for Vec<Node<PMsg>> {
    type View = Vec<Node<PMsg>>;
    type Style = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for El<PMsg> {
    type View = El<PMsg>;
    type Style = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<PMsg: 'static> Render<PMsg> for Vec<El<PMsg>> {
    type View = Vec<El<PMsg>>;
    type Style = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
