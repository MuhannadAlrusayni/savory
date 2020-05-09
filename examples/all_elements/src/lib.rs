use savory_core::prelude::*;
use savory_elements::prelude::*;
use savory_html::{css::unit::px, css::Color, prelude::*};
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
    button: Button,
    checkbox: Checkbox,
    radio: Radio,
    switch: Switch,
    entry: Entry,
    spin_entry: SpinEntry,
    dialog: Dialog,
    dialog_child: Button,
    popover: Popover,
    popover_btn: Button,
    progress_bar: ProgressBar,
}

impl Element for MyApp {
    type Message = Msg;
    type Config = Url;

    fn init(_: Url, orders: &mut impl Orders<Msg>) -> Self {
        let dlg = Dialog::config()
            .title("Title for dialog")
            .subtitle("Some description here")
            .and_toggle(|conf| conf.opened())
            .init(&mut orders.proxy(Msg::Dialog));

        let dlg_child = Button::config()
            .label("hmm")
            .init(&mut orders.proxy(Msg::DialogChild));

        let button = Button::config()
            .label("Click Here")
            .init(&mut orders.proxy(Msg::Button));

        let progress = ProgressBar::config()
            .failure()
            .min(10.)
            .max(25.)
            .value(15.)
            .init(&mut orders.proxy(Msg::ProgressBar));

        let pop_btn = Button::config()
            .label("Popover button")
            .init(&mut orders.proxy(Msg::PopoverButton));

        Self {
            button,
            checkbox: Checkbox::config()
                .label("Checkbox element")
                .init(&mut orders.proxy(Msg::Checkbox)),
            radio: Radio::config()
                .label("Radio element")
                .init(&mut orders.proxy(Msg::Radio)),
            switch: Switch::config().init(&mut orders.proxy(Msg::Switch)),
            entry: Entry::config()
                .placeholder("Ali Yousef")
                .init(&mut orders.proxy(Msg::Entry)),
            spin_entry: SpinEntry::config().init(&mut orders.proxy(Msg::SpinEntry)),
            dialog: dlg,
            dialog_child: dlg_child,
            popover: Popover::config().init(&mut orders.proxy(Msg::Popover)),
            popover_btn: pop_btn,
            progress_bar: progress,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Button(msg) => self.button.update(msg, &mut orders.proxy(Msg::Button)),
            Msg::Checkbox(msg) => self.checkbox.update(msg, &mut orders.proxy(Msg::Checkbox)),
            Msg::Radio(msg) => self.radio.update(msg, &mut orders.proxy(Msg::Radio)),
            Msg::Switch(msg) => self.switch.update(msg, &mut orders.proxy(Msg::Switch)),
            Msg::Entry(msg) => self.entry.update(msg, &mut orders.proxy(Msg::Entry)),
            Msg::SpinEntry(msg) => self
                .spin_entry
                .update(msg, &mut orders.proxy(Msg::SpinEntry)),
            Msg::Dialog(msg) => self.dialog.update(msg, &mut orders.proxy(Msg::Dialog)),
            Msg::DialogChild(msg) => self
                .dialog_child
                .update(msg, &mut orders.proxy(Msg::DialogChild)),
            Msg::ProgressBar(msg) => self
                .progress_bar
                .update(msg, &mut orders.proxy(Msg::ProgressBar)),
            Msg::Popover(msg) => self.popover.update(msg, &mut orders.proxy(Msg::Popover)),
            Msg::PopoverButton(msg) => self
                .popover_btn
                .update(msg, &mut orders.proxy(Msg::PopoverButton)),
        }
    }
}

impl View<Node<Msg>> for MyApp {
    fn view(&self) -> Node<Msg> {
        Flexbox::new()
            .center()
            .column()
            .gap(px(4))
            .add(self.button.view().map_msg(Msg::Button).and_events(|conf| {
                conf.click(|_| Msg::Dialog(dialog::Msg::open()))
                    .click(|_| Msg::ProgressBar(progress_bar::Msg::increment(2.0)))
            }))
            .add(
                self.popover
                    .view()
                    .map_msg(Msg::Popover)
                    .for_class("popover-target", |_| {
                        self.popover_btn
                            .view()
                            .map_msg(Msg::PopoverButton)
                            .and_events(|conf| conf.click(|_| Msg::Popover(popover::Msg::toggle())))
                    })
                    .for_class("popover-content", |_| {
                        self.progress_bar
                            .and_size(|conf| conf.min_width(px(40)))
                            .and_margin(|conf| conf.all(px(4)))
                            .view()
                            .map_msg(Msg::ProgressBar)
                    }),
            )
            .add(self.checkbox.view().map_msg(Msg::Checkbox))
            .add(self.radio.view().map_msg(Msg::Radio))
            .add(self.switch.view().map_msg(Msg::Switch))
            .add(self.entry.view().map_msg(Msg::Entry))
            .add(self.spin_entry.view().map_msg(Msg::SpinEntry))
            .add(
                self.dialog
                    .view()
                    .map_msg(Msg::Dialog)
                    .for_class("dialog-content", |_| {
                        self.dialog_child
                            .and_margin(|conf| conf.y(px(15)))
                            .view()
                            .map_msg(Msg::DialogChild)
                    }),
            )
            .and_size(|conf| conf.full())
            .view()
            .for_class("button", |node| {
                node.and_style(|conf| conf.and_border(|conf| conf.solid().color(Color::Plum)))
            })
            .for_id("4", |node| {
                node.and_style(|conf| conf.add("transform", "scale(2.8)"))
                    .and_events(|conf| conf.click(|_| Msg::Checkbox(checkbox::Msg::toggle())))
            })
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
