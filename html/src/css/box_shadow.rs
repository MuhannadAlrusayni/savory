use crate::css::{color::Color, unit::*, values as val, St, StyleValues, UpdateStyleValues};
use derive_rich::Rich;

/// ```
/// use savory_html::css::{values as val, Style, Color, unit::em};
/// use palette::rgb::Rgb;
///
/// Style::default()
///     // for single shadow
///     .and_box_shadow(|conf| {
///         conf.x(em(0.4))
///             .y(em(-0.8))
///             // we can pass Rgb, Rgba, Hsl, Hsla
///             .color(Rgb::new(0.5, 0.1, 0.1))
///             // or we can use HTML colors
///             .color(Color::BlueViolet)
///             // shadow blur radius
///             .blur(em(1.0))
///             // spread radius
///             .spread(em(2.0))
///             // inset shadow
///             .inset()
///     })
///     // for multiple shadows
///     .and_box_shadow(|conf| {
///         conf
///             .add(|conf| {
///                 conf.x(em(1.0))
///                     .y(em(2.0))
///                     .color(Color::DimGray)
///             })
///             .add(|conf| {
///                 conf.x(em(-2.0))
///                     .y(em(-4.0))
///                     .color(Color::Red)
///             })
///     });
/// ```
#[derive(Clone, Debug, PartialEq, From)]
pub enum BoxShadow {
    One(ShadowValue),
    Multiple(Vec<ShadowValue>),
    Initial(val::Initial),
    Inherit(val::Inherit),
    None(val::None),
    Unset(val::Unset),
}

impl Default for BoxShadow {
    fn default() -> Self {
        BoxShadow::Multiple(vec![])
    }
}

impl UpdateStyleValues for BoxShadow {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        let to_string = |shadow: ShadowValue| {
            let mut vals = vec![];

            if shadow.inset {
                vals.push("inset".to_string());
            }

            vals.push(shadow.x.to_string());
            vals.push(shadow.y.to_string());

            match (shadow.blur, shadow.spread) {
                (Some(blur), Some(spread)) => {
                    vals.push(blur.to_string());
                    vals.push(spread.to_string());
                }
                (Some(blur), None) => {
                    vals.push(blur.to_string());
                }
                // if there was no blur specified then use 0px
                (None, Some(spread)) => {
                    vals.push(px(0).to_string());
                    vals.push(spread.to_string());
                }
                (None, None) => {}
            };

            if let Some(color) = shadow.color {
                vals.push(color.to_string());
            }

            vals.join(" ")
        };

        let val = match self {
            Self::Initial(val) => val.to_string(),
            Self::Inherit(val) => val.to_string(),
            Self::None(val) => val.to_string(),
            Self::Unset(val) => val.to_string(),
            Self::One(shadow) => to_string(shadow),
            Self::Multiple(vec) => {
                let val = vec
                    .into_iter()
                    .map(to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                // if no shadow value added we return values without any updates
                if val.is_empty() {
                    return values;
                }
                val
            }
        };

        values.add(St::BoxShadow, val)
    }
}

impl BoxShadow {
    fn shadow(mut self, conf: impl FnOnce(ShadowValue) -> ShadowValue) -> Self {
        self = match self {
            Self::One(shadow) => Self::One(conf(shadow)),
            Self::Multiple(shadows) => Self::One(conf(
                shadows
                    .into_iter()
                    .next()
                    .unwrap_or_else(ShadowValue::default),
            )),
            _ => Self::One(conf(ShadowValue::default())),
        };
        self
    }

    pub fn new() -> Self {
        BoxShadow::Multiple(vec![])
    }

    pub fn x(self, val: impl Into<Length>) -> Self {
        self.shadow(|sh| sh.x(val))
    }

    pub fn y(self, val: impl Into<Length>) -> Self {
        self.shadow(|sh| sh.y(val))
    }

    pub fn blur(self, val: impl Into<Length>) -> Self {
        self.shadow(|sh| sh.blur(val))
    }

    pub fn try_blur(self, val: Option<impl Into<Length>>) -> Self {
        self.shadow(|sh| sh.try_blur(val))
    }

    pub fn spread(self, val: impl Into<Length>) -> Self {
        self.shadow(|sh| sh.spread(val))
    }

    pub fn try_spread(self, val: Option<impl Into<Length>>) -> Self {
        self.shadow(|sh| sh.try_spread(val))
    }

    pub fn color(self, val: impl Into<Color>) -> Self {
        self.shadow(|sh| sh.color(val))
    }

    pub fn try_color(self, val: Option<impl Into<Color>>) -> Self {
        self.shadow(|sh| sh.try_color(val))
    }

    pub fn inset(self) -> Self {
        self.shadow(|sh| sh.inset())
    }

    pub fn outset(self) -> Self {
        self.shadow(|sh| sh.outset())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, get_val: impl FnOnce(ShadowValue) -> ShadowValue) -> Self {
        let val = get_val(ShadowValue::default());
        self = match self {
            Self::Multiple(mut vec) => {
                vec.push(val);
                Self::Multiple(vec)
            }
            _ => Self::Multiple(vec![val]),
        };
        self
    }
}

#[derive(Rich, Clone, Debug, PartialEq)]
pub struct ShadowValue {
    #[rich(write(rename = x))]
    x: Length,
    #[rich(write(rename = y))]
    y: Length,
    #[rich(write(rename = blur), write(option, rename = try_blur))]
    blur: Option<Length>,
    #[rich(write(rename = spread), write(option, rename = try_spread))]
    spread: Option<Length>,
    #[rich(write(rename = color), write(option, rename = try_color))]
    color: Option<Color>,
    #[rich(value_fns = { inset = true, outset = false })]
    inset: bool,
}

impl Default for ShadowValue {
    fn default() -> Self {
        Self {
            x: px(0),
            y: px(0),
            blur: None,
            spread: None,
            color: None,
            inset: false,
        }
    }
}
