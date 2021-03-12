use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct Home {
    env: Env,
}

impl Element for Home {
    type Config = Config;
    type Message = ();

    fn init(_config: Self::Config, _orders: &mut impl Orders<Self::Message>, env: Env) -> Self {
        Home { env }
    }

    fn update(&mut self, _: (), _: &mut impl Orders<Self::Message>) {
        todo!()
    }
}

impl View<Node<()>> for Home {
    fn view(&self) -> Node<()> {
        Flex::column()
            .push(&Text::new("Home Page", self.env.branch()))
            .push(
                html::a()
                    .push(Text::new("Go to User 22", self.env.branch()).view())
                    .href("/user/22"),
            )
            .view()
    }
}
