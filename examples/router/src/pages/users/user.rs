use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct User {
    #[element(config(required))]
    id: usize,
}

impl Element for User {
    type Config = Config;
    type Message = ();

    fn init(config: Self::Config, _orders: &mut impl Orders<Self::Message>, _: &Env) -> Self {
        User { id: config.id }
    }

    fn update(&mut self, _: (), _: &mut impl Orders<Self::Message>) {
        todo!()
    }
}

impl View<Node<()>> for User {
    fn view(&self) -> Node<()> {
        let ds = DesignSystem::default();
        Flex::column()
            .push(&Text::new(format!("User {}, Welcome", self.id), ds.clone()))
            .push(
                html::a()
                    .push(Text::new("Go to Home", ds.clone()).view())
                    .href("/"),
            )
            .view()
    }
}
