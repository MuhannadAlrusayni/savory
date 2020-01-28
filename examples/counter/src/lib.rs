use khalas::{css::unit::px, prelude::*, theme::ant::Ant};
use seed::prelude::*;
use std::rc::Rc;

#[macro_use]
extern crate seed;

pub struct MyApp {
    inc_btn: Button<Msg>,
    dec_btn: Button<Msg>,
    count: i32,
    theme: Ant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            inc_btn: Button::with_label(Msg::IncBtn, "+").events(|e| e.click(|_| Msg::Increment)),
            dec_btn: Button::with_label(Msg::DecBtn, "-").events(|e| e.click(|_| Msg::Decrement)),
            count: 0,
            theme: Ant::default(),
        }
    }
}

pub enum Msg {
    IncBtn(button::Msg),
    DecBtn(button::Msg),
    Increment,
    Decrement,
    // Popup(i32, i32),
    // Popdown,
}

impl Model<Msg, Msg, ()> for MyApp {
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, ()>) {
        match msg {
            Msg::IncBtn(msg) => self.inc_btn.update(msg, orders),
            Msg::DecBtn(msg) => self.dec_btn.update(msg, orders),
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }
}

impl Render<Msg> for MyApp {
    type View = Node<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        Flexbox::new()
            .gap(px(8.))
            .center()
            .full_size()
            .add(|item| item.content(vec![self.dec_btn.render(theme)]))
            .add(|item| item.content(vec![self.inc_btn.render(theme)]))
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
