#[macro_use]
extern crate seed;

use khalas::{
    css::{self, unit::px},
    el::prelude::*,
    model::Model,
    render::Render,
    theme::{self, Theme},
};
use seed::{prelude::*, App};

#[derive(Clone, Debug)]
pub enum Msg {
    Switch(usize, switch::Msg),
    Btn(usize, button::Msg),
    Checkbox(usize, checkbox::Msg),
    Radio(usize, radio::Msg),
    Entry(usize, entry::Msg),
    Clicked,
}

type GlobalMsg = ();

pub struct MyApp {
    theme: theme::ant::Ant,
    buttons: Vec<Button>,
    switchs: Vec<Switch>,
    checkboxs: Vec<Checkbox>,
    radios: Vec<Radio>,
    entries: Vec<Entry>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

impl MyApp {
    pub fn new() -> Self {
        let url = "http://icons.iconarchive.com/icons/blackvariant/button-ui-requests-2/1024/BattleBears-icon.png";
        let buttons = vec![
            Button::with_label("Home")
                .route("home")
                .suggestion()
                .icon(Icon::url(url).size(|s| s.resize(px(18.), px(18.)))),
            Button::with_label("98 About Us")
                .route("about-us")
                .destructive(),
            Button::with_label("Add new btn").ghost(),
            Button::with_label("DockDuckGo").link(),
            Button::with_label("Add Firend +").dashed(),
            Button::with_label("Contact Us").route("contact-us"),
        ];
        let buttons = [
            buttons
                .clone()
                .into_iter()
                .map(|btn| btn.disable())
                .collect(),
            buttons,
        ]
        .concat();
        Self {
            theme: theme::ant::Ant::default(),
            buttons,
            switchs: vec![
                Switch::default(),
                Switch::default().toggle(),
                Switch::default().disable(),
                Switch::default().toggle().disable(),
            ],
            checkboxs: vec![
                Checkbox::default().label("Dark theme"),
                Checkbox::default().toggle().label("Use 5G"),
                Checkbox::default().label("Use pen").disable(),
                Checkbox::default().toggle().disable(),
            ],
            radios: vec![
                Radio::default().label("Selecte A"),
                Radio::default().label("Selte B").toggle(),
                Radio::default().label("Lectex V").disable(),
                Radio::default().label("Selecte A"),
                Radio::default().label("Selte B").toggle().disable(),
            ],
            entries: vec![
                Entry::default(),
                Entry::default().placeholder("e.g. email@example.org"),
                Entry::default().placeholder("e.g. email@example.org"),
                Entry::default().disable(),
                Entry::default()
                    .placeholder("e.g. email@example.org")
                    .disable(),
            ],
        }
    }
}

impl Model<Msg, GlobalMsg> for MyApp {
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, GlobalMsg>) {
        match msg {
            Msg::Entry(index, msg) => {
                if let Some(entry) = self.entries.get_mut(index) {
                    entry.update(msg, &mut orders.proxy(move |msg| Msg::Entry(index, msg)))
                }
            }
            Msg::Radio(index, msg) => {
                if let Some(radio) = self.radios.get_mut(index) {
                    radio.update(msg, &mut orders.proxy(move |msg| Msg::Radio(index, msg)))
                }
            }
            Msg::Checkbox(index, msg) => {
                if let Some(checkbox) = self.checkboxs.get_mut(index) {
                    checkbox.update(msg, &mut orders.proxy(move |msg| Msg::Checkbox(index, msg)))
                }
            }
            Msg::Switch(index, msg) => {
                if let Some(switch) = self.switchs.get_mut(index) {
                    switch.update(msg, &mut orders.proxy(move |msg| Msg::Switch(index, msg)))
                }
            }
            Msg::Btn(index, btn_msg) => {
                if let Some(btn) = self.buttons.get_mut(index) {
                    btn.update(btn_msg, &mut orders.proxy(move |msg| Msg::Btn(index, msg)));
                }
            }
            Msg::Clicked => {
                self.buttons.push(Button::with_label("Got cliked!"));
            }
        }
    }
}

impl Render<Msg> for MyApp {
    type View = Node<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let switchs = self
            .switchs
            .iter()
            .enumerate()
            .map(|(index, switch)| {
                switch
                    .render(theme)
                    .map_msg(move |msg| Msg::Switch(index, msg))
            })
            .collect::<Vec<Node<Msg>>>();
        let btns = self
            .buttons
            .iter()
            .enumerate()
            .map(|(index, btn)| btn.render(theme).map_msg(move |msg| Msg::Btn(index, msg)))
            .map(|mut btn| {
                btn.add_listener(simple_ev(Ev::Click, Msg::Clicked));
                btn
            })
            .collect::<Vec<Node<Msg>>>();
        let checkboxs = self
            .checkboxs
            .iter()
            .enumerate()
            .map(|(index, checkbox)| {
                checkbox
                    .render(theme)
                    .map_msg(move |msg| Msg::Checkbox(index, msg))
            })
            .collect::<Vec<Node<Msg>>>();
        let radios = self
            .radios
            .iter()
            .enumerate()
            .map(|(index, radio)| {
                radio
                    .render(theme)
                    .map_msg(move |msg| Msg::Radio(index, msg))
            })
            .collect::<Vec<Node<Msg>>>();
        let entries = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                entry
                    .render(theme)
                    .map_msg(move |msg| Msg::Entry(index, msg))
            })
            .collect::<Vec<Node<Msg>>>();

        [switchs, btns, checkboxs, radios, entries]
            .concat()
            .into_iter()
            .fold(Flexbox::new(), |flexbox, btn| flexbox.add(|item| item.content(vec![btn])))
            .add(|item| {
                let url = "http://icons.iconarchive.com/icons/blackvariant/button-ui-requests-2/1024/BattleBears-icon.png";
                item.content(vec![
                    Icon::url(url)
                        .size(|size| size.resize(px(248.), px(248.)))
                        .render(theme)
                ])
            })
            .margin(|m| m.all(|_| px(12.).into()))
            .gap(px(8.))
            .size(|size| size.full())
            .center()
            .wrap()
            .render(theme)
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(
        |msg, model: &mut MyApp, orders| {
            model.update(msg, orders);
            log!("hmmm");
        },
        |model| model.render(&model.theme),
    )
    .build_and_start();
}
