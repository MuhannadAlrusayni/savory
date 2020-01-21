use crate::theme::Theme;
use seed::prelude::*;

pub trait Render<Msg: 'static, ParentMsg: 'static> {
    type View: View<ParentMsg>;

    fn render(
        &self,
        theme: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View;
}

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for Node<Msg> {
    type View = Node<ParentMsg>;

    fn render(
        &self,
        _: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        self.clone().map_msg(map_msg)
    }
}

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for Vec<Node<Msg>> {
    type View = Vec<Node<ParentMsg>>;

    fn render(
        &self,
        _: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        self.clone()
            .into_iter()
            .map(|node| node.map_msg(map_msg.clone()))
            .collect()
    }
}

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for El<Msg> {
    type View = El<ParentMsg>;

    fn render(
        &self,
        _: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        self.clone().map_msg(map_msg)
    }
}

impl<Msg: 'static, ParentMsg: 'static> Render<Msg, ParentMsg> for Vec<El<Msg>> {
    type View = Vec<El<ParentMsg>>;

    fn render(
        &self,
        _: &impl Theme,
        map_msg: impl FnOnce(Msg) -> ParentMsg + 'static + Clone,
    ) -> Self::View {
        self.clone()
            .into_iter()
            .map(|el| el.map_msg(map_msg.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
