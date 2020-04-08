use savory::prelude::*;
use savory_elements::prelude::*;
use wasm_bindgen::prelude::*;

pub struct MyApp {
    menu_button: MenuButton<Msg, SpinEntry<Msg>>,
    theme: Theme,
}

pub enum Msg {
    SpinEntry(spin_entry::Msg),
    MenuButton(menu_button::Msg),
}

impl AppElement for MyApp {
    type Message = Msg;

    fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
        let spin_entry = SpinEntry::new(Msg::SpinEntry)
            .set_min(-40.)
            .set_placeholder(44.)
            .set_step(5.)
            .set_max(40.);

        let menu_button = MenuButton::new(Msg::MenuButton, "Menu", spin_entry);

        Self {
            theme: Theme::default(),
            menu_button,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::SpinEntry(msg) => self.menu_button.child.update(msg, orders),
            Msg::MenuButton(msg) => self.menu_button.update(msg, orders),
        };
    }
}

impl View for MyApp {
    type Output = Node<Msg>;

    fn view(&self) -> Self::Output {
        Flexbox::new()
            .center()
            .full_size()
            .add(&self.menu_button)
            .view()
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
