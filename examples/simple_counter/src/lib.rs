use savory_core::prelude::*;
use savory_html::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Counter(i32);

pub enum Msg {
    Increment,
    Decrement,
}

impl AppElement for Counter {
    type Message = Msg;

    fn init(_: Url, _: &mut impl Orders<Msg>) -> Self {
        Self(0)
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => self.0 += 1,
            Msg::Decrement => self.0 -= 1,
        }
    }
}

impl View for Counter {
    type Output = Node<Msg>;

    fn view(&self) -> Self::Output {
        let inc_btn = html::button()
            .add("Increment")
            .and_events(|events| events.click(|_| Msg::Increment));

        let dec_btn = html::button()
            .add("Decrement")
            .and_events(|events| events.click(|_| Msg::Decrement));

        html::div()
            .add(inc_btn)
            .add(self.0.to_string())
            .add(dec_btn)
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    // this will start the app and mount it on node that have `root` id
    Counter::start();
}
