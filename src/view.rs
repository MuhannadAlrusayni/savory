use crate::theme::Theme;
use seed::prelude::*;

pub trait View<Msg: 'static> {
    fn view(&self, theme: &impl Theme) -> Node<Msg>;
}

impl<Msg: Clone + 'static> View<Msg> for Node<Msg> {
    fn view(&self, _: &impl Theme) -> Node<Msg> {
        self.clone()
    }
}

impl<Msg: Clone + 'static> View<Msg> for El<Msg> {
    fn view(&self, _: &impl Theme) -> Node<Msg> {
        Node::Element(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
