use crate::css::{unit::*, St, StyleValues, UpdateStyleValues};

/// ```
/// use savory_html::css::{Style, unit::{px, em}};
///
/// let mut style = Style::default();
/// style.gap(px(2))
///      // this can take percent value too (.e.g 40%).
///      .gap(0.4)
///      // and can take row and column each with different value
///      .gap((em(4.), em(8.)));
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

impl UpdateStyleValues for Gap {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::Gap, self)
    }
}
