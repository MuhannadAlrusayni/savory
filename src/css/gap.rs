use crate::css::{unit::*, St, StyleMap, ToStyleMap};

/// ```
/// use khalas::css::{Style, unit::{px, em}};
///
/// let mut style = Style::default();
/// style.set_gap(px(2))
///      // this can take percent value too (.e.g 40%).
///      .set_gap(0.4)
///      // and can take row and column each with different value
///      .set_gap((em(4.), em(8.)));
/// ```
#[derive(Clone, Debug, Copy, PartialEq, Display)]
pub enum Gap {
    Value(LengthPercent),
    #[display(fmt = "{} {}", _0, _1)]
    RowColumn(LengthPercent, LengthPercent),
}

impl<T> From<T> for Gap
where
    T: Into<LengthPercent>,
{
    fn from(source: T) -> Self {
        Gap::Value(source.into())
    }
}

impl<T1, T2> From<(T1, T2)> for Gap
where
    T1: Into<LengthPercent>,
    T2: Into<LengthPercent>,
{
    fn from((row, col): (T1, T2)) -> Self {
        let row = row.into();
        let col = col.into();
        Gap::RowColumn(row, col)
    }
}

impl ToStyleMap for Gap {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::Gap, self);
        map
    }
}
