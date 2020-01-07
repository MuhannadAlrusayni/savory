use crate::theme::Theme;
use seed::prelude::*;

pub trait Render<Msg: 'static> {
    type View: View<Msg>;
    type StyleMap;

    fn render(&self, theme: &impl Theme) -> Self::View;
}

impl<Msg: Clone + 'static> Render<Msg> for Node<Msg> {
    type View = Node<Msg>;
    type StyleMap = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<Msg: Clone + 'static> Render<Msg> for Vec<Node<Msg>> {
    type View = Vec<Node<Msg>>;
    type StyleMap = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<Msg: Clone + 'static> Render<Msg> for El<Msg> {
    type View = El<Msg>;
    type StyleMap = ();

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<Msg: Clone + 'static> Render<Msg> for Vec<El<Msg>> {
    type View = Vec<El<Msg>>;
    type StyleMap = ();

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
