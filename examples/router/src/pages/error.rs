use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct Error {
    #[element(config(required))]
    error: String,
}

impl Element for Error {
    type Config = Config;
    type Message = ();

    fn init(config: Self::Config, _orders: &mut impl Orders<Self::Message>) -> Self {
        Error {
            error: config.error,
        }
    }

    fn update(&mut self, _: (), _: &mut impl Orders<Self::Message>) {
        todo!()
    }
}

impl View<Node<()>> for Error {
    fn view(&self) -> Node<()> {
        let ds = DesignSystem::default();
        Flex::column()
            .push(&Text::new(self.error.clone(), ds.clone()))
            .view()
    }
}
