use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct User {
    #[element(config(required))]
    id: usize,
    env: Env,
}

impl Element for User {
    type Config = Config;
    type Message = ();

    fn init(config: Self::Config, _orders: &mut impl Orders<Self::Message>, env: Env) -> Self {
        User { id: config.id, env }
    }

    fn update(&mut self, _: (), _: &mut impl Orders<Self::Message>) {
        todo!()
    }
}

impl View<Node<()>> for User {
    fn view(&self) -> Node<()> {
        Flex::column()
            .push(&Text::new(
                format!("User {}, Welcome", self.id),
                self.env.branch(),
            ))
            .push(
                html::a()
                    .push(Text::new("Go to Home", self.env.branch()).view())
                    .href("/"),
            )
            .view()
    }
}
