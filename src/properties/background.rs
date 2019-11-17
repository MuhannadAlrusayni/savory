use crate::{
    macros::*,
    properties::{color::Color, unit::*},
};
use seed::{dom_types::Style, prelude::*};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Background {
    color: Option<Color>,
    // TODO: support multiple images
    image: Option<Image>,
    repeat: Option<Repeat>,
    attachment: Option<Attachment>,
    position: Option<Position>,
    clip: Option<Clip>,
    origin: Option<Origin>,
    size: Option<Size>,
}

impl Background {
    builder_functions! {
        color(Color),
        image(Image),
        position(Position),
        size(Size),
    }

    builder_enum_functions! {
        image {
            empty() => Image::None,
        },
        repeat {
            repeat_x() => Repeat::RepeatX,
            repeat_y() => Repeat::RepeatY,
            repeat() => Repeat::Repeat,
            repeat_with_space() => Repeat::Space,
            repeat_round() => Repeat::Round,
            no_repeat() => Repeat::NoRepeat,
            initial_repeat() => Repeat::Initial,
            inherit_repeat() => Repeat::Inherit,
        },
        attachment {
            scroll() => Attachment::Scroll,
            fixed() => Attachment::Fixed,
            local() => Attachment::Local,
            initial_attachment() => Attachment::Initial,
            inherit_attachment() => Attachment::Inherit,
        },
        position {
            left_top() => (Horizontal::Left(None), Vertical::Top(None)),
            center_top() => (Horizontal::Center, Vertical::Top(None)),
            right_top() => (Horizontal::Right(None), Vertical::Top(None)),
            left_center() => (Horizontal::Left(None), Vertical::Center),
            center() => (Horizontal::Center, Vertical::Center),
            right_center() => (Horizontal::Right(None), Vertical::Center),
            left_bottom() => (Horizontal::Left(None), Vertical::Bottom(None)),
            center_bottom() => (Horizontal::Center, Vertical::Bottom(None)),
            right_bottom() => (Horizontal::Right(None), Vertical::Bottom(None)),
        },
        clip {
            fill_under_border() => Clip::BorderBox,
            fill_inside_border() => Clip::PaddingBox,
            fill_under_content() => Clip::ContentBox,
        }
        origin {
            image_fill_under_border() => Origin::BorderBox,
            image_inside_border() => Origin::PaddingBox,
            image_under_content() => Origin::ContentBox,
        }
        size {
            full() => (1.0, 1.0),
            half() => (0.5, 0.5),
            quarter() => (0.25, 0.25),
            auto_size() => Size::Auto,
            cover() => Size::Cover,
            contain() => Size::Contain,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Image {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "url({})", _0)]
    #[from(forward)]
    Url(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Repeat {
    #[display(fmt = "repeat-x")]
    RepeatX,
    #[display(fmt = "repeat-y")]
    RepeatY,
    #[display(fmt = "repeat")]
    Repeat,
    #[display(fmt = "space")]
    Space,
    #[display(fmt = "round")]
    Round,
    #[display(fmt = "no-repeat")]
    NoRepeat,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Attachment {
    #[display(fmt = "scroll")]
    Scroll,
    #[display(fmt = "fixed")]
    Fixed,
    #[display(fmt = "local")]
    Local,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
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

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Horizontal {
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "left{}", "display_helper(_0)")]
    Left(Option<Length>),
    #[display(fmt = "right{}", "display_helper(_0)")]
    Right(Option<Length>),
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Vertical {
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "top{}", "display_helper(_0)")]
    Top(Option<Length>),
    #[display(fmt = "bottom{}", "display_helper(_0)")]
    Bottom(Option<Length>),
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

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum Box {
    #[display(fmt = "border-box")]
    BorderBox,
    #[display(fmt = "padding-box")]
    PaddingBox,
    #[display(fmt = "content-box")]
    ContentBox,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
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
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "cover")]
    Cover,
    #[display(fmt = "contain")]
    Contain,
    #[display(fmt = "initial")]
    Initial,
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self::WidthHeight(width.into(), height.into())
    }
}

impl From<&Background> for Style {
    fn from(bg: &Background) -> Self {
        style![
            St::BackgroundColor => bg.color,
            St::BackgroundImage => bg.image,
            St::BackgroundRepeat => bg.repeat,
            St::BackgroundAttachment => bg.attachment,
            St::BackgroundPosition => bg.position,
            St::BackgroundClip => bg.clip,
            St::BackgroundOrigin => bg.origin,
            St::BackgroundSize => bg.size,
        ]
    }
}
