use crate::prelude::*;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
pub struct ScreenInfoNotifier {
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,
    #[rich(read)]
    current_screen_info: ScreenInfo,
}

pub enum Msg {
    SizeChanged,
    ThemeChanged(Theme),
}

impl Element for ScreenInfoNotifier {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders
            .subscribe(|theme: ThemeChanged| Msg::ThemeChanged(theme.0))
            .stream(streams::window_event(Ev::Resize, |_| Msg::SizeChanged));

        let (width, height) = Self::get_screen_size();
        let screen_info = config.theme.screen_info(width, height);
        orders.notify(NewScreenInfo(screen_info));

        Self {
            current_screen_info: screen_info,
            theme: config.theme,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::ThemeChanged(val) => self.theme = val,
            Msg::SizeChanged => {
                let (width, height) = Self::get_screen_size();
                let new_screen_info = self.theme.screen_info(width, height);
                if self.current_screen_info != new_screen_info {
                    self.current_screen_info = new_screen_info;
                    orders.notify(NewScreenInfo(new_screen_info));
                }
            }
        }
    }
}

impl ScreenInfoNotifier {
    fn get_screen_size() -> (u32, u32) {
        web_sys::window()
            .and_then(|window| {
                let get_val = |js_val: Result<wasm_bindgen::JsValue, _>| {
                    js_val
                        .ok()
                        .and_then(|v| v.as_f64())
                        .map(|v| v.round() as u32)
                        .unwrap_or(0)
                };
                let width = get_val(window.inner_width());
                let height = get_val(window.inner_height());
                Some((width, height))
            })
            .unwrap_or((0, 0))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NewScreenInfo(pub ScreenInfo);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScreenInfo {
    pub class: ScreenClass,
    pub orientation: ScreenOrientation,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScreenClass {
    Phone,
    Tablet,
    Desktop,
    BigDesktop,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScreenOrientation {
    Portrait,
    Landscape,
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> ScreenInfoNotifier {
        ScreenInfoNotifier::init(self, orders)
    }
}
