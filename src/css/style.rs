use super::*;
use crate::prelude::{El, UpdateEl};
use derive_rich::Rich;
use indexmap::IndexMap;
pub use seed::prelude::St;

/// This is the main struct used to build and manipulate css properties, it
/// provieds many methods to do that.
///
/// ```
/// use khalas::css::{Style, Color, unit::{ms, px}, St};
///
/// let mut style = Style::default();
/// style
///     .and_transition(|conf| {
///         conf
///             .add("opacity", |conf| conf.duration(ms(150.)).ease())
///             .add("transform", |conf| conf.duration(ms(150.)).ease())
///             .add("visibility", |conf| conf.duration(ms(150.)).ease())
///     })
///     .and_position(|conf| conf.absolute())
///     .and_background(|conf| conf.color(Color::White))
///     .and_border(|conf| {
///         conf.none()
///             .width(px(0))
///             .radius(px(4))
///     })
///     .and_padding(|conf| conf.x(px(4)).y(px(2)))
///     .and_margin(|conf| conf.top(px(2)))
///     .add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)");
/// ```
#[derive(Default, Debug, Clone, Rich)]
pub struct Style {
    #[rich(write, write(option, rename = try_opacity))]
    pub opacity: Option<Opacity>,
    #[rich(write, write(option, rename = try_gap))]
    pub gap: Option<Gap>,
    #[rich(write, write(option, rename = try_flex_wrap))]
    pub flex_wrap: Option<flexbox::Wrap>,
    #[rich(write, write(option, rename = try_flex_basis))]
    pub flex_basis: Option<flexbox::Basis>,
    #[rich(write, write(option, rename = try_flex_direction))]
    pub flex_direction: Option<flexbox::Direction>,
    #[rich(write, write(option, rename = try_order))]
    pub order: Option<flexbox::Order>,
    #[rich(write, write(option, rename = try_flex_grow))]
    pub flex_grow: Option<flexbox::Grow>,
    #[rich(write, write(option, rename = try_flex_shrink))]
    pub flex_shrink: Option<flexbox::Shrink>,
    #[rich(write, write(option, rename = try_justify_content))]
    pub justify_content: Option<box_align::JustifyContent>,
    #[rich(write, write(option, rename = try_align_content))]
    pub align_content: Option<box_align::AlignContent>,
    #[rich(write, write(option, rename = try_align_items))]
    pub align_items: Option<box_align::AlignItems>,
    #[rich(write, write(option, rename = try_justify_self))]
    pub justify_self: Option<box_align::JustifySelf>,
    #[rich(write, write(option, rename = try_align_self))]
    pub align_self: Option<box_align::AlignSelf>,
    #[rich(write, write(option, rename = try_display))]
    pub display: Option<Display>,
    #[rich(write, write(option, rename = try_visibility))]
    pub visibility: Option<Visibility>,
    #[rich(write, write(option, rename = try_cursor))]
    pub cursor: Option<Cursor>,
    #[rich(write(style = compose, rename = and_background), write)]
    pub background: Option<Background>,
    #[rich(write(style = compose, rename = and_border), write)]
    pub border: Option<Border>,
    #[rich(write(style = compose, rename = and_margin), write)]
    pub margin: Option<Margin>,
    #[rich(write(style = compose, rename = and_padding), write)]
    pub padding: Option<Padding>,
    #[rich(write(style = compose, rename = and_size), write)]
    pub size: Option<Size>,
    #[rich(write(style = compose, rename = and_transition), write)]
    pub transition: Option<Transition>,
    #[rich(write(style = compose, rename = and_position), write)]
    pub position: Option<Position>,
    #[rich(write(style = compose, rename = and_text), write)]
    pub text: Option<Text>,
    #[rich(write(style = compose, rename = and_font), write)]
    pub font: Option<Font>,
    #[rich(write(
        /// Here goes other css properties those doesn't have their own method
        /// (.e.g custom css properties), css values are stored as `String`, so
        /// you won't get typed values like the ones that have it's own methods.
        ///
        /// ```
        /// use khalas::css::{Style, St, Color, unit::em};
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
        style = compose, rename = and_others
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
    /// use khalas::css::{Style, values as val, Color};
    ///
    /// let angle = 1;
    /// let mut style = Style::default();
    /// style.config(|conf| {
    ///     match angle {
    ///         1 => conf.and_text(|conf| conf.color(Color::Red)),
    ///         2 => conf.and_text(|conf| conf.color(Color::Blue)),
    ///         3 => conf.and_text(|conf| conf.color(Color::Green)),
    ///         _ => conf.and_text(|conf| conf.color(Color::Black)),
    ///     }
    /// });
    /// ```
    pub fn config(&mut self, block: impl FnOnce(&mut Self) -> &mut Self) -> &mut Self {
        block(self)
    }

    /// Same as `config` but will be called if `condition` is `true`
    pub fn config_if(
        &mut self,
        condition: bool,
        block: impl FnOnce(&mut Self) -> &mut Self,
    ) -> &mut Self {
        if condition {
            self.config(block)
        } else {
            self
        }
    }

    /// if `condition` is `true` then `block` will be called, otherwise
    /// `else_blcok` will be called
    pub fn config_if_else(
        &mut self,
        condition: bool,
        block: impl FnOnce(&mut Self) -> &mut Self,
        else_block: impl FnOnce(&mut Self) -> &mut Self,
    ) -> &mut Self {
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

    /// this method convert this style to seed [`Style`](seed::virtual_dom::Style)
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
    pub fn add(&mut self, key: impl Into<St>, value: impl ToString) -> &mut Self {
        self.others.add(key, value);
        self
    }

    /// Shortcut for `self.others.try_add()`
    pub fn try_add(&mut self, key: impl Into<St>, value: Option<impl ToString>) -> &mut Self {
        self.others.try_add(key, value);
        self
    }

    /// Shortcut for `self.others.merge()`
    pub fn merge(&mut self, others: &impl ToStyleMap) -> &mut Self {
        self.others.merge(others);
        self
    }

    /// Shortcut for `self.others.try_merge()`
    pub fn try_merge(&mut self, others: Option<&impl ToStyleMap>) -> &mut Self {
        self.others.try_merge(others);
        self
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
        .fold(StyleMap::default(), |mut map, prop| {
            if let Some(prop_map) = prop {
                map.extend(prop_map);
            }
            map
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct StyleMap {
    pub map: IndexMap<St, String>,
}

impl StyleMap {
    /// This method provied a way to add custom style or css style that doesn't
    /// have it's own method yet.
    ///
    /// ```
    /// use khalas::css::{StyleMap, values as val, St};
    ///
    /// let mut map = StyleMap::default();
    /// map.add(St::UserSelect, val::None)
    ///     .add(St::BoxSizing, val::BorderBox);
    /// ```
    pub fn add(&mut self, key: impl Into<St>, value: impl ToString) -> &mut Self {
        self.map.insert(key.into(), value.to_string());
        self
    }

    /// This method is similar `add` but it accept an optional value, if the
    /// passed value is `None` then nothing added to the style.
    pub fn try_add(&mut self, key: impl Into<St>, value: Option<impl ToString>) -> &mut Self {
        if let Some(value) = value {
            self.add(key, value)
        } else {
            self
        }
    }

    /// Merge this style map with other
    pub fn merge(&mut self, other: &impl ToStyleMap) -> &mut Self {
        self.map.extend(other.style_map().map);
        self
    }

    /// This method is similar to `merge` but it accept an optional value.
    pub fn try_merge(&mut self, other: Option<&impl ToStyleMap>) -> &mut Self {
        if let Some(other) = other {
            self.merge(other)
        } else {
            self
        }
    }

    fn extend(&mut self, other: Self) -> &mut Self {
        self.map.extend(other.map);
        self
    }
}

/// Any type that act like css property should implemente this trait
pub trait ToStyleMap {
    fn style_map(&self) -> StyleMap;
}
