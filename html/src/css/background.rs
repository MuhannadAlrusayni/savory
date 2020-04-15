use crate::css::{color::Color, unit::*, values as val, St, StyleValues, UpdateStyleValues};
use derive_rich::Rich;

/// ```
/// use savory_html::css::{values as val, Style, Color, unit::em};
///
/// Style::default()
///     .and_background(|conf| {
///         conf.image("/bg/fullpage.png")
///             .color(Color::White)
///             .scroll()
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Background {
    #[rich(write(rename = color))]
    pub color: Option<Color>,
    // TODO: support multiple images
    #[rich(write(rename = image), value_fns = { empty = val::None })]
    pub image: Option<Image>,
    #[rich(value_fns = {
        repeat_x = val::RepeatX,
        repeat_y = val::RepeatY,
        repeat = val::Repeat,
        repeat_with_space = val::Space,
        repeat_round = val::Round,
        no_repeat = val::NoRepeat,
        initial_repeat = val::Initial,
        inherit_repeat = val::Inherit,
    })]
    pub repeat: Option<Repeat>,
    #[rich(write(rename = attachment), value_fns = {
        scroll = val::Scroll,
        fixed = val::Fixed,
        local = val::Local,
        initial_attachment = val::Initial,
        inherit_attachment = val::Inherit,
    })]
    pub attachment: Option<Attachment>,
    #[rich(write(rename = position), value_fns = {
        left_top = (Horizontal::from(val::Left), val::Top.into()),
        center_top = (Horizontal::from(val::Center), val::Top.into()),
        right_top = (Horizontal::from(val::Right), val::Top.into()),
        left_center = (Horizontal::from(val::Left), val::Center.into()),
        center = (Horizontal::from(val::Center), val::Center.into()),
        right_center = (Horizontal::from(val::Right), val::Center.into()),
        left_bottom = (Horizontal::from(val::Left), val::Bottom.into()),
        center_bottom = (Horizontal::from(val::Center), val::Bottom.into()),
        right_bottom = (Horizontal::from(val::Right), val::Bottom.into()),
    })]
    pub position: Option<Position>,
    #[rich(write(rename = clip), value_fns = {
        fill_under_border = val::BorderBox,
        fill_inside_border = val::PaddingBox,
        fill_under_content = val::ContentBox,
    })]
    pub clip: Option<Clip>,
    #[rich(write(rename = origin), value_fns = {
        image_fill_under_border = val::BorderBox,
        image_inside_border = val::PaddingBox,
        image_under_content = val::ContentBox,
    })]
    pub origin: Option<Origin>,
    #[rich(write(rename = size), value_fns = {
        full = (1.0, 1.0),
        half = (0.5, 0.5),
        quarter = (0.25, 0.25),
        auto_size = val::Auto,
        cover = val::Cover,
        contain = val::Contain,
    })]
    pub size: Option<Size>,
}

impl<T: Into<Color>> From<T> for Background {
    fn from(source: T) -> Self {
        Background::default().color(source.into())
    }
}

impl UpdateStyleValues for Background {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
            .try_add(St::BackgroundColor, self.color)
            .try_add(St::BackgroundImage, self.image.as_ref())
            .try_add(St::BackgroundRepeat, self.repeat)
            .try_add(St::BackgroundAttachment, self.attachment)
            .try_add(St::BackgroundPosition, self.position)
    }
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Image {
    #[from]
    None(val::None),
    #[display(fmt = "url({})", _0)]
    #[from(forward)]
    Url(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Repeat {
    RepeatX(val::RepeatX),
    RepeatY(val::RepeatY),
    Repeat(val::Repeat),
    Space(val::Space),
    Round(val::Round),
    NoRepeat(val::NoRepeat),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Attachment {
    Scroll(val::Scroll),
    Fixed(val::Fixed),
    Local(val::Local),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

fn display_helper(v: &Option<impl ToString>) -> String {
    v.as_ref()
        .map_or("".into(), |s| format!(" {}", s.to_string()))
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Horizontal {
    #[from]
    Center(val::Center),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Left(val::Left, Option<Length>),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Right(val::Right, Option<Length>),
}

impl From<val::Left> for Horizontal {
    fn from(source: val::Left) -> Self {
        Self::Left(source, None)
    }
}

impl From<val::Right> for Horizontal {
    fn from(source: val::Right) -> Self {
        Self::Right(source, None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Vertical {
    #[from]
    Center(val::Center),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Top(val::Top, Option<Length>),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Bottom(val::Bottom, Option<Length>),
}

impl From<val::Top> for Vertical {
    fn from(source: val::Top) -> Self {
        Self::Top(source, None)
    }
}

impl From<val::Bottom> for Vertical {
    fn from(source: val::Bottom) -> Self {
        Self::Bottom(source, None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Position {
    #[display(fmt = "{} {}", _0, _1)]
    #[from]
    Percent(Percent, Percent),
    #[display(fmt = "{} {}", _0, _1)]
    #[from]
    Placement(Horizontal, Vertical),
}

impl From<(f32, f32)> for Position {
    fn from((hor, ver): (f32, f32)) -> Self {
        Position::Percent(hor.into(), ver.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Box {
    BorderBox(val::BorderBox),
    PaddingBox(val::PaddingBox),
    ContentBox(val::ContentBox),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

pub type Clip = Box;
pub type Origin = Box;

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Size {
    #[display(fmt = "{} {}", _0, _1)]
    WidthHeight(LengthPercent, LengthPercent),
    Auto(val::Auto),
    Cover(val::Cover),
    Contain(val::Contain),
    Initial(val::Initial),
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self::WidthHeight(width.into(), height.into())
    }
}
