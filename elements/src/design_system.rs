//! Traits and types used to generate elements/views styles.

use crate::prelude::*;
use savory::prelude::Env;
use std::{ops::Deref, rc::Rc};

pub trait ViewStyle {
    type StyleMap;
}

pub struct Designer<T>(Rc<dyn Design<T>>);

impl<T> From<Rc<dyn Design<T>>> for Designer<T> {
    fn from(source: Rc<dyn Design<T>>) -> Self {
        Designer(Rc::clone(&source))
    }
}

impl<T> Deref for Designer<T> {
    type Target = dyn Design<T>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> Clone for Designer<T> {
    fn clone(&self) -> Self {
        Designer(Rc::clone(&self.0))
    }
}

pub trait Design<T: DataLens + ViewStyle> {
    fn design(&self, lens: <T as DataLens>::Data, env: &Env) -> <T as ViewStyle>::StyleMap;
}

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
