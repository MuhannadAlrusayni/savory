use crate::{calc::Calc, unit::*, values as val, St, StyleValues, UpdateStyleValues};
use derive_rich::Rich;
use savory::prelude::DeclarativeConfig;

/// ```
/// use savory_style::{Style, unit::px};
///
/// Style::default()
///     .and_position(|conf| {
///         conf.absolute().top(px(28)).left(px(40))
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Position {
    #[rich(write(rename = position), write(option, rename = try_position), value_fns = {
        static_ = val::Static,
        absolute = val::Absolute,
        fixed = val::Fixed,
        relative = val::Relative,
        sticky = val::Sticky,
        initial = val::Initial,
        inherit = val::Inherit,
    })]
    pub position: Option<PositionType>,
    #[rich(write, write(option))]
    pub left: Option<PostionLength>,
    #[rich(write, write(option))]
    pub top: Option<PostionLength>,
    #[rich(write, write(option))]
    pub right: Option<PostionLength>,
    #[rich(write, write(option))]
    pub bottom: Option<PostionLength>,
    #[rich(write, write(option))]
    pub z_index: Option<i32>,
    #[rich(write, write(option))]
    pub clip: Option<Clip>,
}

impl DeclarativeConfig for Position {}

impl Position {
    pub fn move_top(self) -> Self {
        self.top(px(0))
    }

    pub fn move_right(self) -> Self {
        self.right(px(0))
    }

    pub fn move_bottom(self) -> Self {
        self.bottom(px(0))
    }

    pub fn move_left(self) -> Self {
        self.left(px(0))
    }

    pub fn move_top_stretch(self) -> Self {
        self.move_top().move_left().move_right()
    }

    pub fn move_right_stretch(self) -> Self {
        self.move_right().move_top().move_bottom()
    }

    pub fn move_bottom_stretch(self) -> Self {
        self.move_bottom().move_left().move_right()
    }

    pub fn move_left_stretch(self) -> Self {
        self.move_left().move_top().move_bottom()
    }

    pub fn move_top_right(self) -> Self {
        self.move_top().move_right()
    }

    pub fn move_top_left(self) -> Self {
        self.move_top().move_left()
    }

    pub fn move_bottom_right(self) -> Self {
        self.move_bottom().move_right()
    }

    pub fn move_bottom_left(self) -> Self {
        self.move_bottom().move_left()
    }

    pub fn cover(self) -> Self {
        self.move_top_right().move_bottom_left()
    }
}

impl UpdateStyleValues for Position {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values
            .try_add(St::Position, self.position.as_ref())
            .try_add(St::Left, self.left.as_ref())
            .try_add(St::Top, self.top.as_ref())
            .try_add(St::Right, self.right.as_ref())
            .try_add(St::Bottom, self.bottom.as_ref())
            .try_add(St::ZIndex, self.z_index.as_ref())
            .try_add(St::Clip, self.clip.as_ref())
    }
}

impl<T> From<T> for Position
where
    T: Into<PositionType>,
{
    fn from(source: T) -> Self {
        Self {
            position: Some(source.into()),
            ..Position::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum PostionLength {
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
    #[from]
    Auto(val::Auto),
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
    Calc(Box<Calc<Self>>),
}

impl From<Calc<Self>> for PostionLength {
    fn from(source: Calc<Self>) -> Self {
        Self::Calc(Box::new(source))
    }
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Clip {
    #[from]
    Auto(val::Auto),
    #[display(fmt = "rect({}, {}, {}, {})", top, right, bottom, left)]
    ShapeRect {
        top: ClipRectLength,
        right: ClipRectLength,
        bottom: ClipRectLength,
        left: ClipRectLength,
    },
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

impl Clip {
    pub fn rect(
        top: impl Into<ClipRectLength>,
        right: impl Into<ClipRectLength>,
        bottom: impl Into<ClipRectLength>,
        left: impl Into<ClipRectLength>,
    ) -> Self {
        Self::ShapeRect {
            top: top.into(),
            right: right.into(),
            bottom: bottom.into(),
            left: left.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum ClipRectLength {
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
    #[from]
    Auto(val::Auto),
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
    Calc(Box<Calc<Self>>),
}

impl From<Calc<Self>> for ClipRectLength {
    fn from(source: Calc<Self>) -> Self {
        Self::Calc(Box::new(source))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum PositionType {
    Static(val::Static),
    Absolute(val::Absolute),
    Fixed(val::Fixed),
    Relative(val::Relative),
    Sticky(val::Sticky),
    Initial(val::Initial),
    Inherit(val::Inherit),
}
