# Savory
**Rust / Wasm frontend library for building user interfaces.**

[![master docs](https://img.shields.io/badge/docs-master-blue.svg)](https://malrusayni.gitlab.io/savory/savory_core/)
&middot;
[![crate info](https://img.shields.io/crates/v/savory.svg)](https://crates.io/crates/savory-core)
&middot;
[![pipeline](https://gitlab.com/MAlrusayni/savory/badges/master/pipeline.svg)](https://gitlab.com/MAlrusayni/savory/pipelines)
&middot;
[![rustc version](https://img.shields.io/badge/rustc-stable-green.svg)](https://crates.io/crates/savory)
&middot;
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Savory is library for building user interface based on [Seed]

[Documentation](https://malrusayni.gitlab.io/savory/savory_core/)

# Features

- **Views**: Views can be any type implement `View` trait or any standalone
  function that returns `Node`, views can be trait object which make them very
  composable.
- **Elements**: Savory uses elements as core building unit when building
  stateful UI. Elements owns thier state and handle user inputs via messages.
- **Collection of UI elements**: Savory ships with collection of resuable and
  themeable UI elements.
- **Theme**: UI elements can be themed by any type that implement `ThemeImpl`
  trait, themes have full control on the element appearance.
- **Typed HTML**: Use typed CSS and HTML attributes, Savory try hard not to rely
  on strings when creating CSS and HTML attributes since these can produce hard
  to debug bugs.
- **Enhance Seed API**: Enhancement on Seed API that makes working with `Node`,
  `Orders` fun.

Savory tries to make writing UI elements fun and boilerplate free.

# Screenshot

![Screenshot](docs-resources/screenshot.png)

# Examples

Here we will create the same counter app found in [Elm tutorial], then we will
write the same app but with styled and reusable element.

## Simple Counter

```rust
use savory_core::prelude::*;
use savory_html::prelude::*;
use wasm_bindgen::prelude::*;

// app element (the model)
pub struct Counter(i32);

// app message
pub enum Msg {
    Increment,
    Decrement,
}

impl Element for Counter {
    type Message = Msg;
    type Config = Url;

    // initialize the app in this function
    fn init(_: Url, _: &mut impl Orders<Msg>) -> Self {
        Self(0)
    }

    // handle app messages
    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => self.0 += 1,
            Msg::Decrement => self.0 -= 1,
        }
    }
}

impl View<Node<Msg>> for Counter {
    // view the app
    fn view(&self) -> Node<Msg> {
        let inc_btn = html::button().add("Increment").on_click(|_| Msg::Increment);
        let dec_btn = html::button().add("Decrement").on_click(|_| Msg::Decrement);

        html::div()
            .add(inc_btn)
            .add(self.0.to_string())
            .add(dec_btn)
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    // mount and start the app at `app` element
    Counter::start();
}
```

Preview:
![Screenshot](docs-resources/simple-counter.webm)

[source code](https://gitlab.com/MAlrusayni/savory/-/tree/master/examples/simple_counter)

## Counter As Element

Now we will make counter element and an app element this illustrate how to make
parent and child element, and how to make resuable and stylable element.

```rust
use savory_core::prelude::*;
use savory_elements::prelude::*;
use savory_html::{
    css::{unit::px, values as val, Color, St},
    prelude::*,
};
use wasm_bindgen::prelude::*;

#[derive(Element)]
#[element(style(inc_btn, dec_btn))]
pub struct Counter {
    #[element(config(default = "10"))]
    value: i32,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Element for Counter {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, _: &mut impl Orders<Msg>) -> Self {
        Self {
            value: config.value,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => self.value += 1,
            Msg::Decrement => self.value -= 1,
        }
    }
}

impl View<Node<Msg>> for Counter {
    fn view(&self) -> Node<Msg> {
        // sharde style for buttons
        let style_btns = |conf: css::Style| {
            conf.add(St::Appearance, val::None)
                .background(Color::SlateBlue)
                .text(Color::White)
                .and_border(|conf| conf.none().radius(px(4)))
                .margin(px(4))
                .padding(px(4))
        };

        // create style
        let style = Style::default()
            .and_inc_btn(style_btns)
            .and_dec_btn(style_btns);

        // increment button node
        let inc_btn = html::button()
            .class("inc-btn")
            .set(style.inc_btn)
            .on_click(|_| Msg::Increment)
            .add("Increment");

        // decrement button node
        let dec_btn = html::button()
            .class("dec-btn")
            .set(style.dec_btn)
            .on_click(|_| Msg::Decrement)
            .add("Decrement");

        // contianer node
        html::div()
            .add(dec_btn)
            .add(self.value.to_string())
            .add(inc_btn)
    }
}

// convenient way to convert Config into Counter
impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Counter {
        Counter::init(self, orders)
    }
}

// App Element ---

pub enum AppMsg {
    Counter(Msg),
}

pub struct MyApp {
    counter_element: Counter,
}

impl Element for MyApp {
    type Message = AppMsg;
    type Config = Url;

    fn init(_: Url, orders: &mut impl Orders<AppMsg>) -> Self {
        Self {
            counter_element: Counter::config()
                // give it starting value. 10 will be used as default value if
                // we didn't pass value
                .value(100)
                .init(&mut orders.proxy(AppMsg::Counter)),
        }
    }

    fn update(&mut self, msg: AppMsg, orders: &mut impl Orders<AppMsg>) {
        match msg {
            AppMsg::Counter(msg) => self
                .counter_element
                .update(msg, &mut orders.proxy(AppMsg::Counter)),
        }
    }
}

impl View<Node<AppMsg>> for MyApp {
    fn view(&self) -> Node<AppMsg> {
        self.counter_element.view().map_msg(AppMsg::Counter)
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    // mount and start the app at `app` element
    MyApp::start();
}
```

Preview:
![Screenshot](docs-resources/counter-element.webm)

[source code](https://gitlab.com/MAlrusayni/savory/-/tree/master/examples/counter_element)

A lot of things happening in this example, first we have create element struct
`Counter`, and defined its properties, events and style types, this is all done
by the derive macro `Element` which we will explain how it work later, then we
defined an app element that containes the counter element and initialize it in
the `init` funtion. at the end we just call `start` method to mount and start
the app.

## Counter using Savory Elements!

Savory ships with collections of elements, and we will use them to build counter
app and see what features Savory elements gives us.

``` rust
use savory_core::prelude::*;
use savory_elements::prelude::*;
use wasm_bindgen::prelude::*;

pub struct MyApp {
    spin_entry: SpinEntry,
}

pub enum Msg {
    SpinEntry(spin_entry::Msg),
}

impl Element for MyApp {
    type Message = Msg;
    type Config = Url;

    fn init(_: Url, orders: &mut impl Orders<Msg>) -> Self {
        let spin_entry = SpinEntry::config()
            .min(-40.)
            .placeholder(44.)
            .step(5.)
            .max(40.)
            .init(&mut orders.proxy(Msg::SpinEntry));

        Self { spin_entry }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::SpinEntry(msg) => self
                .spin_entry
                .update(msg, &mut orders.proxy(Msg::SpinEntry)),
        };
    }
}

impl View<Node<Msg>> for MyApp {
    fn view(&self) -> Node<Msg> {
        Flexbox::new()
            .center()
            .add(self.spin_entry.view().map_msg(Msg::SpinEntry))
            .and_size(|conf| conf.full())
            .view()
    }
}

#[wasm_bindgen(start)]
pub fn view() {
    MyApp::start();
}
```

Preview:
![Screenshot](docs-resources/counter-using-savory-elements.webm)

[source code](https://gitlab.com/MAlrusayni/savory/-/tree/master/examples/counter_using_savory_elements)

As you can see this example have less lines and more features, what a neat.

It happens that Savory elements have `SpinEntry` element, which work just like
counter, and we used it in our example as simple as that, so Savory tries to
provides you the most needed elements so you don't need to build every thing
from scratch, even if you want build your own element in some way, you can still
use Savory elements as building block in your own element.

# Qucikstart

first thing first, add savory crates into your `Cargo.toml` file:
``` toml
savory-core = "0.5.0"
savory-html = "0.5.0"
savory-elements = "0.5.0"
wasm-bindgen = "0.2.55"
```

TODO

# Ecosystem

- `savory` - savory CLI
- [`savory-core`] - Library for building user interface
- [`savory-html`] - Typed HTML for Savory
- [`savory-elements`] - UI Elements based on Savory
- [`savory-derive`] - Helper derives
- [`savory-theme`] - Helper derives
- [`savory-icons`] - Helper derives

# License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Savory by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[Seed]: https://seed-rs.org/
[Elm tutorial]: https://guide.elm-lang.org/
[`savory-core`]: https://gitlab.com/MAlrusayni/savory/tree/master/core
[`savory-html`]: https://gitlab.com/MAlrusayni/savory/tree/master/html
[`savory-elements`]: https://gitlab.com/MAlrusayni/savory/tree/master/elements
[`savory-derive`]: https://gitlab.com/MAlrusayni/savory/tree/master
[`savory-theme`]: https://gitlab.com/MAlrusayni/savory/tree/master/theme
[`savory-icons`]: https://gitlab.com/MAlrusayni/savory/tree/master/icons
