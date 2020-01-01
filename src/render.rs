use crate::theme::Theme;
use seed::prelude::*;

pub trait Render<Msg: 'static> {
    fn render(&self, theme: &impl Theme) -> Node<Msg>;
}

impl<Msg: Clone + 'static> Render<Msg> for Node<Msg> {
    fn render(&self, _: &impl Theme) -> Node<Msg> {
        self.clone()
    }
}

impl<Msg: Clone + 'static> Render<Msg> for El<Msg> {
    fn render(&self, _: &impl Theme) -> Node<Msg> {
        Node::Element(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
