// extern crate khalas;

use khalas::{css::unit::px, prelude::*, renders, theme::ant::Ant};

pub enum Msg {
    Button(button::Msg),
    Checkbox(checkbox::Msg),
    Radio(radio::Msg),
    Switch(switch::Msg),
    Entry(entry::Msg),
    SpinEntry(spin_entry::Msg),
    Dialog(dialog::Msg),
    DialogChild(button::Msg),
    ProgressBar(progress_bar::Msg),
    MenuButton(menu_button::Msg),
}

pub struct Page {
    theme: Ant,
    button: Button<Msg>,
    checkbox: Checkbox<Msg>,
    radio: Radio<Msg>,
    switch: Switch<Msg>,
    entry: Entry<Msg>,
    spin_entry: SpinEntry<Msg>,
    dialog: Dialog<Msg, Button<Msg>>,
    // progress_bar: ProgressBar<Msg>,
    menu_button: MenuButton<Msg, ProgressBar<Msg>>,
}

impl Default for Page {
    fn default() -> Self {
        let dialog = Dialog::new(Msg::Dialog, Button::with_label(Msg::DialogChild, "hmm"))
            .open()
            .and_header_bar(|conf| {
                conf.set_title("Title for the widget")
                    .set_subtitle("subtitle for more description")
            });

        let button = Button::with_label(Msg::Button, "Click Here").and_events(|conf| {
            conf.click(|_| Msg::Dialog(dialog::Msg::Show))
                .click(|_| Msg::ProgressBar(progress_bar::Msg::Increment(2.)))
        });
        let progrese_bar = ProgressBar::new(Msg::ProgressBar)
            .failure()
            .set_min(10.)
            .set_max(25.)
            .set_value(13.);

        Self {
            theme: Ant::new(),
            button,
            checkbox: Checkbox::with_label(Msg::Checkbox, "Checkbox element"),
            radio: Radio::with_label(Msg::Radio, "Radio element"),
            switch: Switch::new(Msg::Switch),
            entry: Entry::with_placeholder(Msg::Entry, "Ali Yousef"),
            spin_entry: SpinEntry::new(Msg::SpinEntry),
            dialog,
            menu_button: MenuButton::new(Msg::MenuButton, "Progress Info", progrese_bar),
        }
    }
}

impl Model<Msg> for Page {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Button(msg) => self.button.update(msg, orders),
            Msg::Checkbox(msg) => self.checkbox.update(msg, orders),
            Msg::Radio(msg) => self.radio.update(msg, orders),
            Msg::Switch(msg) => self.switch.update(msg, orders),
            Msg::Entry(msg) => self.entry.update(msg, orders),
            Msg::SpinEntry(msg) => self.spin_entry.update(msg, orders),
            Msg::Dialog(msg) => self.dialog.update(msg, orders),
            Msg::DialogChild(msg) => self.dialog.child.update(msg, orders),
            Msg::ProgressBar(msg) => self.menu_button.child.update(msg, orders),
            Msg::MenuButton(msg) => self.menu_button.update(msg, orders),
        }
    }
}

impl Render<Msg> for Page {
    type View = Node<Msg>;
    type Style = ();

    fn style(&self, _: &Theme) -> Style {
        ()
    }

    fn render_with_style(&self, theme: &Theme, _: Style) -> Self::View {
        Flexbox::new()
            .center()
            .full_size()
            .column()
            .set_gap(px(4))
            .add_items(renders! {
                theme,
                self.button,
                self.menu_button,
                self.checkbox,
                self.radio,
                self.switch,
                self.entry,
                self.spin_entry,
                self.dialog,
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
