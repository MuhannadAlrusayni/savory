#[macro_use]
extern crate seed;

use khalas::{
    css::{self, unit::px},
    el::{
        button::{self as btn, Button},
        icon::Icon,
        layout::flexbox::Flexbox,
    },
    model::Model,
    render::Render,
    theme::{self, Theme},
};
use seed::{prelude::*, App};

#[derive(Clone, Debug)]
pub enum Msg {
    Btn(usize, btn::Msg),
    Clicked,
}

type GlobalMsg = ();

pub struct MyApp {
    theme: theme::ant::Ant,
    buttons: Vec<Button>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

impl MyApp {
    pub fn new() -> Self {
        let url = "http://icons.iconarchive.com/icons/blackvariant/button-ui-requests-2/1024/BattleBears-icon.png";
        Self {
            theme: theme::ant::Ant::default(),
            buttons: vec![
                Button::with_label("Home")
                    .route("home")
                    .suggestion()
                    .icon(Icon::url(url).size(|s| s.resize(px(18.), px(18.)))),
                Button::with_label("98 About Us")
                    .route("about-us")
                    .destructive(),
                Button::with_label("Add new btn").ghost(),
                Button::with_label("DockDuckGo").link(),
                Button::with_label("Contact Us").route("contact-us"),
            ],
        }
    }
}

impl Model<Msg, GlobalMsg> for MyApp {
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, GlobalMsg>) {
        match msg {
            Msg::Btn(index, btn_msg) => {
                if let Some(btn) = self.buttons.get_mut(index) {
                    btn.update(
                        btn_msg,
                        &mut orders.proxy(move |child_msg| Msg::Btn(index, child_msg)),
                    );
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
        self.buttons
            .iter()
            .enumerate()
            .map(|(index, btn)| btn.render(theme).map_msg(move |msg| Msg::Btn(index, msg)))
            .map(|mut btn| {
                btn.add_listener(simple_ev(Ev::Click, Msg::Clicked));
                btn
            })
            .fold(Flexbox::new(), |flexbox, btn| flexbox.add(|item| item.content(vec![btn])))
            .add(|item| {
                let url = "http://icons.iconarchive.com/icons/blackvariant/button-ui-requests-2/1024/BattleBears-icon.png";
                item.content(vec![
                    Icon::url(url)
                        .size(|size| size.resize(px(248.), px(248.)))
                        .render(theme)
                ])
            })
            .gap(px(8.))
            .size(|size| size.full())
            .center()
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
