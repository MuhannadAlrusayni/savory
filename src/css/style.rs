use super::*;
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
/// div![
///     Style::default()
///         .transition(|conf| {
///             conf
///                 .add("opacity", |conf| conf.duration(ms(150.)).ease())
///                 .add("transform", |conf| conf.duration(ms(150.)).ease())
///                 .add("visibility", |conf| conf.duration(ms(150.)).ease())
///         })
///         .position(|conf| conf.absolute())
///         .background(|conf| conf.color(Color::white))
///         .border(|conf| {
///             conf.none()
///                 .width(px(0.))
///                 .radius(px(4.))
///         })
///         .add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)")
///         .padding(|conf| conf.x(px(4.)).y(px(2)))
///         .margin(|conf| conf.top(px(popover.offset)))
///         .config_block(|conf| {
///             if popover.is_visible() {
///                 conf.opacity(1.).visibility(val::Visible)
///             } else {
///                 conf.visibility(val::Hidden).opacity(0.)
///             }
///         });
/// ]
/// ```
#[derive(Default, Debug, Clone)]
pub struct Style(IndexMap<St, String>);

macro css_props( $( $(#[$doc:meta])* $fn_ident:ident($prop_ty:ty) $(,)? )* ) {
    impl Style {
        $(
            $( #[$doc] )*
            pub fn $fn_ident<R: Into<$prop_ty>>(mut self, get_prop: impl Fn($prop_ty) -> R) -> Self {
                let value = get_prop(<$prop_ty>::default()).into();
                self.merge(&value)
            }
        )*
    }
}

macro simple_css_props( $( $(#[$doc:meta])* $fn_ident:ident($prop_ty:ty) $(,)? )* ) {
    impl Style {
        $(
            $( #[$doc] )*
            pub fn $fn_ident(mut self, value: impl Into<$prop_ty>) -> Self {
                self.merge(&value.into())
            }
        )*
    }
}

simple_css_props! {
    /// ```
    /// use khalas::css::Style;
    ///
    /// div![
    ///     Style::default()
    ///         .opacity(0.75)
    /// ]
    /// ```
    opacity(Opacity),
    /// ```
    /// use khalas::css::{Style, unit::px};
    ///
    /// div![
    ///     Style::default()
    ///         .gap(px(2.))
    /// ]
    /// ```
    gap(Gap),
    /// This method accept any type implemente `Into<Wrap>`, so we can
    /// pass `Wrap`, `Nowrap` and `WrapReverse`
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .flex_wrap(val::Wrap)
    /// ]
    /// ```
    flex_wrap(flexbox::Wrap),
    /// This method accept any type implemente `Into<Basis>`, so we can
    /// pass `Auto`, `Content`, `Inherit` or any unit type
    ///
    /// ```
    /// use khalas::css::{Style, values as val, unit::{px, em}};
    ///
    /// div![
    ///     Style::default()
    ///         // pass auto
    ///         .flex_basis(val::Auto)
    ///         // or 4px
    ///         .flex_basis(px(4.))
    ///         // or 80%
    ///         .flex_basis(0.8) // not that f32 get converted to unit::Percent type
    /// ]
    /// ```
    flex_basis(flexbox::Basis),
    /// This method accept any type implemente `Into<Direction>`, so we can pass
    /// `Row`, `RowReverse`, `Column` and `ColumnReverse`.
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .flex_direction(val::Column)
    /// ]
    /// ```
    flex_direction(flexbox::Direction),
    /// ```
    /// use khalas::css::Style;
    ///
    /// div![
    ///     Style::default()
    ///         .order(3)
    /// ]
    /// ```
    order(flexbox::Order),
    /// ```
    /// use khalas::css::Style;
    ///
    /// div![
    ///     Style::default()
    ///         .flex_grow(2.0)
    /// ]
    /// ```
    flex_grow(flexbox::Grow),
    /// ```
    /// use khalas::css::Style;
    ///
    /// div![
    ///     Style::default()
    ///         .flex_shrink(4.0)
    /// ]
    /// ```
    flex_shrink(flexbox::Shrink),
    /// This method accept any type implemente `Into<JustifyContent>`, so we can
    /// pass `Normal`, `SpaceBetween`, `SpaceAround`, `Stretch`, `Center`,
    /// `Start` and many other values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .justify_content(val::Center)
    /// ]
    /// ```
    justify_content(box_align::JustifyContent),
    /// This method accept any type implemente `Into<AlignContent>`, so we can
    /// pass `Normal`, `Baseline`, `FirstBaseline`, `Stretch`, `Center`,
    /// `Start` and many other values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .align_content(val::Stretch)
    /// ]
    /// ```
    align_content(box_align::AlignContent),
    /// This method accept any type implemente `Into<AlignItems>`, so we can
    /// pass `Normal`, `Baseline`, `FirstBaseline`, `Stretch`, `Center`,
    /// `Start` and many other values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .align_items(val::Stretch)
    /// ]
    /// ```
    align_items(box_align::AlignItems),
    /// This method accept any type implemente `Into<JustifySelf>`, so we can
    /// pass `Auto`, `Normal`, `Stretch`, `Center`, `Start` and many other
    /// values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .justify_self(val::Stretch)
    /// ]
    /// ```
    justify_self(box_align::JustifySelf),
    /// This method accept any type implemente `Into<AlignSelf>`, so we can
    /// pass `Auto`, `Normal`, `Stretch`, `Center`, `Start` and many other
    /// values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .align_self(val::Stretch)
    /// ]
    /// ```
    align_self(box_align::AlignSelf),
    /// This method accept any type implemente `Into<css::Display>`, so we can
    /// pass `Flex`, `Grid`, `Block`, `Table`, `None` and many other values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .display(val::Flex)
    /// ]
    /// ```
    display(Display),
    /// This method accept any type implemente `Into<Visibility>`, so we can
    /// pass `Visible`, `Hidden`, `Collapse`, `Initial`, `Inherit`
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .visibility(val::Hidden)
    /// ]
    /// ```
    visibility(Visibility),
    /// This method accept any type implemente `Into<Cursor>`, so we can pass
    /// `Grab`, `Help`, `NoDrop`, `Progress`, `ZoomIn` and many other values
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .cursor(val::Progress)
    /// ]
    /// ```
    cursor(Cursor),
}

css_props! {
    /// background properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, Color};
    ///
    /// div![
    ///     Style::default()
    ///         .background(|conf| {
    ///             conf.image("/bg/fullpage.png")
    ///                 .scroll()
    ///         })
    /// ]
    /// ```
    background(Background),
    /// border properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, values as val, unit::px, Color};
    ///
    /// div![
    ///     Style::default()
    ///         .border(|conf| {
    ///             conf.solid() // or .style(val::Solid)
    ///                 .width(px(2.))
    ///                 .color(Color::DimGray)
    ///                 .radius(px(4.))
    ///         })
    /// ]
    /// ```
    border(Border),
    /// margin properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, values as val, unit::px};
    ///
    /// div![
    ///     Style::default()
    ///         .margin(|conf| {
    ///             conf.x(val::Auto)
    ///                 .y(px(4.))
    ///         })
    /// ]
    /// ```
    margin(Margin),
    /// padding properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, values as val, unit::px};
    ///
    /// div![
    ///     Style::default()
    ///         .padding(|conf| {
    ///             conf.x(val::Auto)
    ///                 .y(px(4.))
    ///         })
    /// ]
    /// ```
    padding(Padding),
    /// size properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, values as val, unit::em};
    ///
    /// div![
    ///     Style::default()
    ///         .size(|conf| {
    ///             conf.width(em(2.))
    ///                 .height(em(1.5))
    ///                 .min_width(em(1.5))
    ///                 .min_height(em(1.))
    ///                 .max_width(em(4.))
    ///                 .max_height(em(3.))
    ///         })
    /// ]
    /// ```
    size(Size),
    /// transition properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, values as val, unit::{sec, ms}};
    ///
    /// // transition for all properties
    /// div![
    ///     Style::default()
    ///         .transition(|conf| {
    ///             conf.all(|conf| {
    ///                 conf.duration(sec(0.3))
    ///                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
    ///             })
    ///         })
    /// ]
    ///
    /// // transition for opacity only
    /// div![
    ///     Style::default()
    ///         .transition(|conf| {
    ///             conf.add("opacity", |conf| {
    ///                 conf.duration(ms(150.))
    ///                     .ease()
    ///                     .delay(sec(0.5))
    ///             })
    ///         })
    /// ]
    /// ```
    transition(Transition),
    /// position properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{unit::sec, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .position(|conf| {
    ///             conf.absolute().top(px(top)).left(px(left))
    ///         })
    /// ]
    /// ```
    position(Position),
    /// text properties can be add and manipulated using this method
    ///
    /// ```
    /// use khalas::css::{Style, values as val, Color, unit::em};
    /// use palette::Rgb;
    ///
    /// div![
    ///     Style::default()
    ///         .text(|conf| {
    ///             conf.line_height(1.7)
    ///                 // we can pass Rgb, Rgba, Hsl, Hsla
    ///                 .color(Rgb::new(0.5, 0.1, 0.1))
    ///                 // or we can use html colors
    ///                 .color(color::BlueViolet)
    ///                 .align(val::Center)
    ///                 .transform(val::Capitalize)
    ///                 .indent(em(2.))
    ///         })
    /// ]
    /// ```
    text(Text),
}

impl Style {
    pub fn new() -> Self {
        Self(IndexMap::default())
    }

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
    pub fn add(mut self, key: impl Into<St>, value: impl ToString) -> Self {
        self.0.insert(key.into(), value.to_string());
        self
    }

    /// This method is similar `add` but it accept an optional value, if the
    /// passed value was `None` then nothing added to the style.
    pub fn try_add(self, key: impl Into<St>, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            self.add(key, value)
        } else {
            self
        }
    }

    /// This method accept any type that implemente
    /// [`ToStyle`](crate::css::ToStyle) and merge it this style.
    pub fn merge(mut self, other: &impl ToStyle) -> Self {
        self.0.extend(other.to_style().0.into_iter());
        self
    }

    /// This method is similar to `merge` but it accept an optional value.
    pub fn try_merge(self, other: Option<&impl ToStyle>) -> Self {
        if let Some(other) = other {
            self.merge(other)
        } else {
            self
        }
    }

    /// This method accept closure that configure the style
    ///
    /// ```
    /// use khalas::css::{values as val, Style};
    ///
    /// div![
    ///     Style::default()
    ///         .config_block(|conf| {
    ///             if popover.is_visible() {
    ///                 conf.opacity(1.).visibility(val::Visible)
    ///             } else {
    ///                 conf.visibility(val::Hidden).opacity(0.)
    ///             }
    ///         });
    /// ]
    /// ```
    pub fn config_block(self, block: impl FnOnce(Self) -> Self) -> Self {
        block(self)
    }

    /// This method convert this style to html style value
    pub fn to_css(&self) -> Option<String> {
        self.0.iter().fold(Option::None, |mut css, (key, value)| {
            *css.get_or_insert(String::default()) += &format!("{}: {};", key.as_str(), value);
            css
        })
    }

    /// this method convert this style to seed [`Style`](seed::virtual_dom::Style)
    pub fn to_seed_style(&self) -> Option<seed::virtual_dom::Style> {
        self.0.iter().fold(Option::None, |mut style, (key, value)| {
            style
                .get_or_insert(seed::virtual_dom::Style::empty())
                .add(key.clone(), value);
            style
        })
    }
}

impl<Msg> UpdateEl<El<Msg>> for Style {
    fn update(self, el: &mut El<Msg>) {
        if let Some(style) = self.to_seed_style() {
            // style.update(el);
            el.style.merge(style);
        }
    }
}

/// Any type that act like css property should implemente this trait, thus it
/// can be added to [`Style`](crate::css::Style) using `merge` and `try_merge`
/// methods.
pub trait ToStyle {
    fn to_style(&self) -> Style;
}

impl ToStyle for Style {
    fn to_style(&self) -> Style {
        self.clone()
    }
}
