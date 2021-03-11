use savory::prelude::*;
use savory_elements::prelude::*;
use savory_style::prelude::*;

pub struct MyApp {
    ds: DesignSystem,
    value: usize,
    inc: Button,
    dec: Button,
}

pub enum Msg {
    Increment,
    Decrement,
    IncBtn(button::Msg),
    DecBtn(button::Msg),
}

impl Element for MyApp {
    type Message = Msg;
    type Config = Url;

    fn init(_: Url, orders: &mut impl Orders<Msg>, env: &Env) -> Self {
        Self {
            ds: DesignSystem::default(),
            value: 10,
            inc: Button::config()
                .text("Increment")
                .init(&mut orders.proxy(Msg::IncBtn), &env),
            dec: Button::config()
                .text("Increment")
                .init(&mut orders.proxy(Msg::DecBtn), &env),
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => self.value += 1,
            Msg::Decrement => self.value -= 1,
            Msg::IncBtn(msg) => self.inc.update(msg, &mut orders.proxy(Msg::IncBtn)),
            Msg::DecBtn(msg) => self.dec.update(msg, &mut orders.proxy(Msg::DecBtn)),
        };
    }
}

impl View<Node<Msg>> for MyApp {
    fn view(&self) -> Node<Msg> {
        Flex::row()
            .center()
            .push(
                self.inc
                    .view()
                    .map_msg(Msg::IncBtn)
                    .on_click(|_| Msg::Increment),
            )
            .push(&Text::new(self.value.to_string(), self.ds.clone()))
            .push(
                self.dec
                    .view()
                    .map_msg(Msg::IncBtn)
                    .on_click(|_| Msg::Decrement),
            )
            .view()
            .and_style(|style| style.and_size(|s| s.full()))
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
