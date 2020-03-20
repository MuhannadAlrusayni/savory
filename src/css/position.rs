use crate::css::{unit::*, values as val, St, StyleMap, ToStyleMap};
use derive_rich::Rich;

/// ```
/// use khalas::css::{Style, unit::px};
///
/// let mut style = Style::default();
/// style
///     .and_position(|conf| {
///         conf.absolute().set_top(px(28)).set_left(px(40))
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Position {
    #[rich(value_fns = {
        static_pos = val::Static,
        absolute = val::Absolute,
        fixed = val::Fixed,
        relative = val::Relative,
        sticky = val::Sticky,
        initial = val::Initial,
        inherit = val::Inherit,
    })]
    position: Option<PositionType>,
    #[rich(write)]
    left: Option<PostionLength>,
    #[rich(write)]
    top: Option<PostionLength>,
    #[rich(write)]
    right: Option<PostionLength>,
    #[rich(write)]
    bottom: Option<PostionLength>,
    #[rich(write)]
    z_index: Option<i32>,
    #[rich(write(style = compose))]
    clip: Option<Clip>,
}

impl ToStyleMap for Position {
    fn style_map(&self) -> StyleMap {
        StyleMap::default()
            .try_add(St::Position, self.position.as_ref())
            .try_add(St::Left, self.left.as_ref())
            .try_add(St::Top, self.top.as_ref())
            .try_add(St::Right, self.right.as_ref())
            .try_add(St::Bottom, self.bottom.as_ref())
            .try_add(St::ZIndex, self.z_index.as_ref())
            .try_add(St::Clip, self.clip.as_ref())
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
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
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
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

impl Default for Clip {
    fn default() -> Self {
        val::Initial.into()
    }
}

impl Clip {
    pub fn auto(self) -> Self {
        val::Auto.into()
    }

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

    pub fn initial(self) -> Self {
        val::Initial.into()
    }

    pub fn inherit(self) -> Self {
        val::Inherit.into()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
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
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum PositionType {
    #[from]
    Static(val::Static),
    #[from]
    Absolute(val::Absolute),
    #[from]
    Fixed(val::Fixed),
    #[from]
    Relative(val::Relative),
    #[from]
    Sticky(val::Sticky),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}
