use crate::css::{color::Color, unit::*, values as val, St, StyleValues, UpdateStyleValues};
use derive_rich::Rich;
use std::borrow::Cow;

/// ```
/// use savory_html::css::{values as val, Style, Color, unit::{em, px}};
/// use palette::rgb::Rgb;
///
/// Style::default()
///     .and_text(|conf| {
///         conf.line_height(1.7)
///             // we can pass Rgb, Rgba, Hsl, Hsla
///             .color(Rgb::new(0.5, 0.1, 0.1))
///             // or we can use HTML colors
///             .color(Color::BlueViolet)
///             .align(val::Center)
///             .transform(val::Capitalize)
///             .indent(em(2.))
///             // for single text shadow
///             .and_shadow(|conf| {
///                 conf.x(px(3))
///                     .y(px(4))
///                     .color(Color::Blue)
///                     .blur(px(2))
///             })
///             // for multiple text shadows
///             .and_shadow(|conf| {
///                 conf.add(|conf| conf.x(px(2))).y(px(-4))
///                     .add(|conf| conf.x(px(9)))
///             })
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Text {
    #[rich(write(rename = color), write(option, rename = try_color))]
    pub color: Option<Color>,
    #[rich(write(rename = direction), write(option, rename = try_direction))]
    pub direction: Option<Direction>,
    #[rich(write(rename = letter_spacing), write(option, rename = try_letter_spacing))]
    pub letter_spacing: Option<LetterSpacing>,
    #[rich(write(rename = word_spacing), write(option, rename = try_word_spacing))]
    pub word_spacing: Option<WordSpacing>,
    #[rich(write(rename = line_height), write(option, rename = try_line_height))]
    pub line_height: Option<LineHeight>,
    #[rich(write(rename = align), write(option, rename = try_align))]
    pub align: Option<TextAlign>,
    #[rich(write(rename = align_last), write(option, rename = try_align_last))]
    pub align_last: Option<TextAlignLast>,
    #[rich(write(rename = justify), write(option, rename = try_justify))]
    pub justify: Option<TextJustify>,
    #[rich(write(rename = shadow), write(option, rename = try_shadow), write(style = compose))]
    pub shadow: Option<TextShadow>,
    #[rich(write(rename = indent), write(option, rename = try_indent))]
    pub indent: Option<TextIndent>,
    #[rich(write(rename = decoration), write(option, rename = try_decoration), write(style = compose))]
    pub decoration: Option<TextDecoration>,
    #[rich(write(rename = white_space), write(option, rename = try_white_space))]
    pub white_space: Option<WhiteSpace>,
    #[rich(write(rename = unicode_bidi), write(option, rename = try_unicode_bidi))]
    pub unicode_bidi: Option<UnicodeBidi>,
    #[rich(write(rename = transform), write(option, rename = try_transform))]
    pub transform: Option<TextTransform>,
    #[rich(write(rename = overflow), write(option, rename = try_overflow))]
    pub overflow: Option<TextOverflow>,
    #[rich(write(rename = vertical_align), write(option, rename = try_vertical_align))]
    pub vertical_align: Option<VerticalAlign>,
    #[rich(write(rename = writing_mode), write(option, rename = try_writing_mode))]
    pub writing_mode: Option<WritingMode>,
    #[rich(write(rename = word_wrap), write(option, rename = try_word_wrap))]
    pub word_wrap: Option<WordWrap>,
    #[rich(write(rename = word_break), write(option, rename = try_word_break))]
    pub word_break: Option<WordBreak>,
}

impl UpdateStyleValues for Text {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
            .try_add(St::Color, self.color)
            .try_add(St::Direction, self.direction)
            .try_add(St::LetterSpacing, self.letter_spacing)
            .try_add(St::LineHeight, self.line_height)
            .try_add(St::TextAlign, self.align)
            .try_add(St::TextDecoration, self.decoration.clone())
            .try_add(St::TextIndent, self.indent)
            .try_merge(self.shadow)
            .try_add(St::TextTransform, self.transform)
            .try_add(St::TextOverflow, self.overflow.clone())
            .try_add(St::UnicodeBidi, self.unicode_bidi)
            .try_add(St::VerticalAlign, self.vertical_align)
            .try_add(St::WhiteSpace, self.white_space)
            .try_add(St::WordSpacing, self.word_spacing)
    }
}

impl<T: Into<Color>> From<T> for Text {
    fn from(source: T) -> Self {
        Self::default().color(source.into())
    }
}

#[derive(Clone, Debug, PartialEq, From)]
pub enum TextShadow {
    One(Shadow),
    Multiple(Vec<Shadow>),
    Initial(val::Initial),
    Inherit(val::Inherit),
    None(val::None),
    Unset(val::Unset),
}

impl UpdateStyleValues for TextShadow {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        let to_string = |shadow: Shadow| {
            let mut vals = vec![];

            vals.push(shadow.x.to_string());
            vals.push(shadow.y.to_string());

            if let Some(blur) = shadow.blur {
                vals.push(blur.to_string());
            }

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

        values.add(St::TextShadow, val)
    }
}

impl Default for TextShadow {
    fn default() -> Self {
        TextShadow::Multiple(vec![])
    }
}

impl TextShadow {
    fn shadow(mut self, conf: impl FnOnce(Shadow) -> Shadow) -> Self {
        self = match self {
            Self::One(shadow) => Self::One(conf(shadow)),
            Self::Multiple(shadows) => Self::One(conf(
                shadows.into_iter().next().unwrap_or_else(Shadow::default),
            )),
            _ => Self::One(conf(Shadow::default())),
        };
        self
    }

    pub fn new() -> Self {
        TextShadow::default()
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

    pub fn color(self, val: impl Into<Color>) -> Self {
        self.shadow(|sh| sh.color(val))
    }

    pub fn try_color(self, val: Option<impl Into<Color>>) -> Self {
        self.shadow(|sh| sh.try_color(val))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, get_val: impl FnOnce(Shadow) -> Shadow) -> Self {
        let val = get_val(Shadow::default());
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
pub struct Shadow {
    #[rich(write(rename = x))]
    x: Length,
    #[rich(write(rename = y))]
    y: Length,
    #[rich(write(rename = blur), write(option, rename = try_blur))]
    blur: Option<Length>,
    #[rich(write(rename = color), write(option, rename = try_color))]
    color: Option<Color>,
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            x: px(0),
            y: px(0),
            blur: None,
            color: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Direction {
    Ltr(val::Ltr),
    Rtl(val::Rtl),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Spacing {
    Normal(val::Normal),
    Length(Length),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

pub type LetterSpacing = Spacing;
pub type WordSpacing = Spacing;

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum LineHeight {
    Normal(val::Normal),
    Number(f32),
    Length(Length),
    Percent(Percent),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextAlign {
    Left(val::Left),
    Right(val::Right),
    Center(val::Center),
    Justify(val::Justify),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

fn display_helper(value: &Option<impl ToString>) -> String {
    value
        .as_ref()
        .map(|v| v.to_string() + " ")
        .unwrap_or_else(|| "".into())
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextDecoration {
    #[display(
        fmt = "{}{}{}",
        "display_helper(line)",
        "display_helper(color)",
        "display_helper(style).trim()"
    )]
    Decoration {
        // TODO: add support for multiple unique values
        line: Option<TextDecorationLine>,
        color: Option<TextDecorationColor>,
        style: Option<TextDecorationStyle>,
    },
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

impl Default for TextDecoration {
    fn default() -> Self {
        val::Initial.into()
    }
}

impl TextDecoration {
    pub fn line(mut self, value: impl Into<TextDecorationLine>) -> Self {
        match self {
            Self::Decoration { ref mut line, .. } => *line = Some(value.into()),
            _ => {
                self = Self::Decoration {
                    line: Some(value.into()),
                    color: None,
                    style: None,
                }
            }
        };
        self
    }

    pub fn color(mut self, value: impl Into<TextDecorationColor>) -> Self {
        match self {
            Self::Decoration { ref mut color, .. } => *color = Some(value.into()),
            _ => {
                self = Self::Decoration {
                    line: Some(val::None.into()),
                    color: Some(value.into()),
                    style: None,
                }
            }
        };
        self
    }

    pub fn style(mut self, value: impl Into<TextDecorationStyle>) -> Self {
        match self {
            Self::Decoration { ref mut style, .. } => *style = Some(value.into()),
            _ => {
                self = Self::Decoration {
                    line: Some(val::None.into()),
                    color: None,
                    style: Some(value.into()),
                }
            }
        };
        self
    }

    // TODO: add shortcute functions none(), solid() ..etc
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextDecorationLine {
    None(val::None),
    Underline(val::Underline),
    Overline(val::Overline),
    LineThrough(val::LineThrough),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextDecorationColor {
    Color(Color),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextDecorationStyle {
    Solid(val::Solid),
    Double(val::Double),
    Dotted(val::Dotted),
    Dashed(val::Dashed),
    Wavy(val::Wavy),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextIndent {
    Length(Length),
    Percent(Percent),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextTransform {
    None(val::None),
    Capitalize(val::Capitalize),
    Uppercase(val::Uppercase),
    Lowercase(val::Lowercase),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum TextOverflow {
    Clip(val::Clip),
    Ellipsis(val::Ellipsis),
    String(Cow<'static, str>),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum UnicodeBidi {
    Normal(val::Normal),
    Embed(val::Embed),
    BidiOverride(val::BidiOverride),
    Isolate(val::Isolate),
    IsolateOverride(val::IsolateOverride),
    Plaintext(val::Plaintext),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum VerticalAlign {
    Baseline(val::Baseline),
    Sub(val::Sub),
    Super(val::Super),
    Top(val::Top),
    TextTop(val::TextTop),
    Middle(val::Middle),
    Bottom(val::Bottom),
    TextBottom(val::TextBottom),
    Length(Length),
    Percent(Percent),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WhiteSpace {
    Normal(val::Normal),
    Nowrap(val::Nowrap),
    Pre(val::Pre),
    PreLine(val::PreLine),
    PreWrap(val::PreWrap),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextAlignLast {
    Auto(val::Auto),
    Left(val::Left),
    Right(val::Right),
    Center(val::Center),
    Justify(val::Justify),
    Start(val::Start),
    End(val::End),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextJustify {
    Auto(val::Auto),
    InterWord(val::InterWord),
    InterCharacter(val::InterCharacter),
    None(val::None),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WordBreak {
    Normal(val::Normal),
    BreakAll(val::BreakAll),
    KeepAll(val::KeepAll),
    BreakWord(val::BreakWord),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WordWrap {
    Normal(val::Normal),
    BreakWord(val::BreakWord),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WritingMode {
    HorizontalTb(val::HorizontalTb),
    VerticalRl(val::VerticalRl),
    VerticalLr(val::VerticalLr),
}
