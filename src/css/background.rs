use crate::css::{St, self, color::Color, unit::*, ToStyle};
use derive_rich::Rich;

// TODO: use css types in this module enums

#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Background {
    #[rich(read, write(take))]
    color: Option<Color>,
    // TODO: support multiple images
    #[rich(read, write(take), value_fns(take) = { empty = css::None })]
    image: Option<Image>,
    #[rich(value_fns(take) = {
        repeat_x = css::RepeatX,
        repeat_y = css::RepeatY,
        repeat = css::Repeat,
        repeat_with_space = css::Space,
        repeat_round = css::Round,
        no_repeat = css::NoRepeat,
        initial_repeat = css::Initial,
        inherit_repeat = css::Inherit,
    })]
    repeat: Option<Repeat>,
    #[rich(value_fns(take) = {
        scroll = css::Scroll,
        fixed = css::Fixed,
        local = css::Local,
        initial_attachment = css::Initial,
        inherit_attachment = css::Inherit,
    })]
    attachment: Option<Attachment>,
    #[rich(read, write(take), value_fns(take) = {
        left_top = (Horizontal::from(css::Left), css::Top.into()),
        center_top = (Horizontal::from(css::Center), css::Top.into()),
        right_top = (Horizontal::from(css::Right), css::Top.into()),
        left_center = (Horizontal::from(css::Left), css::Center.into()),
        center = (Horizontal::from(css::Center), css::Center.into()),
        right_center = (Horizontal::from(css::Right), css::Center.into()),
        left_bottom = (Horizontal::from(css::Left), css::Bottom.into()),
        center_bottom = (Horizontal::from(css::Center), css::Bottom.into()),
        right_bottom = (Horizontal::from(css::Right), css::Bottom.into()),
    })]
    position: Option<Position>,
    #[rich(value_fns(take) = {
        fill_under_border = css::BorderBox,
        fill_inside_border = css::PaddingBox,
        fill_under_content = css::ContentBox,
    })]
    clip: Option<Clip>,
    #[rich(value_fns(take) = {
        image_fill_under_border = css::BorderBox,
        image_inside_border = css::PaddingBox,
        image_under_content = css::ContentBox,
    })]
    origin: Option<Origin>,
    #[rich(read, write(take), value_fns(take) = {
        full = (1.0, 1.0),
        half = (0.5, 0.5),
        quarter = (0.25, 0.25),
        auto_size = css::Auto,
        cover = css::Cover,
        contain = css::Contain,
    })]
    size: Option<Size>,
}

impl ToStyle for Background {
    fn to_style(&self) -> css::Style {
        css::Style::new()
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
    None(css::None),
    #[display(fmt = "url({})", _0)]
    #[from(forward)]
    Url(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Repeat {
    #[from]
    RepeatX(css::RepeatX),
    #[from]
    RepeatY(css::RepeatY),
    #[from]
    Repeat(css::Repeat),
    #[from]
    Space(css::Space),
    #[from]
    Round(css::Round),
    #[from]
    NoRepeat(css::NoRepeat),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Attachment {
    #[from]
    Scroll(css::Scroll),
    #[from]
    Fixed(css::Fixed),
    #[from]
    Local(css::Local),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Em(Em),
    #[from]
    Ex(Ex),
    #[from]
    Cap(Cap),
    #[from]
    Ch(Ch),
    #[from]
    Ic(Ic),
    #[from]
    Rem(Rem),
    #[from]
    Rlh(Rlh),
    #[from]
    Vm(Vm),
    #[from]
    Vh(Vh),
    #[from]
    Vi(Vi),
    #[from]
    Vb(Vb),
    #[from]
    Vmin(Vmin),
    #[from]
    Vmax(Vmax),
    #[from]
    Cm(Cm),
    #[from]
    Mm(Mm),
    #[from]
    Q(Q),
    #[from]
    In(In),
    #[from]
    Pc(Pc),
    #[from]
    Pt(Pt),
    #[from]
    Px(Px),
}

fn display_helper(v: &Option<impl ToString>) -> String {
    v.as_ref()
        .map_or("".into(), |s| format!(" {}", s.to_string()))
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Horizontal {
    #[from]
    Center(css::Center),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Left(css::Left, Option<Length>),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Right(css::Right, Option<Length>),
}

impl From<css::Left> for Horizontal {
    fn from(source: css::Left) -> Self {
        Self::Left(source, None)
    }
}

impl From<css::Right> for Horizontal {
    fn from(source: css::Right) -> Self {
        Self::Right(source, None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Vertical {
    #[from]
    Center(css::Center),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Top(css::Top, Option<Length>),
    #[display(fmt = "{}{}", _0, "display_helper(_1)")]
    Bottom(css::Bottom, Option<Length>),
}

impl From<css::Top> for Vertical {
    fn from(source: css::Top) -> Self {
        Self::Top(source, None)
    }
}

impl From<css::Bottom> for Vertical {
    fn from(source: css::Bottom) -> Self {
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
    BorderBox(css::BorderBox),
    #[from]
    PaddingBox(css::PaddingBox),
    #[from]
    ContentBox(css::ContentBox),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

pub type Clip = Box;
pub type Origin = Box;

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum LengthPercent {
    #[from]
    Em(Em),
    #[from]
    Ex(Ex),
    #[from]
    Cap(Cap),
    #[from]
    Ch(Ch),
    #[from]
    Ic(Ic),
    #[from]
    Rem(Rem),
    #[from]
    Rlh(Rlh),
    #[from]
    Vm(Vm),
    #[from]
    Vh(Vh),
    #[from]
    Vi(Vi),
    #[from]
    Vb(Vb),
    #[from]
    Vmin(Vmin),
    #[from]
    Vmax(Vmax),
    #[from]
    Cm(Cm),
    #[from]
    Mm(Mm),
    #[from]
    Q(Q),
    #[from]
    In(In),
    #[from]
    Pc(Pc),
    #[from]
    Pt(Pt),
    #[from]
    Px(Px),
    #[from(forward)]
    Percent(Percent),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Size {
    #[from]
    #[display(fmt = "{} {}", _0, _1)]
    WidthHeight(LengthPercent, LengthPercent),
    #[from]
    Auto(css::Auto),
    #[from]
    Cover(css::Cover),
    #[from]
    Contain(css::Contain),
    #[from]
    Initial(css::Initial),
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self::WidthHeight(width.into(), height.into())
    }
}
