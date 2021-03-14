//! Text view
//!
//! Text plays an importent rule in UI/UX developemnt, Savory ships with
//! powerfull text view that covers most users need. There is a lot of
//! builder-style methods that tweak the text style and behavior.
//!
//! See [`Text`] docs to find out more about these methods.
//!
//! # Usage
//! TODO
//!
//! [`Text`]: crate::prelude::Text

use crate::{id::Id, prelude::*};

use derive_rich::Rich;
use savory::prelude::*;
use savory_style::{
    font::{Size, Style as FontStyle, Weight},
    prelude::*,
    // text::Direction,
    text::{LineHeight, TextAlign, TextIndent, TextJustify, TextShadow},
    unit,
    values as val,
    Color,
};
use std::borrow::Cow;

/// Text view
///
/// see the [module docs](crate::prelude::text)
#[derive(Rich, Clone, Element)]
#[element(view, style_map)]
pub struct Text {
    #[rich(write)]
    pub id: Option<Id>,
    #[rich(write)]
    pub env: Env,
    #[rich(write, write(option), value_fns = {
        rtl = Direction::Rtl,
        ltr = Direction::Ltr,
        auto_dir = Direction::Auto,
    })]
    pub direction: Option<Direction>,

    #[rich(write)]
    pub text: Cow<'static, str>,
    #[rich(write, read(copy, rename = is_disabled))]
    #[element(data_lens)]
    pub disabled: bool,

    #[rich(write, write(option), value_fns = {
        xx_small = val::XXSmall,
        x_small = val::XSmall,
        small = val::Small,
        large = val::Large,
        x_large = val::XLarge,
        xx_large = val::XXLarge,
        smaller = val::Smaller,
        larger = val::Larger,
    })]
    #[element(data_lens(clone))]
    pub size: Option<Size>,
    #[rich(write, write(option), value_fns = {
        italic = val::Italic,
        oblique = val::Oblique,
    })]
    #[element(data_lens)]
    pub style: Option<FontStyle>,
    #[rich(write)]
    #[element(data_lens)]
    pub small_caps: bool,
    #[rich(write, write(option), value_fns = {
        thin_weight        = Weight::L100,
        extra_light_weight = Weight::L200,
        light_weight       = Weight::L300,
        normal_weight      = Weight::L400,
        medium_weight      = Weight::L500,
        semi_bold_weight   = Weight::L600,
        bold_weight        = Weight::L700,
        bold               = Weight::L700,
        ultra_bold_weight  = Weight::L800,
        heavy_weight       = Weight::L900,
    })]
    #[element(data_lens)]
    pub weight: Option<Weight>,
    #[rich(write, write(option))]
    #[element(data_lens)]
    pub color: Option<Color>,
    #[rich(write, write(option))]
    #[element(data_lens(clone))]
    pub letter_spacing: Option<unit::Length>,
    #[rich(write, write(option))]
    #[element(data_lens(clone))]
    pub word_spacing: Option<unit::Length>,
    #[rich(write, write(option))]
    #[element(data_lens(clone))]
    pub lines_spacing: Option<LineHeight>,
    #[rich(write, write(option), value_fns = {
        start = val::Start,
        end = val::End,
        left = val::Left,
        right = val::Right,
        center = val::Center,
        justify = val::Justify,
    })]
    #[element(data_lens)]
    pub align: Option<TextAlign>,
    #[rich(write, write(option), value_fns = {
        justify_by_word = val::InterWord,
        justify_by_character = val::InterCharacter,
    })]
    #[element(data_lens)]
    pub justify_by: Option<TextJustify>,
    #[rich(write, write(option))]
    #[element(data_lens(clone))]
    pub indent: Option<TextIndent>,
    #[rich(write)]
    #[element(data_lens)]
    pub wrap: bool,
    #[rich(write, write(option), write(style = compose))]
    #[element(data_lens(clone))]
    pub shadow: Option<TextShadow>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Rtl,
    Ltr,
    Auto,
}

impl<Msg> View<Node<Msg>> for Text {
    fn view(&self) -> Node<Msg> {
        let style = self.env.ds().text(self.data_lens());
        html::p()
            .config(|p| match self.direction {
                Some(Direction::Rtl) => p.dir("rtl"),
                Some(Direction::Ltr) => p.dir("ltr"),
                Some(Direction::Auto) => p.dir("auto"),
                None => p,
            })
            .try_id(self.id.clone())
            .class("text")
            .style(style)
            .push(self.text.clone())
    }
}

impl Text {
    pub fn new(text: impl Into<Cow<'static, str>>, env: Env) -> Self {
        Text {
            id: None,
            env,
            direction: None,
            text: text.into(),
            disabled: false,
            size: None,
            style: None,
            small_caps: false,
            weight: None,
            color: None,
            letter_spacing: None,
            word_spacing: None,
            lines_spacing: None,
            align: None,
            justify_by: None,
            indent: None,
            wrap: false,
            shadow: None,
        }
    }
}
