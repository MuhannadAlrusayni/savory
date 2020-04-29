use savory_core::prelude::*;
use savory_elements::prelude::*;
use savory_html::css::unit::px;
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
    ProgressBar(progress_bar::Msg),
    PopoverButton(button::Msg),
    Popover(popover::Msg),
}

pub struct MyApp {
    button: Button<Msg>,
    checkbox: Checkbox<Msg>,
    radio: Radio<Msg>,
    switch: Switch<Msg>,
    entry: Entry<Msg>,
    spin_entry: SpinEntry<Msg>,
    dialog: Dialog<Msg, Modifier<Button<Msg>>>,
    popover: Popover<Msg, Modifier<ProgressBar<Msg>>, Button<Msg>>,
}

impl HasConfig for MyApp {
    type Config = Url;
}

impl Element<Msg> for MyApp {
    type Message = Msg;

    fn init(_: Url, orders: &mut impl Orders<Msg>) -> Self {
        let dlg = Dialog::config(
            Msg::Dialog,
            Button::config(Msg::DialogChild)
                .label("hmm")
                .init(orders)
                .and_margin(|conf| conf.y(px(15))),
        )
        .title("Title for dialog")
        .subtitle("Some description here")
        .and_toggle(|conf| conf.opened())
        .init(orders);

        let button = Button::config(Msg::Button)
            .label("Click Here")
            .events(|| {
                button::events().and_button(|conf| {
                    conf.click(|_| Msg::Dialog(dialog::Msg::open()))
                        .click(|_| Msg::ProgressBar(progress_bar::Msg::increment(2.0)))
                })
            })
            .init(orders);

        let progress = ProgressBar::config(Msg::ProgressBar)
            .failure()
            .min(10.)
            .max(25.)
            .value(15.)
            .init(orders)
            .and_size(|conf| conf.min_width(px(40)))
            .and_margin(|conf| conf.all(px(4)));

        let pop_btn = Button::config(Msg::PopoverButton)
            .label("Popover button")
            .events(|| {
                button::events()
                    .and_button(|conf| conf.click(|_| Msg::Popover(popover::Msg::toggle())))
            })
            .init(orders);

        Self {
            button,
            checkbox: Checkbox::config(Msg::Checkbox)
                .label("Checkbox element")
                .init(orders),
            radio: Radio::config(Msg::Radio).label("Radio element").init(orders),
            switch: Switch::config(Msg::Switch).init(orders),
            entry: Entry::config(Msg::Entry)
                .placeholder("Ali Yousef")
                .init(orders),
            spin_entry: SpinEntry::config(Msg::SpinEntry).init(orders),
            dialog: dlg,
            popover: Popover::config(Msg::Popover, progress, pop_btn).init(orders),
        }
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
            Msg::ProgressBar(msg) => self.popover.child.update(msg, orders),
            Msg::Popover(msg) => self.popover.update(msg, orders),
            Msg::PopoverButton(msg) => self.popover.target.update(msg, orders),
        }
    }
}

impl View for MyApp {
    type Output = Node<Msg>;

    fn view(&self) -> Self::Output {
        Flexbox::new()
            .center()
            .column()
            .gap(px(4))
            .extend(vec![
                &self.button as &dyn View<Output = Node<Msg>>,
                &self.popover,
                &self.checkbox,
                &self.radio,
                &self.switch,
                &self.entry,
                &self.spin_entry,
                &self.dialog,
            ])
            .and_size(|conf| conf.full())
            .view()
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
