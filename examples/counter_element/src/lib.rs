use savory_core::prelude::*;
use savory_elements::prelude::*;
use savory_html::{
    css::{unit::px, values as val, Color, St},
    prelude::*,
};
use wasm_bindgen::prelude::*;

#[derive(Element)]
#[element(style(inc_btn, dec_btn), events(inc_btn, dec_btn))]
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
        let style_btns = |conf: css::Style| {
            conf.add(St::Appearance, val::None)
                .background(Color::SlateBlue)
                .text(Color::White)
                .and_border(|conf| conf.none().radius(px(4)))
                .margin(px(4))
                .padding(px(4))
        };

        // create style
        let style = Style::default()
            .and_inc_btn(style_btns)
            .and_dec_btn(style_btns);

        // increment button node
        let inc_btn = html::button()
            .class("inc-btn")
            .set(style.inc_btn)
            .and_events(|conf| conf.click(|_| Msg::Increment))
            .add("Increment");

        // decrement button node
        let dec_btn = html::button()
            .class("dec-btn")
            .set(style.dec_btn)
            .and_events(|conf| conf.click(|_| Msg::Decrement))
            .add("Decrement");

        // contianer node
        html::div()
            .add(dec_btn)
            .add(self.value.to_string())
            .add(inc_btn)
    }
}

// convenient way to convert Config into Counter
impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Counter {
        Counter::init(self, orders)
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
