use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich)]
pub struct Modifier<T> {
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
    pub target: T,
}

impl<T> Modifier<T> {
    pub fn on(target: T) -> Self {
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

impl<T: HasConfig> HasConfig for Modifier<T> {
    type Config = T::Config;
}

impl<T, PMsg> Element<PMsg> for Modifier<T>
where
    PMsg: 'static,
    T: Element<PMsg>,
{
    type Message = T::Message;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let target = T::init(config, orders);
        Self::on(target)
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg>) {
        self.target.update(msg, orders)
    }
}

impl<T, PMsg> View for Modifier<T>
where
    T: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
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

pub trait IntoModifier<T> {
    fn and_padding(self, set_value: impl Fn(css::Padding) -> css::Padding) -> Modifier<T>;
    fn and_margin(self, set_value: impl Fn(css::Margin) -> css::Margin) -> Modifier<T>;
    fn and_size(self, set_value: impl Fn(css::Size) -> css::Size) -> Modifier<T>;
    fn and_font(self, set_value: impl Fn(css::Font) -> css::Font) -> Modifier<T>;
    fn and_border(self, set_value: impl Fn(css::Border) -> css::Border) -> Modifier<T>;
    fn and_position(self, set_value: impl Fn(css::Position) -> css::Position) -> Modifier<T>;

    fn padding(self, value: impl Into<css::Padding>) -> Modifier<T>;
    fn margin(self, value: impl Into<css::Margin>) -> Modifier<T>;
    fn size(self, value: impl Into<css::Size>) -> Modifier<T>;
    fn font(self, value: impl Into<css::Font>) -> Modifier<T>;
    fn border(self, value: impl Into<css::Border>) -> Modifier<T>;
    fn position(self, value: impl Into<css::Position>) -> Modifier<T>;
    fn opacity(self, value: impl Into<css::Opacity>) -> Modifier<T>;
}

impl<T, PMsg> IntoModifier<T> for T
where
    T: View<Output = Node<PMsg>>,
{
    fn and_padding(self, set_value: impl Fn(css::Padding) -> css::Padding) -> Modifier<T> {
        Modifier::on(self).and_padding(set_value)
    }

    fn and_margin(self, set_value: impl Fn(css::Margin) -> css::Margin) -> Modifier<T> {
        Modifier::on(self).and_margin(set_value)
    }

    fn and_size(self, set_value: impl Fn(css::Size) -> css::Size) -> Modifier<T> {
        Modifier::on(self).and_size(set_value)
    }

    fn and_font(self, set_value: impl Fn(css::Font) -> css::Font) -> Modifier<T> {
        Modifier::on(self).and_font(set_value)
    }

    fn and_border(self, set_value: impl Fn(css::Border) -> css::Border) -> Modifier<T> {
        Modifier::on(self).and_border(set_value)
    }

    fn and_position(self, set_value: impl Fn(css::Position) -> css::Position) -> Modifier<T> {
        Modifier::on(self).and_position(set_value)
    }

    fn padding(self, value: impl Into<css::Padding>) -> Modifier<T> {
        Modifier::on(self).padding(value)
    }

    fn margin(self, value: impl Into<css::Margin>) -> Modifier<T> {
        Modifier::on(self).margin(value)
    }

    fn size(self, value: impl Into<css::Size>) -> Modifier<T> {
        Modifier::on(self).size(value)
    }

    fn font(self, value: impl Into<css::Font>) -> Modifier<T> {
        Modifier::on(self).font(value)
    }

    fn border(self, value: impl Into<css::Border>) -> Modifier<T> {
        Modifier::on(self).border(value)
    }

    fn position(self, value: impl Into<css::Position>) -> Modifier<T> {
        Modifier::on(self).position(value)
    }

    fn opacity(self, value: impl Into<css::Opacity>) -> Modifier<T> {
        Modifier::on(self).opacity(value)
    }
}
