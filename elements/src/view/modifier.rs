use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich)]
pub struct Modifier<'a, Msg> {
    #[rich(write, write(style = compose))]
    pub padding: Option<css::Padding>,
    #[rich(write, write(style = compose))]
    pub margin: Option<css::Margin>,
    #[rich(write, write(style = compose))]
    pub size: Option<css::Size>,
    #[rich(write, write(style = compose))]
    pub font: Option<css::Font>,
    #[rich(write, write(style = compose))]
    pub border: Option<css::Border>,
    #[rich(write, write(style = compose))]
    pub position: Option<css::Position>,
    #[rich(write, write(option))]
    pub opacity: Option<css::Opacity>,
    pub target: &'a dyn View<Node<Msg>>,
}

impl<'a, Msg> Modifier<'a, Msg> {
    pub fn on(target: &'a dyn View<Node<Msg>>) -> Self {
        Self {
            padding: None,
            margin: None,
            size: None,
            font: None,
            border: None,
            opacity: None,
            position: None,
            target,
        }
    }
}

impl<'a, Msg> View<Node<Msg>> for Modifier<'a, Msg> {
    fn view(&self) -> Node<Msg> {
        let style = css::Style::default()
            .try_padding(self.padding)
            .try_margin(self.margin)
            .try_size(self.size)
            .try_font(self.font.clone())
            .try_border(self.border)
            .try_opacity(self.opacity)
            .try_position(self.position);

        self.target.view().add(style)
    }
}

pub trait NodeModifier<'a, Msg> {
    fn and_padding(&'a self, set_value: impl Fn(css::Padding) -> css::Padding)
        -> Modifier<'a, Msg>;
    fn and_margin(&'a self, set_value: impl Fn(css::Margin) -> css::Margin) -> Modifier<'a, Msg>;
    fn and_size(&'a self, set_value: impl Fn(css::Size) -> css::Size) -> Modifier<'a, Msg>;
    fn and_font(&'a self, set_value: impl Fn(css::Font) -> css::Font) -> Modifier<'a, Msg>;
    fn and_border(&'a self, set_value: impl Fn(css::Border) -> css::Border) -> Modifier<'a, Msg>;
    fn and_position(
        &'a self,
        set_value: impl Fn(css::Position) -> css::Position,
    ) -> Modifier<'a, Msg>;

    fn padding(&'a self, value: impl Into<css::Padding>) -> Modifier<'a, Msg>;
    fn margin(&'a self, value: impl Into<css::Margin>) -> Modifier<'a, Msg>;
    fn size(&'a self, value: impl Into<css::Size>) -> Modifier<'a, Msg>;
    fn font(&'a self, value: impl Into<css::Font>) -> Modifier<'a, Msg>;
    fn border(&'a self, value: impl Into<css::Border>) -> Modifier<'a, Msg>;
    fn position(&'a self, value: impl Into<css::Position>) -> Modifier<'a, Msg>;
    fn opacity(&'a self, value: impl Into<css::Opacity>) -> Modifier<'a, Msg>;
}

impl<'a, Msg, T: View<Node<Msg>>> NodeModifier<'a, Msg> for T {
    fn and_padding(
        &'a self,
        set_value: impl Fn(css::Padding) -> css::Padding,
    ) -> Modifier<'a, Msg> {
        Modifier::on(self).and_padding(set_value)
    }

    fn and_margin(&'a self, set_value: impl Fn(css::Margin) -> css::Margin) -> Modifier<'a, Msg> {
        Modifier::on(self).and_margin(set_value)
    }

    fn and_size(&'a self, set_value: impl Fn(css::Size) -> css::Size) -> Modifier<'a, Msg> {
        Modifier::on(self).and_size(set_value)
    }

    fn and_font(&'a self, set_value: impl Fn(css::Font) -> css::Font) -> Modifier<'a, Msg> {
        Modifier::on(self).and_font(set_value)
    }

    fn and_border(&'a self, set_value: impl Fn(css::Border) -> css::Border) -> Modifier<'a, Msg> {
        Modifier::on(self).and_border(set_value)
    }

    fn and_position(
        &'a self,
        set_value: impl Fn(css::Position) -> css::Position,
    ) -> Modifier<'a, Msg> {
        Modifier::on(self).and_position(set_value)
    }

    fn padding(&'a self, value: impl Into<css::Padding>) -> Modifier<'a, Msg> {
        Modifier::on(self).padding(value)
    }

    fn margin(&'a self, value: impl Into<css::Margin>) -> Modifier<'a, Msg> {
        Modifier::on(self).margin(value)
    }

    fn size(&'a self, value: impl Into<css::Size>) -> Modifier<'a, Msg> {
        Modifier::on(self).size(value)
    }

    fn font(&'a self, value: impl Into<css::Font>) -> Modifier<'a, Msg> {
        Modifier::on(self).font(value)
    }

    fn border(&'a self, value: impl Into<css::Border>) -> Modifier<'a, Msg> {
        Modifier::on(self).border(value)
    }

    fn position(&'a self, value: impl Into<css::Position>) -> Modifier<'a, Msg> {
        Modifier::on(self).position(value)
    }

    fn opacity(&'a self, value: impl Into<css::Opacity>) -> Modifier<'a, Msg> {
        Modifier::on(self).opacity(value)
    }
}
