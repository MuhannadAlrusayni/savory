use khalas::{css::unit::px, prelude::*, theme::ant::Ant};
use seed::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[macro_use]
extern crate seed;

pub struct MyApp {
    spin_entry: SpinEntry<Msg>,
    pop_btn: Button<Msg>,
    theme: Ant,
    popup: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            spin_entry: SpinEntry::new(Msg::SpinEntry),
            pop_btn: Button::with_label(Msg::PopBtn, "Menu")
                .events(|conf| conf.click(|_| Msg::TogglePopover)),
            theme: Ant::default(),
            popup: false,
        }
    }
}

pub enum Msg {
    SpinEntry(spin_entry::Msg),
    PopBtn(button::Msg),
    TogglePopover,
}

impl Model<Msg, ()> for MyApp {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, ()>) {
        match msg {
            Msg::SpinEntry(msg) => self.spin_entry.update(msg, orders),
            Msg::PopBtn(msg) => self.pop_btn.update(msg, orders),
            Msg::TogglePopover => self.popup = !self.popup,
        }
    }
}

impl Render<Msg> for MyApp {
    type View = Node<Msg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, theme: &impl Theme, _: Self::Style) -> Self::View {
        let child = Flexbox::new()
            .gap(px(8.))
            .center()
            .full_size()
            .add(|_| self.spin_entry.render(theme));

        let popover = Popover::new(&self.pop_btn, &child)
            .visible(self.popup)
            .offset(4);

        Flexbox::new()
            .center()
            .full_size()
            .add(|item| item.content(nodes![popover.render(theme)]))
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
