//! ProgressBar element
//!
//! ProgressBar used to show the progression of long task.
//!
//! See [`ProgressBar`] docs to find out more about its methods.
//!
//! # Usage
//! TODO
//!
//! [`ProgressBar`]: crate::prelude::ProgressBar

use crate::{id::Id, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_style::{self as style, prelude::*};

pub enum Msg {
    DesignSystem(DesignSystem),
    Disable(bool),
    Value(f32),
}

#[derive(Element, Rich)]
#[element(style_map(indicator, progress_bar))]
pub struct ProgressBar {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Option<Id>,
    design_system: DesignSystem,

    #[rich(read(copy))]
    #[element(config(default = "0.0", no_pub), data_lens(copy))]
    value: f32,
    #[rich(read(copy))]
    #[element(config(default = "100.0", no_pub), data_lens(copy))]
    max: f32,
    #[rich(read(copy))]
    #[element(config(default = "0.0", no_pub), data_lens(copy))]
    min: f32,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default), data_lens(copy))]
    disabled: bool,

    #[element(config, data_lens(copy))]
    color: Option<style::Color>,
}

impl ProgressBar {
    fn set_value(&mut self, val: f32, _: &mut impl Orders<Msg>) {
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
}

impl Element for ProgressBar {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>, _: &Env) -> Self {
        orders.subscribe(|ds: DesignSystemChanged| Msg::DesignSystem(ds.0));

        Self {
            id: config.id,
            design_system: DesignSystem::default(),
            value: config.value,
            max: config.max,
            min: config.min,
            disabled: config.disabled,
            color: config.color,
        }
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::DesignSystem(val) => self.design_system = val,
            Msg::Disable(val) => self.disabled = val,
            Msg::Value(val) => self.set_value(val, orders),
        }
    }
}

impl View<Node<Msg>> for ProgressBar {
    fn view(&self) -> Node<Msg> {
        let style_map = self.design_system.progress_bar(self.data_lens());
        let indicator = html::div().class("indicator").style(style_map.indicator);

        html::div()
            .try_id(self.id.clone())
            .class("progress-bar")
            .style(style_map.progress_bar)
            .push(indicator)
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
