use savory::prelude::*;
use savory_elements::prelude::*;
use savory_style::{prelude::*, unit::px, values as val, Color, St};

#[derive(Element)]
pub struct Counter {
    #[element(config(default = "10"))]
    value: i32,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Element for Counter {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, _: &mut impl Orders<Msg>) -> Self {
        Self {
            value: config.value,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => self.value += 1,
            Msg::Decrement => self.value -= 1,
        }
    }
}

impl View<Node<Msg>> for Counter {
    fn view(&self) -> Node<Msg> {
        // sharde style for buttons
        let style_btns = |conf: Style| {
            conf.push(St::Appearance, val::None)
                .background(Color::SlateBlue)
                .text(Color::White)
                .and_border(|conf| conf.none().radius(px(4)))
                .margin(px(4))
                .padding(px(4))
        };

        // increment button node
        let inc_btn = html::button()
            .class("inc-btn")
            .and_style(style_btns)
            .on_click(|_| Msg::Increment)
            .push("Increment");

        // decrement button node
        let dec_btn = html::button()
            .class("dec-btn")
            .and_style(style_btns)
            .on_click(|_| Msg::Decrement)
            .push("Decrement");

        // contianer node
        html::div()
            .push(dec_btn)
            .push(self.value.to_string())
            .push(inc_btn)
    }
}

// App Element ---

pub enum AppMsg {
    Counter(Msg),
}

pub struct MyApp {
    counter_element: Counter,
}

impl Element for MyApp {
    type Message = AppMsg;
    type Config = Url;

    fn init(_: Url, orders: &mut impl Orders<AppMsg>) -> Self {
        Self {
            counter_element: Counter::config()
                // give it starting value. 10 will be used as default value if
                // we didn't pass value
                .value(100)
                .init(&mut orders.proxy(AppMsg::Counter)),
        }
    }

    fn update(&mut self, msg: AppMsg, orders: &mut impl Orders<AppMsg>) {
        match msg {
            AppMsg::Counter(msg) => self
                .counter_element
                .update(msg, &mut orders.proxy(AppMsg::Counter)),
        }
    }
}

impl View<Node<AppMsg>> for MyApp {
    fn view(&self) -> Node<AppMsg> {
        self.counter_element.view().map_msg(AppMsg::Counter)
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    // mount and start the app at `app` element
    MyApp::start();
}
