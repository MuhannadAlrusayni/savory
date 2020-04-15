use super::*;
use crate::prelude::{El, UpdateEl};
use indexmap::IndexMap;
use std::borrow::Cow;

pub use seed::prelude::St;

/// This is the main struct used to build and manipulate css properties, it
/// provieds many methods to do that.
///
/// ```
/// use savory::css::{Style, Color, unit::{ms, px}, St};
///
/// let mut style = Style::default();
/// style
///     .and_transition(|conf| {
///         conf
///             .add("opacity", |conf| conf.set_duration(ms(150.)).ease())
///             .add("transform", |conf| conf.set_duration(ms(150.)).ease())
///             .add("visibility", |conf| conf.set_duration(ms(150.)).ease())
///     })
///     .and_position(|conf| conf.absolute())
///     .and_background(|conf| conf.set_color(Color::White))
///     .and_border(|conf| {
///         conf.none()
///             .set_width(px(0))
///             .set_radius(px(4))
///     })
///     .and_padding(|conf| conf.set_x(px(4)).set_y(px(2)))
///     .and_margin(|conf| conf.set_top(px(2)))
///     .add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)");
/// ```
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Style {
    values: StyleValues,
}

macro_rules! setter_functions {
    ( @more_fns $prop_ty:ident and ) => {
        paste::item! {
            pub fn [<and_ $prop_ty:snake>](mut self, val: impl FnOnce($prop_ty) -> $prop_ty) -> Self
            where
                $prop_ty: Default + UpdateStyleValues,
            {
                self.values = val($prop_ty::default()).update_style_values(self.values);
                self
            }

        }
    };
    ( $( $prop_ty:ident $( +$ext:ident )? $(,)? )+ ) => {
        $(
            paste::item! {
                pub fn [<$prop_ty:snake>](mut self, val: impl Into<$prop_ty>) -> Self
                where
                    $prop_ty: UpdateStyleValues,
                {
                    self.values = val.into().update_style_values(self.values);
                    self
                }

                pub fn [<try_ $prop_ty:snake>](self, val: Option<impl Into<$prop_ty>>) -> Self {
                    if let Some(val) = val {
                        self.[<$prop_ty:snake>](val)
                    } else {
                        self
                    }
                }
            }
            $( setter_functions!(@more_fns $prop_ty $ext); )?
        )+
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    /// This method accept closure that configure the style
    ///
    /// ```
    /// use savory::css::{Style, values as val, Color};
    ///
    /// let angle = 1;
    /// let mut style = Style::default();
    /// style.config(|conf| {
    ///     match angle {
    ///         1 => conf.and_text(|conf| conf.set_color(Color::Red)),
    ///         2 => conf.and_text(|conf| conf.set_color(Color::Blue)),
    ///         3 => conf.and_text(|conf| conf.set_color(Color::Green)),
    ///         _ => conf.and_text(|conf| conf.set_color(Color::Black)),
    ///     }
    /// });
    /// ```
    pub fn config(self, block: impl FnOnce(Self) -> Self) -> Self {
        block(self)
    }

    /// Same as `config` but will be called if `condition` is `true`
    pub fn config_if(self, condition: bool, block: impl FnOnce(Self) -> Self) -> Self {
        if condition {
            self.config(block)
        } else {
            self
        }
    }

    /// if `condition` is `true` then `block` will be called, otherwise
    /// `else_blcok` will be called
    pub fn config_if_else(
        self,
        condition: bool,
        block: impl FnOnce(Self) -> Self,
        else_block: impl FnOnce(Self) -> Self,
    ) -> Self {
        if condition {
            self.config(block)
        } else {
            self.config(else_block)
        }
    }

    /// This method convert this style to html style value
    pub fn to_css(&self) -> Option<String> {
        self.values
            .clone()
            .into_iter()
            .fold(Option::None, |mut css, (key, value)| {
                *css.get_or_insert(String::default()) += &format!("{}: {};", key.as_str(), value);
                css
            })
    }

    /// this method convert this style to seed `Style`
    pub fn to_seed_style(&self) -> Option<seed::virtual_dom::Style> {
        self.values
            .clone()
            .into_iter()
            .fold(Option::None, |mut style, (key, value)| {
                style
                    .get_or_insert(seed::virtual_dom::Style::empty())
                    .add(key, value);
                style
            })
    }

    /// Shortcut for `self.others.add()`
    pub fn add(mut self, key: impl Into<St>, value: impl ToString) -> Self {
        self.values = self.values.add(key, value);
        self
    }

    /// Shortcut for `self.values.try_add()`
    pub fn try_add(mut self, key: impl Into<St>, value: Option<impl ToString>) -> Self {
        self.values = self.values.try_add(key, value);
        self
    }

    pub fn add_custom(mut self, key: impl Into<Cow<'static, str>>, value: impl ToString) -> Self {
        self.values = self.values.add_custom(key, value);
        self
    }

    pub fn try_add_custom(
        mut self,
        key: impl Into<Cow<'static, str>>,
        value: Option<impl ToString>,
    ) -> Self {
        self.values = self.values.try_add_custom(key, value);
        self
    }

    /// Shortcut for `self.values.merge()`
    pub fn merge(mut self, other: impl UpdateStyleValues) -> Self {
        self.values = self.values.merge(other);
        self
    }

    /// Shortcut for `self.values.try_merge()`
    pub fn try_merge(mut self, other: Option<impl UpdateStyleValues>) -> Self {
        self.values = self.values.try_merge(other);
        self
    }

    setter_functions! {
        Opacity,
        Gap,
        AlignContent,
        AlignItems,
        JustifyContent,
        JustifySelf,
        AlignSelf,
        FlexWrap,
        FlexBasis,
        FlexDirection,
        FlexOrder,
        FlexGrow,
        FlexShrink,
        Display,
        Visibility,
        Cursor,
        Background +and,
        Border +and,
        Margin +and,
        Padding +and,
        Size +and,
        Transition +and,
        Position +and,
        Text +and,
        Font +and,
    }
}

impl Style {}

impl<Msg> UpdateEl<Msg> for Style {
    fn update_el(self, el: &mut El<Msg>) {
        if let Some(style) = self.to_seed_style() {
            style.update_el(el);
        }
    }
}

pub trait UpdateStyleValues {
    fn update_style_values(self, values: StyleValues) -> StyleValues;
}

#[derive(Clone, IntoIterator, Index, IndexMut, Debug, PartialEq, Default)]
pub struct StyleValues(pub IndexMap<St, String>);

impl StyleValues {
    pub fn get(&self, key: &St) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }

    /// This method provied a way to add custom style or css style that doesn't
    /// have it's own method yet.
    ///
    /// ```
    /// use savory::css::{StyleValues, UpdateStyleValues, values as val, St};
    ///
    /// let mut map = StyleValues::default();
    /// map.add(St::UserSelect, val::None)
    ///     .add(St::BoxSizing, val::BorderBox);
    /// ```
    pub fn add(mut self, key: impl Into<St>, value: impl ToString) -> Self {
        self.0.insert(key.into(), value.to_string());
        self
    }

    /// This method is similar `add` but it accept an optional value, if the
    /// passed value is `None` then nothing added to the style.
    pub fn try_add(self, key: impl Into<St>, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            self.add(key, value)
        } else {
            self
        }
    }

    pub fn add_custom(self, key: impl Into<Cow<'static, str>>, value: impl ToString) -> Self {
        self.add(St::Custom(key.into()), value)
    }

    pub fn try_add_custom(
        self,
        key: impl Into<Cow<'static, str>>,
        value: Option<impl ToString>,
    ) -> Self {
        self.try_add(St::Custom(key.into()), value)
    }

    /// Merge this style map with other
    pub fn merge(mut self, other: impl UpdateStyleValues) -> Self {
        self = other.update_style_values(self);
        self
    }

    /// This method is similar to `merge` but it accept an optional value.
    pub fn try_merge(self, other: Option<impl UpdateStyleValues>) -> Self {
        if let Some(other) = other {
            self.merge(other)
        } else {
            self
        }
    }
}
