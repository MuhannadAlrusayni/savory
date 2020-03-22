use khalas::{css::unit::px, prelude::*, theme::ant::Ant};

#[macro_use]
extern crate seed;

pub struct MyApp {
    menu_button: MenuButton<Msg, SpinEntry<Msg>>,
    theme: Ant,
}

impl Default for MyApp {
    fn default() -> Self {
        let spin_entry = SpinEntry::new(Msg::SpinEntry)
            .set_min(-40.)
            .set_placeholder(44.)
            .set_step(5.)
            .set_max(40.);

        let menu_button = MenuButton::new(Msg::MenuButton, "Menu", spin_entry);

        Self {
            theme: Ant::default(),
            menu_button,
        }
    }
}

pub enum Msg {
    SpinEntry(spin_entry::Msg),
    MenuButton(menu_button::Msg),
}

impl Model<Msg, ()> for MyApp {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, ()>) {
        match msg {
            Msg::SpinEntry(msg) => self.menu_button.child.update(msg, orders),
            Msg::MenuButton(msg) => self.menu_button.update(msg, orders),
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
        let menu_button = self.menu_button.render(theme);

        Flexbox::new()
            .center()
            .full_size()
            .add(nodes![menu_button])
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
