// extern crate khalas;

use khalas::{css::unit::px, prelude::*, renders, theme::ant::Ant};

pub enum Msg {
    Button(button::Msg),
    Checkbox(checkbox::Msg),
    Radio(radio::Msg),
    Switch(switch::Msg),
    Entry(entry::Msg),
    SpinEntry(spin_entry::Msg),
}

pub struct Page {
    theme: Ant,
    button: Button<Msg>,
    checkbox: Checkbox<Msg>,
    radio: Radio<Msg>,
    switch: Switch<Msg>,
    entry: Entry<Msg>,
    spin_entry: SpinEntry<Msg>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            theme: Ant::new(),
            button: Button::with_label(Msg::Button, "Click Here"),
            checkbox: Checkbox::with_label(Msg::Checkbox, "Checkbox element"),
            radio: Radio::with_label(Msg::Radio, "Radio element"),
            switch: Switch::new(Msg::Switch),
            entry: Entry::with_placeholder(Msg::Entry, "Ali Yousef"),
            spin_entry: SpinEntry::new(Msg::SpinEntry),
        }
    }
}

impl Model<Msg, ()> for Page {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, ()>) {
        match msg {
            Msg::Button(msg) => self.button.update(msg, orders),
            Msg::Checkbox(msg) => self.checkbox.update(msg, orders),
            Msg::Radio(msg) => self.radio.update(msg, orders),
            Msg::Switch(msg) => self.switch.update(msg, orders),
            Msg::Entry(msg) => self.entry.update(msg, orders),
            Msg::SpinEntry(msg) => self.spin_entry.update(msg, orders),
        }
    }
}

impl Render<Msg> for Page {
    type View = Node<Msg>;
    type Style = ();

    fn style(&self, _: &impl Theme) -> Self::Style {
        ()
    }

    fn render_with_style(&self, theme: &impl Theme, _: Self::Style) -> Self::View {
        Flexbox::new()
            .center()
            .full_size()
            .column()
            .set_gap(px(4))
            .add_items(renders! {
                theme,
                self.button,
                self.checkbox,
                self.radio,
                self.switch,
                self.entry,
                self.spin_entry,
            })
            .render(theme)
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(
        |msg, page: &mut Page, orders| {
            page.update(msg, orders);
        },
        |page| page.render(&page.theme),
    )
    .build_and_start();
}
