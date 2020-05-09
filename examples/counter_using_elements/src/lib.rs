use savory_core::prelude::*;
use savory_elements::prelude::*;
use wasm_bindgen::prelude::*;

pub struct MyApp {
    spin_entry: SpinEntry,
}

pub enum Msg {
    SpinEntry(spin_entry::Msg),
}

impl Element for MyApp {
    type Message = Msg;
    type Config = Url;

    fn init(_: Url, orders: &mut impl Orders<Msg>) -> Self {
        let spin_entry = SpinEntry::config()
            .min(-40.)
            .placeholder(44.)
            .step(5.)
            .max(40.)
            .init(&mut orders.proxy(Msg::SpinEntry));

        Self { spin_entry }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::SpinEntry(msg) => self
                .spin_entry
                .update(msg, &mut orders.proxy(Msg::SpinEntry)),
        };
    }
}

impl View<Node<Msg>> for MyApp {
    fn view(&self) -> Node<Msg> {
        Flexbox::new()
            .center()
            .add(self.spin_entry.view().map_msg(Msg::SpinEntry))
            .and_size(|conf| conf.full())
            .view()
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
