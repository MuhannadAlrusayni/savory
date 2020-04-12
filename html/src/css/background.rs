use crate::css::{color::Color, unit::*, values as val, St, StyleMap, ToStyleMap};
use derive_rich::Rich;

/// ```
/// use savory::css::{values as val, Style, Color, unit::em};
///
/// let mut style = Style::default();
/// style
///     .and_background(|conf| {
///         conf.set_image("/bg/fullpage.png")
///             .scroll()
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Background {
    #[rich(read, write)]
    color: Option<Color>,
    // TODO: support multiple images
    #[rich(read, write, value_fns = { empty = val::None })]
    image: Option<Image>,
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
    repeat: Option<Repeat>,
    #[rich(value_fns = {
        scroll = val::Scroll,
        fixed = val::Fixed,
        local = val::Local,
        initial_attachment = val::Initial,
        inherit_attachment = val::Inherit,
    })]
    attachment: Option<Attachment>,
    #[rich(read, write, value_fns = {
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
    position: Option<Position>,
    #[rich(value_fns = {
        fill_under_border = val::BorderBox,
        fill_inside_border = val::PaddingBox,
        fill_under_content = val::ContentBox,
    })]
    clip: Option<Clip>,
    #[rich(value_fns = {
        image_fill_under_border = val::BorderBox,
        image_inside_border = val::PaddingBox,
        image_under_content = val::ContentBox,
    })]
    origin: Option<Origin>,
    #[rich(read, write, value_fns = {
        full = (1.0, 1.0),
        half = (0.5, 0.5),
        quarter = (0.25, 0.25),
        auto_size = val::Auto,
        cover = val::Cover,
        contain = val::Contain,
    })]
    size: Option<Size>,
}

impl_add_and_add_assign!(Background {
    color
    image { clone }
    repeat
    attachment
    position
    clip
    origin
    size
});

impl<T: Into<Color>> From<T> for Background {
    fn from(source: T) -> Self {
        Background::default().set_color(source.into())
    }
}

impl ToStyleMap for Background {
    fn style_map(&self) -> StyleMap {
        StyleMap::default()
            .try_add(St::BackgroundColor, self.color)
            .try_add(St::BackgroundImage, self.image.as_ref())
            .try_add(St::BackgroundRepeat, self.repeat)
            .try_add(St::BackgroundAttachment, self.attachment)
            .try_add(St::BackgroundPosition, self.position)
    }
}

impl Background {
    pub fn transparent(self) -> Self {
        self.set_color(Color::Transparent)
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
    #[from]
    RepeatX(val::RepeatX),
    #[from]
    RepeatY(val::RepeatY),
    #[from]
    Repeat(val::Repeat),
    #[from]
    Space(val::Space),
    #[from]
    Round(val::Round),
    #[from]
    NoRepeat(val::NoRepeat),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Attachment {
    #[from]
    Scroll(val::Scroll),
    #[from]
    Fixed(val::Fixed),
    #[from]
    Local(val::Local),
    #[from]
    Initial(val::Initial),
    #[from]
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
    #[from]
    BorderBox(val::BorderBox),
    #[from]
    PaddingBox(val::PaddingBox),
    #[from]
    ContentBox(val::ContentBox),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

pub type Clip = Box;
pub type Origin = Box;

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Size {
    #[from]
    #[display(fmt = "{} {}", _0, _1)]
    WidthHeight(LengthPercent, LengthPercent),
    #[from]
    Auto(val::Auto),
    #[from]
    Cover(val::Cover),
    #[from]
    Contain(val::Contain),
    #[from]
    Initial(val::Initial),
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self::WidthHeight(width.into(), height.into())
    }
}
