use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct Error {
    #[element(config(required))]
    error: String,
    #[element(config(required))]
    id: usize,
    env: Env,
}

impl Element for Error {
    type Config = Config;
    type Message = ();

    fn init(config: Self::Config, _orders: &mut impl Orders<Self::Message>, env: Env) -> Self {
        Error {
            error: config.error,
            id: config.id,
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
            .push(&Text::new(
                format!("User {} - {}", self.id, self.error),
                self.env.branch(),
            ))
            .view()
    }
}
