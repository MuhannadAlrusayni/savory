use khalas::{css::unit::px, prelude::*, theme::ant::Ant};
use seed::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[macro_use]
extern crate seed;

pub struct MyApp {
    inc_btn: Button<Msg>,
    dec_btn: Button<Msg>,
    pop_btn: Button<Msg>,
    count: i32,
    theme: Ant,
    popup: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            inc_btn: Button::with_label(Msg::IncBtn, "+").events(|e| e.click(|_| Msg::Increment)),
            dec_btn: Button::with_label(Msg::DecBtn, "-").events(|e| e.click(|_| Msg::Decrement)),
            pop_btn: Button::with_label(Msg::PopBtn, "Menu")
                .events(|conf| conf.click(Msg::TogglePopover)),
            count: 0,
            theme: Ant::default(),
            popup: false,
        }
    }
}

pub enum Msg {
    IncBtn(button::Msg),
    DecBtn(button::Msg),
    Increment,
    Decrement,
    PopBtn(button::Msg),
    TogglePopover(web_sys::MouseEvent),
}

impl Model<Msg, ()> for MyApp {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, ()>) {
        match msg {
            Msg::IncBtn(msg) => self.inc_btn.update(msg, orders),
            Msg::DecBtn(msg) => self.dec_btn.update(msg, orders),
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::PopBtn(msg) => self.pop_btn.update(msg, orders),
            Msg::TogglePopover(_) => self.popup = !self.popup,
        }
    }
}

impl Render<Msg> for MyApp {
    type View = Node<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let child = Flexbox::new()
            .gap(px(8.))
            .center()
            .full_size()
            .add(|item| item.content(vec![self.dec_btn.render(theme)]))
            .add(|item| item.content(vec![self.inc_btn.render(theme)]))
            .add(|item| item.content(vec![h3![self.count.to_string()]]))
            .render(theme);

        let popover = Popover::new(self.pop_btn.render(theme), child).visible(self.popup);

        Flexbox::new()
            .center()
            .full_size()
            .add(|item| item.content(vec![popover.render(theme)]))
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
