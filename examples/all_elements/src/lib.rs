use savory::prelude::*;
use savory_elements::prelude::*;
// use savory_html::css::unit::px;
use wasm_bindgen::prelude::*;

pub enum Msg {
    Button(button::Msg),
    Checkbox(checkbox::Msg),
    Radio(radio::Msg),
    Switch(switch::Msg),
    Entry(entry::Msg),
    SpinEntry(spin_entry::Msg),
    Dialog(dialog::Msg),
    DialogChild(button::Msg),
    ProgressBar(progress_bar::Msg<Msg>),
    MenuButton(menu_button::Msg),
}

pub struct MyApp {
    theme: Theme,
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

impl AppElement for MyApp {
    type Message = Msg;

    fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
        todo!()
        // let dialog = Dialog::new(Msg::Dialog, Button::with_label(Msg::DialogChild, "hmm"))
        //     .open()
        //     .and_header_bar(|conf| {
        //         conf.set_title("Title for the widget")
        //             .set_subtitle("subtitle for more description")
        //     });

        // let button = Button::with_label(Msg::Button, "Click Here").and_events(|conf| {
        //     conf.and_button(|conf| {
        //         conf.click(|_| Msg::Dialog(dialog::Msg::Show))
        //             .click(|_| Msg::ProgressBar(progress_bar::Msg::Increment(2.)))
        //     })
        // });
        // let progrese_bar = ProgressBar::new(Msg::ProgressBar)
        //     .failure()
        //     .set_min(10.)
        //     .set_max(25.)
        //     .set_value(13.);

        // Self {
        //     theme: Theme::default(),
        //     button,
        //     checkbox: Checkbox::with_label(Msg::Checkbox, "Checkbox element"),
        //     radio: Radio::with_label(Msg::Radio, "Radio element"),
        //     switch: Switch::new(Msg::Switch),
        //     entry: Entry::with_placeholder(Msg::Entry, "Ali Yousef"),
        //     spin_entry: SpinEntry::new(Msg::SpinEntry),
        //     dialog,
        //     menu_button: MenuButton::new(Msg::MenuButton, "Progress Info", progrese_bar),
        // }
    }

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

impl Render for MyApp {
    type Output = Node<Msg>;

    fn render(&self) -> Self::Output {
        todo!()
        // Flexbox::new()
        //     .center()
        //     .full_size()
        //     .column()
        //     .set_gap(px(4))
        //     .add_items(renders! {
        //         theme,
        //         self.button,
        //         self.menu_button,
        //         self.checkbox,
        //         self.radio,
        //         self.switch,
        //         self.entry,
        //         self.spin_entry,
        //         self.dialog,
        //     })
        //     .render()
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    MyApp::start();
}
