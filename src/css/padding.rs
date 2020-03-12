use super::{
    unit::{self, *},
    values as val, St, StyleMap, ToStyleMap,
};
use derive_rich::Rich;

/// ```
/// use khalas::css::{values as val, Style, unit::px};
///
/// let mut style = Style::default();
/// style
///     .and_padding(|conf| {
///         conf.x(px(2))
///             .y(px(4))
///     });
/// ```
#[derive(Rich, Clone, Debug, Copy, PartialEq, From, Default)]
pub struct Padding {
    #[rich(read, write)]
    top: Option<Length>,
    #[rich(read, write)]
    right: Option<Length>,
    #[rich(read, write)]
    bottom: Option<Length>,
    #[rich(read, write)]
    left: Option<Length>,
}

impl From<Length> for Padding {
    fn from(source: Length) -> Self {
        let mut padding = Padding::default();
        padding.all(source);
        padding
    }
}

impl ToStyleMap for Padding {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.try_add(St::PaddingTop, self.top)
            .try_add(St::PaddingRight, self.right)
            .try_add(St::PaddingBottom, self.bottom)
            .try_add(St::PaddingLeft, self.left);
        map
    }
}

impl Padding {
    pub fn all(&mut self, value: impl Into<Length>) -> &mut Self {
        let value = value.into();
        self.right(value).top(value).left(value).bottom(value)
    }

    pub fn zero(&mut self) -> &mut Self {
        self.all(px(0.))
    }

    pub fn x(&mut self, value: impl Into<Length>) -> &mut Self {
        let value = value.into();
        self.left(value).right(value)
    }

    pub fn y(&mut self, value: impl Into<Length>) -> &mut Self {
        let value = value.into();
        self.top(value).bottom(value)
    }

    pub fn horizontal(&mut self, value: impl Into<Length>) -> &mut Self {
        self.y(value)
    }

    pub fn vertical(&mut self, value: impl Into<Length>) -> &mut Self {
        self.x(value)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Length {
    #[from]
    Length(unit::Length),
    #[from(forward)]
    Percent(Percent),
    #[from]
    Inherit(val::Inherit),
}
