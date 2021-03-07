use savory::prelude::*;
use savory_elements::prelude::*;

#[derive(Element)]
pub struct Home;

impl Element for Home {
    type Config = Config;
    type Message = ();

    fn init(_config: Self::Config, _orders: &mut impl Orders<Self::Message>) -> Self {
        Home
    }

    fn update(&mut self, _: (), _: &mut impl Orders<Self::Message>) {
        todo!()
    }
}

impl View<Node<()>> for Home {
    fn view(&self) -> Node<()> {
        let ds = DesignSystem::default();
        Flex::column()
            .push(&Text::new("Home Page", ds.clone()))
            .push(
                html::a()
                    .push(Text::new("Go to User 22", ds.clone()).view())
                    .href("/user/22"),
            )
            .view()
    }
}
