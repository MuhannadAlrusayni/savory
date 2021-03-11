use savory::prelude::*;

// app element (the model)
pub struct Counter(i32);

// app message
pub enum Msg {
    Increment,
    Decrement,
}

impl Element for Counter {
    type Message = Msg;
    type Config = Url;

    // initialize the app in this function
    fn init(_: Url, _: &mut impl Orders<Msg>, _: &Env) -> Self {
        Self(0)
    }

    // handle app messages
    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => self.0 += 1,
            Msg::Decrement => self.0 -= 1,
        }
    }
}

impl View<Node<Msg>> for Counter {
    // view the app
    fn view(&self) -> Node<Msg> {
        let inc_btn = html::button()
            .push("Increment")
            .on_click(|_| Msg::Increment);
        let dec_btn = html::button()
            .push("Decrement")
            .on_click(|_| Msg::Decrement);

        html::div()
            .push(inc_btn)
            .push(self.0.to_string())
            .push(dec_btn)
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    // mount and start the app at `app` element
    Counter::start();
}
