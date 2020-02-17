pub use super::box_align::{AlignContent, AlignItems, AlignSelf, JustifyContent};
use crate::css::{unit::*, values as val, St, StyleMap, ToStyleMap};

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Basis {
    #[from]
    Content(val::Content),
    #[from]
    Auto(val::Auto),
    #[from]
    Inherit(val::Inherit),
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

impl ToStyleMap for Basis {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::FlexBasis, self);
        map
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum Direction {
    #[from]
    Row(val::Row),
    #[from]
    RowReverse(val::RowReverse),
    #[from]
    Column(val::Column),
    #[from]
    ColumnReverse(val::ColumnReverse),
}

impl ToStyleMap for Direction {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::FlexDirection, self);
        map
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum Wrap {
    #[from]
    Wrap(val::Wrap),
    #[from]
    Nowrap(val::Nowrap),
    #[from]
    WrapReverse(val::WrapReverse),
}

impl ToStyleMap for Wrap {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::FlexWrap, self);
        map
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub struct Order(i32);

impl ToStyleMap for Order {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::Order, self.0);
        map
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub struct Grow(f32);

impl ToStyleMap for Grow {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::FlexGrow, self.0);
        map
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub struct Shrink(f32);

impl ToStyleMap for Shrink {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::FlexShrink, self.0);
        map
    }
}
