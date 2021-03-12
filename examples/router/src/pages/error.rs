use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct Error {
    #[element(config(required))]
    error: String,
    env: Env,
}

impl Element for Error {
    type Config = Config;
    type Message = ();

    fn init(config: Self::Config, _orders: &mut impl Orders<Self::Message>, env: Env) -> Self {
        Error {
            error: config.error,
            env,
        }
    }

    fn update(&mut self, _: (), _: &mut impl Orders<Self::Message>) {
        todo!()
    }
}

impl View<Node<()>> for Error {
    fn view(&self) -> Node<()> {
        Flex::column()
            .push(&Text::new(self.error.clone(), self.env.branch()))
            .view()
    }
}
