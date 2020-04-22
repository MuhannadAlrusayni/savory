use savory_core::prelude::*;
use savory_elements::prelude::*;
use wasm_bindgen::prelude::*;

pub struct MyApp {
    spin_entry: SpinEntry<Msg>,
}

pub enum Msg {
    SpinEntry(spin_entry::Msg),
}

impl Element<Msg> for MyApp {
    type Message = Msg;
    type Props = Url;

    fn init(_: Url, orders: &mut impl Orders<Msg>) -> Self {
        let spin_entry = SpinEntry::build(Msg::SpinEntry)
            .min(-40.)
            .placeholder(44.)
            .step(5.)
            .max(40.)
            .init(orders);

        Self { spin_entry }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::SpinEntry(msg) => self.spin_entry.update(msg, orders),
        };
    }
}

impl View for MyApp {
    type Output = Node<Msg>;

    fn view(&self) -> Self::Output {
        Flexbox::new()
            .center()
            .add(&self.spin_entry)
            .and_size(|conf| conf.full())
            .view()
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
