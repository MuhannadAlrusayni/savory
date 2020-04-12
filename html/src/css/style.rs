use super::*;
use crate::prelude::{El, UpdateEl};
use derive_rich::Rich;
use indexmap::IndexMap;
pub use seed::prelude::St;
use std::ops::{Add, AddAssign};

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
#[derive(Default, PartialEq, Debug, Clone, Rich)]
pub struct Style {
    #[rich(write, write(option))]
    pub opacity: Option<Opacity>,
    #[rich(write, write(option))]
    pub gap: Option<Gap>,
    #[rich(write, write(option))]
    pub flex_wrap: Option<flexbox::Wrap>,
    #[rich(write, write(option))]
    pub flex_basis: Option<flexbox::Basis>,
    #[rich(write, write(option))]
    pub flex_direction: Option<flexbox::Direction>,
    #[rich(write, write(option))]
    pub order: Option<flexbox::Order>,
    #[rich(write, write(option))]
    pub flex_grow: Option<flexbox::Grow>,
    #[rich(write, write(option))]
    pub flex_shrink: Option<flexbox::Shrink>,
    #[rich(write, write(option))]
    pub justify_content: Option<box_align::JustifyContent>,
    #[rich(write, write(option))]
    pub align_content: Option<box_align::AlignContent>,
    #[rich(write, write(option))]
    pub align_items: Option<box_align::AlignItems>,
    #[rich(write, write(option))]
    pub justify_self: Option<box_align::JustifySelf>,
    #[rich(write, write(option))]
    pub align_self: Option<box_align::AlignSelf>,
    #[rich(write, write(option))]
    pub display: Option<Display>,
    #[rich(write, write(option))]
    pub visibility: Option<Visibility>,
    #[rich(write, write(option))]
    pub cursor: Option<Cursor>,
    #[rich(write(style = compose), write, write(option))]
    pub background: Option<Background>,
    #[rich(write(style = compose), write, write(option))]
    pub border: Option<Border>,
    #[rich(write(style = compose), write, write(option))]
    pub margin: Option<Margin>,
    #[rich(write(style = compose), write, write(option))]
    pub padding: Option<Padding>,
    #[rich(write(style = compose), write, write(option))]
    pub size: Option<Size>,
    #[rich(write(style = compose), write, write(option))]
    pub transition: Option<Transition>,
    #[rich(write(style = compose), write, write(option))]
    pub position: Option<Position>,
    #[rich(write(style = compose), write, write(option))]
    pub text: Option<Text>,
    #[rich(write(style = compose), write, write(option))]
    pub font: Option<Font>,
    #[rich(write(
        /// Here goes other css properties those doesn't have their own method
        /// (.e.g custom css properties), css values are stored as `String`, so
        /// you won't get typed values like the ones that have it's own methods.
        ///
        /// ```
        /// use savory::css::{Style, St, Color, unit::em};
        ///
        /// fn get_color() -> Option<Color> {
        ///     Some(Color::Black)
        /// }
        ///
        /// let mut style = Style::default();
        /// style
        ///     .and_others(|conf| {
        ///         conf.try_add(St::from("--box-bg-color"), get_color())
        ///             .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
        ///             .add(St::from("--container-gap"), em(2.))
        ///     });
        /// ```
        style = compose
    ))]
    pub others: StyleMap,
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
        self.style_map()
            .map
            .into_iter()
            .fold(Option::None, |mut css, (key, value)| {
                *css.get_or_insert(String::default()) += &format!("{}: {};", key.as_str(), value);
                css
            })
    }

    /// this method convert this style to seed `Style`
    pub fn to_seed_style(&self) -> Option<seed::virtual_dom::Style> {
        self.style_map()
            .map
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
        self.others = self.others.add(key, value);
        self
    }

    /// Shortcut for `self.others.try_add()`
    pub fn try_add(mut self, key: impl Into<St>, value: Option<impl ToString>) -> Self {
        self.others = self.others.try_add(key, value);
        self
    }

    /// Shortcut for `self.others.merge()`
    pub fn merge(mut self, others: &impl ToStyleMap) -> Self {
        self.others = self.others.merge(others);
        self
    }

    /// Shortcut for `self.others.try_merge()`
    pub fn try_merge(mut self, others: Option<&impl ToStyleMap>) -> Self {
        self.others = self.others.try_merge(others);
        self
    }
}

impl Add for Style {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for Style {
    fn add_assign(&mut self, other: Self) {
        self.opacity = other.opacity.or(self.opacity);
        self.gap = other.gap.or(self.gap);
        self.flex_wrap = other.flex_wrap.or(self.flex_wrap);
        self.flex_basis = other.flex_basis.or(self.flex_basis);
        self.flex_direction = other.flex_direction.or(self.flex_direction);
        self.order = other.order.or(self.order);
        self.flex_grow = other.flex_grow.or(self.flex_grow);
        self.flex_shrink = other.flex_shrink.or(self.flex_shrink);
        self.justify_content = other.justify_content.or(self.justify_content);
        self.align_content = other.align_content.or(self.align_content);
        self.align_items = other.align_items.or(self.align_items);
        self.justify_self = other.justify_self.or(self.justify_self);
        self.align_self = other.align_self.or(self.align_self);
        self.display = other.display.or(self.display);
        self.visibility = other.visibility.or(self.visibility);
        self.cursor = other.cursor.or(self.cursor);

        fn sum_vals<T: Add<Output = T>>(val1: Option<T>, val2: Option<T>) -> Option<T> {
            match (val1, val2) {
                (Some(val1), Some(val2)) => Some(val1 + val2),
                (Some(val), None) | (None, Some(val)) => Some(val),
                (None, None) => None,
            }
        }

        self.background = sum_vals(self.background.clone(), other.background);
        self.border = sum_vals(self.border, other.border);
        self.margin = sum_vals(self.margin, other.margin);
        self.padding = sum_vals(self.padding, other.padding);
        self.size = sum_vals(self.size, other.size);
        self.transition = sum_vals(self.transition.clone(), other.transition);
        self.position = sum_vals(self.position.clone(), other.position);
        self.text = sum_vals(self.text.clone(), other.text);
        self.font = sum_vals(self.font.clone(), other.font);
        self.others += other.others;
    }
}

impl<Msg> UpdateEl<Msg> for Style {
    fn update_el(self, el: &mut El<Msg>) {
        if let Some(style) = self.to_seed_style() {
            style.update_el(el);
        }
    }
}

impl ToStyleMap for Style {
    fn style_map(&self) -> StyleMap {
        fn map(property: &Option<impl ToStyleMap>) -> Option<StyleMap> {
            property.as_ref().map(|prop| prop.style_map())
        }

        vec![
            map(&self.opacity),
            map(&self.gap),
            map(&self.flex_wrap),
            map(&self.flex_basis),
            map(&self.flex_direction),
            map(&self.flex_grow),
            map(&self.flex_shrink),
            map(&self.justify_content),
            map(&self.align_content),
            map(&self.align_items),
            map(&self.justify_self),
            map(&self.align_self),
            map(&self.display),
            map(&self.visibility),
            map(&self.cursor),
            map(&self.background),
            map(&self.border),
            map(&self.margin),
            map(&self.padding),
            map(&self.size),
            map(&self.transition),
            map(&self.position),
            map(&self.text),
            map(&self.font),
            Some(self.others.clone()),
        ]
        .into_iter()
        .fold(StyleMap::default(), |map, prop| {
            if let Some(prop_map) = prop {
                map.extend(prop_map)
            } else {
                map
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StyleMap {
    pub map: IndexMap<St, String>,
}

impl Add for StyleMap {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for StyleMap {
    fn add_assign(&mut self, other: Self) {
        for (key, val) in other.map.into_iter() {
            self.map.insert(key, val);
        }
    }
}

impl StyleMap {
    /// This method provied a way to add custom style or css style that doesn't
    /// have it's own method yet.
    ///
    /// ```
    /// use savory::css::{StyleMap, values as val, St};
    ///
    /// let mut map = StyleMap::default();
    /// map.add(St::UserSelect, val::None)
    ///     .add(St::BoxSizing, val::BorderBox);
    /// ```
    pub fn add(mut self, key: impl Into<St>, value: impl ToString) -> Self {
        self.map.insert(key.into(), value.to_string());
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

    /// Merge this style map with other
    pub fn merge(mut self, other: &impl ToStyleMap) -> Self {
        self.map.extend(other.style_map().map);
        self
    }

    /// This method is similar to `merge` but it accept an optional value.
    pub fn try_merge(self, other: Option<&impl ToStyleMap>) -> Self {
        if let Some(other) = other {
            self.merge(other)
        } else {
            self
        }
    }

    fn extend(mut self, other: Self) -> Self {
        self.map.extend(other.map);
        self
    }
}

/// Any type that act like css property should implemente this trait
pub trait ToStyleMap {
    fn style_map(&self) -> StyleMap;
}

// macro used to implemente Add and AddAssign for Style, and style properties
#[macro_export]
macro_rules! impl_add_and_add_assign {
    ( @attr $ty:ident $attr:ident clone $(,)? ) => {
        |s: &mut $ty| { s.$attr.clone() }
    };

    ( @attr $ty:ident $attr:ident $(,)? ) => {
        |s: &mut $ty| { s.$attr }
    };

    ( $name:ident { $( $attr:ident $( { $($tokens:tt)* } $(,)? )? )* } ) => {
        impl ::std::ops::Add for $name {
            type Output = Self;

            fn add(mut self, other: Self) -> Self::Output {
                self += other;
                self
            }
        }

        impl ::std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                $(
                    let get_val = impl_add_and_add_assign!(@attr $name $attr $( $( $tokens )* )? );
                    let val = get_val(self);
                    self.$attr = other.$attr.or(val);
                )*
            }
        }
    };
}
