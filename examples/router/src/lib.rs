pub mod pages;

use savory::prelude::*;

pub struct App {
    pages: pages::Pages,
}

pub enum Msg {
    Pages(pages::Msg),
}

impl Element for App {
    type Message = Msg;
    type Config = Url;

    fn init(url: Self::Config, orders: &mut impl Orders<Self::Message>, env: Env) -> Self {
        let pages = pages::Config { url }.init(&mut orders.proxy(Msg::Pages), env);
        App { pages }
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<Self::Message>) {
        match msg {
            Msg::Pages(msg) => self.pages.update(msg, &mut orders.proxy(Msg::Pages)),
        }
    }
}

impl View<Node<Msg>> for App {
    fn view(&self) -> Node<Msg> {
        self.pages.view().map_msg(Msg::Pages)
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start();
}
