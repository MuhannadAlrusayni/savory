use savory_core::prelude::*;
use savory_elements::prelude::*;
use savory_html::{
    css::{unit::px, values as val, Color, St},
    prelude::*,
};
use wasm_bindgen::prelude::*;

#[derive(Element)]
#[element(style(inc_btn, dec_btn), events(inc_btn, dec_btn))]
pub struct Counter<PMsg> {
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    local_events: Events<Msg>,
    #[element(props(default = "10"))]
    value: i32,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl<PMsg: 'static> Element<PMsg> for Counter<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, _: &mut impl Orders<PMsg>) -> Self {
        let local_events = Events::default()
            // increment button events
            .and_inc_btn(|conf| conf.click(|_| Msg::Increment))
            // decrement button events
            .and_dec_btn(|conf| conf.click(|_| Msg::Decrement));

        Self {
            msg_mapper: props.msg_mapper,
            local_events,
            value: props.value,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg>) {
        match msg {
            Msg::Increment => self.value += 1,
            Msg::Decrement => self.value -= 1,
        }
    }
}

impl<PMsg: 'static> View for Counter<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
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
            .set(&self.local_events.inc_btn)
            .add("Increment");

        // decrement button node
        let dec_btn = html::button()
            .class("dec-btn")
            .set(style.dec_btn)
            .set(&self.local_events.dec_btn)
            .add("Decrement");

        // contianer node
        html::div()
            .add(dec_btn)
            .add(self.value.to_string())
            .add(inc_btn)
            // map the output node to the parent node
            .map_msg_with(&self.msg_mapper)
    }
}

// convenient way to convert Props into Counter
impl<PMsg: 'static> Props<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Counter<PMsg> {
        Counter::init(self, orders)
    }
}

// App Element ---

pub enum AppMsg {
    Counter(Msg),
}

pub struct MyApp {
    counter_element: Counter<AppMsg>,
}

impl Element<AppMsg> for MyApp {
    type Message = AppMsg;
    type Props = Url;

    fn init(_: Url, orders: &mut impl Orders<AppMsg>) -> Self {
        Self {
            counter_element: Counter::build(AppMsg::Counter)
                // give it starting value. 10 will be used as default value if
                // we didn't pass value
                .value(100)
                .init(orders),
        }
    }

    fn update(&mut self, msg: AppMsg, orders: &mut impl Orders<AppMsg>) {
        match msg {
            AppMsg::Counter(msg) => self.counter_element.update(msg, orders),
        }
    }
}

impl View for MyApp {
    type Output = Node<AppMsg>;

    fn view(&self) -> Self::Output {
        self.counter_element.view()
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    // mount and start the app at `app` element
    MyApp::start();
}
