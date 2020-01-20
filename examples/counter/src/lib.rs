use khalas::{css::unit::px, el, prelude::*, theme::ant::Ant};
use seed::prelude::*;

#[macro_use]
extern crate seed;

pub struct MyApp {
    inc_btn: el::Button,
    dec_btn: el::Button,
    count: i32,
    theme: Ant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            inc_btn: el::Button::with_label("+"),
            dec_btn: el::Button::with_label("-"),
            count: 0,
            theme: Ant::default(),
        }
    }
}

pub enum Msg {
    IncBtn(el::button::Msg),
    DecBtn(el::button::Msg),
    Increment,
    Decrement,
}

impl Model<Msg, ()> for MyApp {
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, ()>) {
        match msg {
            Msg::IncBtn(msg) => self.inc_btn.update(msg, &mut orders.proxy(Msg::IncBtn)),
            Msg::DecBtn(msg) => self.dec_btn.update(msg, &mut orders.proxy(Msg::DecBtn)),
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }
}

impl Render<Msg> for MyApp {
    type View = Node<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        el::Flexbox::new()
            .gap(px(8.))
            .center()
            .full_size()
            .add(|item| {
                let mut btn = self.dec_btn.render(theme).map_msg(Msg::DecBtn);
                btn.add_listener(ev(Ev::Click, |_| Msg::Decrement));
                item.content(vec![btn])
            })
            .add(|item| {
                let mut btn = self.inc_btn.render(theme).map_msg(Msg::IncBtn);
                btn.add_listener(ev(Ev::Click, |_| Msg::Increment));
                item.content(vec![btn])
            })
            .add(|item| item.content(vec![h3![self.count.to_string()]]))
            .render(theme)
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(
        |msg, model: &mut MyApp, orders| {
            model.update(msg, orders);
        },
        |model| model.render(&model.theme),
    )
    .build_and_start();
}
