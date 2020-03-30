use crate::css::{color::Color, unit::*, values as val, St, StyleMap, ToStyleMap};
use derive_rich::Rich;
use std::borrow::Cow;

/// ```
/// use khalas::css::{values as val, Style, Color, unit::em};
/// use palette::rgb::Rgb;
///
/// let mut style = Style::default();
/// style
///     .and_text(|conf| {
///         conf.set_line_height(1.7)
///             // we can pass Rgb, Rgba, Hsl, Hsla
///             .set_color(Rgb::new(0.5, 0.1, 0.1))
///             // or we can use HTML colors
///             .set_color(Color::BlueViolet)
///             .set_align(val::Center)
///             .set_transform(val::Capitalize)
///             .set_indent(em(2.))
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Text {
    #[rich(write)]
    pub color: Option<Color>,
    #[rich(write)]
    pub direction: Option<Direction>,
    #[rich(write)]
    pub letter_spacing: Option<LetterSpacing>,
    #[rich(write)]
    pub word_spacing: Option<WordSpacing>,
    #[rich(write)]
    pub line_height: Option<LineHeight>,
    #[rich(write)]
    pub align: Option<TextAlign>,
    #[rich(write)]
    pub align_last: Option<TextAlignLast>,
    #[rich(write)]
    pub justify: Option<TextJustify>,
    // TODO
    // #[rich(write)]
    // pub text_shadow: Option<TextShadow>,
    #[rich(write)]
    pub indent: Option<TextIndent>,
    #[rich(write(style = compose))]
    pub decoration: Option<TextDecoration>,
    #[rich(write)]
    pub white_space: Option<WhiteSpace>,
    #[rich(write)]
    pub unicode_bidi: Option<UnicodeBidi>,
    #[rich(write)]
    pub transform: Option<TextTransform>,
    #[rich(write)]
    pub overflow: Option<TextOverflow>,
    #[rich(write)]
    pub vertical_align: Option<VerticalAlign>,
    #[rich(write)]
    pub writing_mode: Option<WritingMode>,
    #[rich(write)]
    pub word_wrap: Option<WordWrap>,
    #[rich(write)]
    pub word_break: Option<WordBreak>,
}

impl_add_and_add_assign!(Text {
    color
    direction
    letter_spacing
    word_spacing
    line_height
    align
    align_last
    justify
    indent
    decoration
    white_space
    unicode_bidi
    transform
    overflow { clone }
    vertical_align
    writing_mode
    word_wrap
    word_break
});

impl ToStyleMap for Text {
    fn style_map(&self) -> StyleMap {
        StyleMap::default()
            .try_add(St::Color, self.color)
            .try_add(St::Direction, self.direction)
            .try_add(St::LetterSpacing, self.letter_spacing)
            .try_add(St::LineHeight, self.line_height)
            .try_add(St::TextAlign, self.align)
            .try_add(St::TextDecoration, self.decoration.clone())
            .try_add(St::TextIndent, self.indent)
            // .try_add(St::TextShadow, self.shadow)
            .try_add(St::TextTransform, self.transform)
            .try_add(St::TextOverflow, self.overflow.clone())
            .try_add(St::UnicodeBidi, self.unicode_bidi)
            .try_add(St::VerticalAlign, self.vertical_align)
            .try_add(St::WhiteSpace, self.white_space)
            .try_add(St::WordSpacing, self.word_spacing)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Direction {
    #[from]
    Ltr(val::Ltr),
    #[from]
    Rtl(val::Rtl),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Spacing {
    #[from]
    Normal(val::Normal),
    #[from]
    Length(Length),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

pub type LetterSpacing = Spacing;
pub type WordSpacing = Spacing;

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum LineHeight {
    #[from]
    Normal(val::Normal),
    #[from]
    Number(f32),
    #[from]
    Length(Length),
    #[from]
    Percent(Percent),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextAlign {
    #[from]
    Left(val::Left),
    #[from]
    Right(val::Right),
    #[from]
    Center(val::Center),
    #[from]
    Justify(val::Justify),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

fn display_helper(value: &Option<impl ToString>) -> String {
    value
        .as_ref()
        .map(|v| v.to_string() + " ")
        .unwrap_or("".into())
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
    pub fn set_line(mut self, value: impl Into<TextDecorationLine>) -> Self {
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

    pub fn set_color(mut self, value: impl Into<TextDecorationColor>) -> Self {
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

    pub fn set_style(mut self, value: impl Into<TextDecorationStyle>) -> Self {
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
    #[from]
    None(val::None),
    #[from]
    Underline(val::Underline),
    #[from]
    Overline(val::Overline),
    #[from]
    LineThrough(val::LineThrough),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextDecorationColor {
    #[from]
    Color(Color),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextDecorationStyle {
    #[from]
    Solid(val::Solid),
    #[from]
    Double(val::Double),
    #[from]
    Dotted(val::Dotted),
    #[from]
    Dashed(val::Dashed),
    #[from]
    Wavy(val::Wavy),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextIndent {
    #[from]
    Length(Length),
    #[from]
    Percent(Percent),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextTransform {
    #[from]
    None(val::None),
    #[from]
    Capitalize(val::Capitalize),
    #[from]
    Uppercase(val::Uppercase),
    #[from]
    Lowercase(val::Lowercase),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum TextOverflow {
    #[from]
    Clip(val::Clip),
    #[from]
    Ellipsis(val::Ellipsis),
    #[from]
    String(Cow<'static, str>),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum UnicodeBidi {
    #[from]
    Normal(val::Normal),
    #[from]
    Embed(val::Embed),
    #[from]
    BidiOverride(val::BidiOverride),
    #[from]
    Isolate(val::Isolate),
    #[from]
    IsolateOverride(val::IsolateOverride),
    #[from]
    Plaintext(val::Plaintext),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum VerticalAlign {
    #[from]
    Baseline(val::Baseline),
    #[from]
    Sub(val::Sub),
    #[from]
    Super(val::Super),
    #[from]
    Top(val::Top),
    #[from]
    TextTop(val::TextTop),
    #[from]
    Middle(val::Middle),
    #[from]
    Bottom(val::Bottom),
    #[from]
    TextBottom(val::TextBottom),
    #[from]
    Length(Length),
    #[from]
    Percent(Percent),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WhiteSpace {
    #[from]
    Normal(val::Normal),
    #[from]
    Nowrap(val::Nowrap),
    #[from]
    Pre(val::Pre),
    #[from]
    PreLine(val::PreLine),
    #[from]
    PreWrap(val::PreWrap),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextAlignLast {
    #[from]
    Auto(val::Auto),
    #[from]
    Left(val::Left),
    #[from]
    Right(val::Right),
    #[from]
    Center(val::Center),
    #[from]
    Justify(val::Justify),
    #[from]
    Start(val::Start),
    #[from]
    End(val::End),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum TextJustify {
    #[from]
    Auto(val::Auto),
    #[from]
    InterWord(val::InterWord),
    #[from]
    InterCharacter(val::InterCharacter),
    #[from]
    None(val::None),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WordBreak {
    #[from]
    Normal(val::Normal),
    #[from]
    BreakAll(val::BreakAll),
    #[from]
    KeepAll(val::KeepAll),
    #[from]
    BreakWord(val::BreakWord),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WordWrap {
    #[from]
    Normal(val::Normal),
    #[from]
    BreakWord(val::BreakWord),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum WritingMode {
    #[from]
    HorizontalTb(val::HorizontalTb),
    #[from]
    VerticalRl(val::VerticalRl),
    #[from]
    VerticalLr(val::VerticalLr),
}
