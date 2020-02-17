use crate::css::{unit::*, St, StyleMap, ToStyleMap};

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

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Gap {
    #[from(forward)]
    Value(LengthPercent),
    #[display(fmt = "{} {}", _0, _1)]
    #[from]
    RowColumn(LengthPercent, LengthPercent),
}

impl ToStyleMap for Gap {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::Gap, self);
        map
    }
}
