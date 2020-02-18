use super::*;
use derive_rich::Rich;
use indexmap::IndexMap;
pub use seed::prelude::St;
use seed::prelude::UpdateEl;
use seed::prelude::*;

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
///     .and_background(|conf| conf.color(Color::white))
///     .and_border(|conf| {
///         conf.none()
///             .width(px(0.))
///             .radius(px(4.))
///     })
///     .and_add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)")
///     .and_padding(|conf| conf.x(px(4.)).y(px(2)))
///     .and_margin(|conf| conf.top(px(popover.offset)))
///     .and_config_block(|conf| {
///         if popover.is_visible() {
///             conf.opacity(1.).visibility(val::Visible)
///         } else {
///             conf.visibility(val::Hidden).opacity(0.)
///         }
///     });
///
/// div![
///     style,
///     ...
/// ]
/// ```
#[derive(Default, Debug, Clone, Rich)]
pub struct Style {
    #[rich(
        write(
            /// ```
            /// style.opacity(0.75);
            /// ```
        ),
        write(
            /// ```
            /// style.try_opacity(Some(0.75));
            /// ```
            option, rename = try_opacity
        ))]
    pub opacity: Option<Opacity>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::unit::{px, em};
            ///
            /// style.gap(px(2.))
            ///      // this can take percent value too (.e.g 40%).
            ///      .gap(0.4)
            ///      // and can take row and column each with different value
            ///      .gap((em(4.), em(8.)))
            /// ```
        ),
        write(
            /// ```
            /// fn calc_gap() -> Option<Gap> { .. }
            ///
            /// style.try_gap(calc_gap());
            /// ```
            option, rename = try_gap
        ))]
    pub gap: Option<Gap>,
    #[rich(
        write(
            /// This method accept `Wrap`, `Nowrap` and `WrapReverse`.
            ///
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.flex_wrap(val::Wrap);
            /// ```
        ),
        write(
            /// ```
            /// fn get_wrap() -> Option<Wrap> { .. }
            ///
            /// style.try_flex_wrap(get_wrap());
            /// ```
            option, rename = try_flex_wrap
        ))]
    pub flex_wrap: Option<flexbox::Wrap>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::{values as val, unit::{px, em}};
            ///
            /// style
            ///     // pass auto
            ///     .flex_basis(val::Auto)
            ///     // or 4px
            ///     .flex_basis(px(4.))
            ///     // or 80%
            ///     .flex_basis(0.8); // not that f32 get converted to unit::Percent type
            /// ```
        ),
        write(
            /// ```
            /// fn get_basis() -> Option<Basis> { .. }
            ///
            /// style.try_flex_basis(get_basis());
            /// ```
            option, rename = try_flex_basis
        ))]
    pub flex_basis: Option<flexbox::Basis>,
    #[rich(
        write(
            /// This method accept `Row`, `RowReverse`, `Column` and `ColumnReverse`.
            ///
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.flex_direction(val::Column);
            /// ```
        ),
        write(
            /// ```
            /// fn get_direction() -> Option<Direction> { .. }
            ///
            /// style.try_flex_direction(get_direction());
            /// ```
            option, rename = try_flex_direction
        ))]
    pub flex_direction: Option<flexbox::Direction>,
    #[rich(
        write(
            /// ```
            /// style.order(3);
            /// ```
        ),
        write(
            /// ```
            /// fn get_order() -> Option<Order> { .. }
            ///
            /// style.try_order(get_order());
            /// ```
            option, rename = try_order
        ))]
    pub order: Option<flexbox::Order>,
    #[rich(
        write(
            /// ```
            /// style.flex_grow(2.0);
            /// ```
        ),
        write(
            /// ```
            /// fn get_grow() -> Option<Order> { .. }
            ///
            /// style.try_flex_grow(get_grow());
            /// ```
            option, rename = try_flex_grow
        ))]
    pub flex_grow: Option<flexbox::Grow>,
    #[rich(
        write(
            /// ```
            /// style.flex_shrink(4.0);
            /// ```
        ),
        write(
            /// ```
            /// fn get_shrink() -> Option<Shrink> { .. }
            ///
            /// style.try_flex_shrink(get_shrink());
            /// ```
            option, rename = try_flex_shrink
        ))]
    pub flex_shrink: Option<flexbox::Shrink>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.justify_content(val::Center)
            /// ```
        ),
        write(
            /// ```
            /// fn get_justify_content() -> Option<JustifyContent> { .. }
            ///
            /// style.try_justify_content(get_justify_content());
            /// ```
            option, rename = try_justify_content
        ))]
    pub justify_content: Option<box_align::JustifyContent>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.align_content(val::Stretch)
            /// ```
        ),
        write(
            /// ```
            /// fn get_align_content() -> Option<AlignContent> { .. }
            ///
            /// style.try_align_content(get_align_content());
            /// ```
            option, rename = try_align_content
        ))]
    pub align_content: Option<box_align::AlignContent>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.align_items(val::Stretch);
            /// ```
        ),
        write(
            /// ```
            /// fn get_align_items() -> Option<AlignItems> { .. }
            ///
            /// style.try_align_items(get_align_items());
            /// ```
            option, rename = try_align_items
        ))]
    pub align_items: Option<box_align::AlignItems>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.justify_self(val::Stretch);
            /// ```
        ),
        write(
            /// ```
            /// fn get_justify_self() -> Option<JustifySelf> { .. }
            ///
            /// style.try_justify_self(get_justify_self());
            /// ```
            option, rename = try_justify_self
        ))]
    pub justify_self: Option<box_align::JustifySelf>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.align_self(val::Stretch);
            /// ```
        ),
        write(
            /// ```
            /// fn get_align_self() -> Option<AlignSelf> { .. }
            ///
            /// style.try_align_self(get_align_self());
            /// ```
            option, rename = try_align_self
        ))]
    pub align_self: Option<box_align::AlignSelf>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.display(val::Flex);
            /// ```
        ),
        write(
            /// ```
            /// fn get_display() -> Option<css::Display> { .. }
            ///
            /// style.try_display(get_display());
            /// ```
            option, rename = try_display
        ))]
    pub display: Option<Display>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.visibility(val::Hidden);
            /// ```
        ),
        write(
            /// ```
            /// fn get_visibility() -> Option<Visibility> { .. }
            ///
            /// style.try_visibility(get_visibility());
            /// ```
            option, rename = try_visibility
        ))]
    pub visibility: Option<Visibility>,
    #[rich(
        write(
            /// ```
            /// use khalas::css::values as val;
            ///
            /// style.cursor(val::Progress);
            /// ```
        ),
        write(
            /// ```
            /// fn get_cursor() -> Option<Cursor> { .. }
            ///
            /// style.try_cursor(get_cursor());
            /// ```
            option, rename = try_cursor
        ))]
    pub cursor: Option<Cursor>,
    #[rich(
        write(
            /// background properties can be add and configured using this method
            ///
            /// ```
            /// style.and_background(|conf| {
            ///         conf.image("/bg/fullpage.png")
            ///             .scroll()
            ///     });
            /// ```
            style = compose, rename = and_background
        ),
        write(
            /// ```
            /// fn get_background() -> Background { .. }
            ///
            /// style.background(get_background());
            /// ```
        ))]
    pub background: Option<Background>,
    #[rich(
        write(
            /// border properties can be add and configured using this method
            ///
            /// ```
            /// use khalas::css::{values as val, unit::px, Color};
            ///
            /// style
            ///     .and_border(|conf| {
            ///         conf.solid() // or .style(val::Solid)
            ///             .width(px(2.))
            ///             .color(Color::DimGray)
            ///             .radius(px(4.))
            ///     });
            /// ```
            style = compose, rename = and_border
        ),
        write(
            /// ```
            /// fn get_border() -> Border { .. }
            ///
            /// style.border(get_border());
            /// ```
        ))]
    pub border: Option<Border>,
    #[rich(
        write(
            /// margin properties can be add and manipulated using this method
            ///
            /// ```
            /// use khalas::css::{values as val, unit::px};
            ///
            /// style
            ///     .and_margin(|conf| {
            ///         conf.x(val::Auto)
            ///             .y(px(4.))
            ///     })
            /// ]
            /// ```
            style = compose, rename = and_margin
        ),
        write(
            /// ```
            /// fn get_margin() -> Margin { .. }
            ///
            /// style.margin(get_margin());
            /// ```
        ))]
    pub margin: Option<Margin>,
    #[rich(
        write(
            /// padding properties can be add and configured using this method
            ///
            /// ```
            /// use khalas::css::{values as val, unit::px};
            ///
            /// style
            ///     .and_padding(|conf| {
            ///         conf.x(val::Auto)
            ///             .y(px(4.))
            ///     });
            /// ```
            style = compose, rename = and_padding
        ),
        write(
            /// ```
            /// fn get_padding() -> Padding { .. }
            ///
            /// style.padding(get_padding());
            /// ```
        ))]
    pub padding: Option<Padding>,
    #[rich(
        write(
            /// size properties can be add and configured using this method
            ///
            /// ```
            /// use khalas::css::{unit::em};
            ///
            /// style
            ///     .and_size(|conf| {
            ///         conf.width(em(2.))
            ///             .height(em(1.5))
            ///             .min_width(em(1.5))
            ///             .min_height(em(1.))
            ///             .max_width(em(4.))
            ///             .max_height(em(3.))
            ///     });
            /// ```
            style = compose, rename = and_size
        ),
        write(
            /// ```
            /// fn get_size() -> Size { .. }
            ///
            /// style.size(get_size());
            /// ```
        ))]
    pub size: Option<Size>,
    #[rich(
        write(
            /// transition properties can be add and configured using this method
            ///
            /// ```
            /// use khalas::css::{values as val, unit::{sec, ms}};
            ///
            /// style
            ///     .and_transition(|conf| {
            ///         conf
            ///             // transition for all properties
            ///             .all(|conf| {
            ///                 conf.duration(sec(0.3))
            ///                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
            ///             })
            ///             // or transition for specific properties (e.g. opacity only)
            ///             .add("opacity", |conf| {
            ///                 conf.duration(ms(150.))
            ///                     .ease()
            ///                     .delay(sec(0.5))
            ///             })
            ///         });
            /// ```
            style = compose, rename = and_transition
        ),
        write(
            /// ```
            /// fn get_transition() -> Transition { .. }
            ///
            /// style.transition(get_transition());
            /// ```
        ))]
    pub transition: Option<Transition>,
    #[rich(
        write(
            /// position properties can be add and configured using this method
            ///
            /// ```
            /// use khalas::css::unit::px;
            ///
            /// style
            ///     .and_position(|conf| {
            ///         conf.absolute().top(px(top)).left(px(left))
            ///     })
            /// ```
            style = compose, rename = and_position
        ),
        write(
            /// ```
            /// fn get_position() -> Position { .. }
            ///
            /// style.position(get_position());
            /// ```
        ))]
    pub position: Option<Position>,
    #[rich(
        write(
            /// text properties can be add and configured using this method
            ///
            /// ```
            /// use khalas::css::{values as val, Color, unit::em};
            /// use palette::Rgb;
            ///
            /// style
            ///     .and_text(|conf| {
            ///         conf.line_height(1.7)
            ///             // we can pass Rgb, Rgba, Hsl, Hsla
            ///             .color(Rgb::new(0.5, 0.1, 0.1))
            ///             // or we can use html colors
            ///             .color(color::BlueViolet)
            ///             .align(val::Center)
            ///             .transform(val::Capitalize)
            ///             .indent(em(2.))
            ///     })
            /// ```
            style = compose, rename = and_text
        ),
        write(
            /// ```
            /// fn get_text() -> Text { .. }
            ///
            /// style.text(get_text());
            /// ```
        ))]
    pub text: Option<Text>,
    #[rich(write(
        /// Here goes other css properties those doesn't have their own method
        /// (.e.g custom css properties), css values are stored as `String`, so
        /// you won't get typed values like the ones that have it's own methods.
        ///
        /// ```
        /// use khalas::css::{St, Color, unit::em};
        ///
        /// fn get_color() -> Option<Color> { .. }
        ///
        /// style
        ///     .and_others(|conf| {
        ///         conf.try_add(St::from("--box-bg-color"), get_color())
        ///             .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
        ///             .add(St::from("--container-gap"), em(2.))
        ///     })
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
    /// use khalas::css::{values as val, Style};
    ///
    /// style.config_block(|conf| {
    ///     if popover.is_visible() {
    ///         conf.opacity(1.).visibility(val::Visible)
    ///     } else {
    ///         conf.visibility(val::Hidden).opacity(0.)
    ///     }
    /// });
    /// ```
    pub fn config_block(&mut self, block: impl FnOnce(&mut Self) -> &mut Self) -> &mut Self {
        block(self)
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

impl<Msg> UpdateEl<El<Msg>> for Style {
    fn update(self, el: &mut El<Msg>) {
        if let Some(style) = self.to_seed_style() {
            style.update(el);
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
    /// use khalas::css::{Style, values as val, St};
    ///
    /// div![
    ///     Style::default()
    ///         .add(St::UserSelect, val::None)
    ///         .add(St::BoxSizing, val::BorderBox)
    /// ]
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
