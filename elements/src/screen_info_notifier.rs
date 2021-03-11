use crate::prelude::*;
use savory::prelude::*;

#[derive(Rich, Element)]
pub struct ScreenInfoNotifier {
    design_system: DesignSystem,
    #[rich(read)]
    current_screen_info: ScreenInfo,
}

pub enum Msg {
    SizeChanged,
    DesignSystemChanged(DesignSystem),
}

impl Element for ScreenInfoNotifier {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>, env: &Env) -> Self {
        orders
            .subscribe(|ds: DesignSystemChanged| Msg::DesignSystemChanged(ds.0))
            .stream(streams::window_event(Ev::Resize, |_| Msg::SizeChanged));

        let (width, height) = Self::get_screen_size();
        let screen_info = config.design_system.screen_info(width, height);
        orders.notify(NewScreenInfo(screen_info));

        Self {
            current_screen_info: screen_info,
            design_system: DesignSystem::default(),
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::DesignSystemChanged(val) => self.design_system = val,
            Msg::SizeChanged => {
                let (width, height) = Self::get_screen_size();
                let new_screen_info = self.design_system.screen_info(width, height);
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
