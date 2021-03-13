//! Traits and types used to generate elements/views styles.

pub mod default_ds;

use crate::prelude::*;
use savory_style as style;
use std::{ops::Deref, rc::Rc};

/// Type that hold `DesignSystemImpl` trait
#[derive(Clone)]
pub struct DesignSystem(Rc<dyn DesignSystemImpl>);

impl Default for DesignSystem {
    fn default() -> Self {
        DesignSystem::new(default_ds::SavoryDS::default())
    }
}

impl DesignSystem {
    pub fn new<DS: DesignSystemImpl + 'static>(ds: DS) -> Self {
        Self(Rc::new(ds))
    }
}

impl Deref for DesignSystem {
    type Target = dyn DesignSystemImpl;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// Trait used by design system types to generate elements/views styles.
pub trait DesignSystemImpl {
    // // this implementions is from
    // // https://github.com/mdgriffith/style-elements/blob/master/experiments/Aligned/src/Element.elm#L1269
    // /// Calculate screen info based on it's width and height
    // fn screen_info(&self, width: u32, height: u32) -> ScreenInfo {
    //     let class = match width {
    //         0..=600 => ScreenClass::Phone,
    //         601..=1200 => ScreenClass::Tablet,
    //         1201..=1800 => ScreenClass::Desktop,
    //         _ => ScreenClass::BigDesktop,
    //     };
    //     let orientation = if width < height {
    //         ScreenOrientation::Portrait
    //     } else {
    //         ScreenOrientation::Landscape
    //     };
    //     ScreenInfo { class, orientation }
    // }
    /// Generate needed styles for `Text` view
    fn text(&self, class: text::TextLens) -> style::Style;
    /// Generate needed styles for `Button` view
    fn button(&self, class: button::ButtonLens) -> style::Style;
    /// Generate needed styles for `Switch` view
    fn switch(&self, class: switch::SwitchLens) -> switch::StyleMap;
    /// Generate needed styles for `Radio` view
    fn radio(&self, class: radio::RadioLens) -> radio::StyleMap;
    /// Generate needed styles for `TextInput` view
    fn text_input(&self, class: text_input::TextInputLens) -> style::Style;
    /// Generate needed styles for `ProgressBar` view
    fn progress_bar(&self, class: progress_bar::ProgressBarLens) -> progress_bar::StyleMap;
}
