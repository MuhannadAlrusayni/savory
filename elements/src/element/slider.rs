//! Slider element
//!
//! Slider used to select a value from of values. Silder will try to fill
//! horizontal space of its container.
//!
//! See [`Slider`] docs to find out more about its methods.
//!
//! # Usage
//! TODO
//!
//! [`Slider`]: crate::prelude::Slider

use crate::{id::Id, prelude::*};
use derive_rich::Rich;
use savory::{prelude::*, web_sys};
use savory_style::{self as style, prelude::*};

pub enum Msg {
    Rerender,
    Focus(bool),
    MouseOver(bool),
    Disable(bool),
    Value(f32),
    MoveValueTo(i32),
}

#[derive(Element, Rich)]
#[element(style_map(slider, indicator, button, bar))]
pub struct Slider {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Option<Id>,
    env: Env,
    slider_ref: ElRef<web_sys::HtmlElement>,

    #[rich(read(copy))]
    #[element(config(default = "0.0", no_pub), data_lens)]
    value: f32,
    #[rich(read(copy))]
    #[element(config(default = "100.0", no_pub), data_lens)]
    max: f32,
    #[rich(read(copy))]
    #[element(config(default = "0.0", no_pub), data_lens)]
    min: f32,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default), data_lens)]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(data_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(data_lens)]
    mouse_over: bool,

    #[element(config, data_lens)]
    color: Option<style::Color>,
}

impl Slider {
    fn set_value(&mut self, val: f32) {
        let min = self.min;
        let max = self.max;
        let value = self.value;

        if val < min && value != min {
            self.value = min;
        } else if val > max && value != max {
            self.value = max;
        } else if val <= max && val >= min && val != value {
            self.value = val;
        } else {
            // do nothing
        }
    }

    fn move_by(&mut self, x: i32) {
        fn map_range(x: f64, (in_min, in_max): (f64, f64), (out_min, out_max): (f64, f64)) -> f64 {
            (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
        }

        let rect = self
            .slider_ref
            .get()
            .expect("Slider is not rendered yet!, cannot get it.")
            .get_bounding_client_rect();
        let val = map_range(
            x as f64,
            (rect.left(), rect.right()),
            (self.min as f64, self.max as f64),
        );

        self.set_value(val as f32);
    }
}

impl Element for Slider {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>, env: Env) -> Self {
        orders.subscribe(|_: RerenderRequested| Msg::Rerender);

        Self {
            id: config.id,
            env,
            slider_ref: ElRef::default(),
            value: config.value,
            max: config.max,
            min: config.min,
            disabled: config.disabled,
            focused: false,
            mouse_over: false,
            color: config.color,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Rerender => {}
            Msg::Disable(val) => self.disabled = val,
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::Focus(val) => self.focused = val,
            Msg::Value(val) => {
                if !self.disabled {
                    self.set_value(val)
                }
            }
            Msg::MoveValueTo(val) => {
                if !self.disabled {
                    self.move_by(val)
                }
            }
        }
    }
}

impl View<Node<Msg>> for Slider {
    fn view(&self) -> Node<Msg> {
        let style_map = self
            .env
            .designer::<Slider>()
            .design(self.data_lens(), &self.env);

        let button = html::div().class("button").style(style_map.button);

        let bar = html::div().style(style_map.bar).el_ref(&self.slider_ref);
        let indicator = html::div().style(style_map.indicator);

        let mouse_handler = |ev: web_sys::MouseEvent| {
            if ev.buttons() == 1 {
                Some(Msg::MoveValueTo(ev.client_x()))
            } else {
                None
            }
        };

        let touch_handler = |ev: web_sys::TouchEvent| {
            Msg::MoveValueTo(ev.touches().get(0).expect("Touch wasn't found").client_x())
        };

        html::div()
            .try_id(self.id.clone())
            .class("slider")
            .style(style_map.slider)
            .tab_index(0)
            .push(bar)
            .push(indicator)
            .push(button)
            .on_mouse_down(mouse_handler)
            .on_mouse_move(mouse_handler)
            .on_touch_start(touch_handler)
            .on_touch_move(touch_handler)
            .on_mouse_down(|_| Msg::Focus(true))
            .on_mouse_up(|_| Msg::Focus(false))
            .on_touch_start(|_| Msg::Focus(true))
            .on_touch_end(|_| Msg::Focus(false))
            .on_focus(|_| Msg::Focus(true))
            .on_blur(|_| Msg::Focus(false))
            .on_mouse_enter(|_| Msg::MouseOver(true))
            .on_mouse_leave(|_| Msg::MouseOver(false))
    }
}

impl Config {
    pub fn value(mut self, val: f32) -> Self {
        let min = self.min;
        let max = self.max;
        let value = self.value;

        if val < min && value != min {
            self.value = min;
        } else if val > max && value != max {
            self.value = max;
        } else if val <= max && val >= min && val != value {
            self.value = val;
        } else {
            // do nothing
        }

        self
    }

    pub fn max(mut self, val: f32) -> Self {
        let min = self.min;
        let max = self.max;
        let value = self.value;

        if val > min && val != max {
            self.max = val;
            self.value(value)
        } else if val < min {
            self.max = self.min;
            self.min(val)
        } else if val == min {
            self.max = self.min + 1.0;
            self.min(val)
        } else {
            self
        }
    }

    pub fn min(mut self, val: f32) -> Self {
        let min = self.min;
        let max = self.max;
        let value = self.value;

        if val < max && val != min {
            self.min = val;
            self.value(value)
        } else if val > max {
            self.min = self.max;
            self.max(val)
        } else if val == max {
            self.min = self.max - 1.0;
            self.max(val)
        } else {
            self
        }
    }
}
