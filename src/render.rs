use crate::theme::Theme;
use seed::prelude::*;

pub trait Render<Msg: 'static> {
    type View: View<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View;
}

impl<Msg: 'static> Render<Msg> for Node<Msg> {
    type View = Node<Msg>;

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<Msg: 'static> Render<Msg> for Vec<Node<Msg>> {
    type View = Vec<Node<Msg>>;

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<Msg: 'static> Render<Msg> for El<Msg> {
    type View = El<Msg>;

    fn render(&self, _: &impl Theme) -> Self::View {
        self.clone()
    }
}

impl<Msg: 'static> Render<Msg> for Vec<El<Msg>> {
    type View = Vec<El<Msg>>;

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
