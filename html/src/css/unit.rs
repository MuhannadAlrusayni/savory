// TODO: Impl Add, Div, Sub, Mul, PartialEq and other triats for unit types
// where posible.

// Font-relative lengths
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}em", _0)]
pub struct Em(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}ex", _0)]
pub struct Ex(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}cap", _0)]
pub struct Cap(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}ch", _0)]
pub struct Ch(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}ic", _0)]
pub struct Ic(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}rem", _0)]
pub struct Rem(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}rlh", _0)]
pub struct Rlh(f32);

// Viewport-relative lengths
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}vm", _0)]
pub struct Vm(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}vh", _0)]
pub struct Vh(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}vi", _0)]
pub struct Vi(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}vb", _0)]
pub struct Vb(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}vmin", _0)]
pub struct Vmin(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}vmax", _0)]
pub struct Vmax(f32);

// Absolute lengths
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}cm", _0)]
pub struct Cm(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}mm", _0)]
pub struct Mm(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}q", _0)]
pub struct Q(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}in", _0)]
pub struct In(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}pc", _0)]
pub struct Pc(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}pt", _0)]
pub struct Pt(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}px", _0)]
pub struct Px(f32);

impl From<i32> for Px {
    fn from(source: i32) -> Self {
        Px(source as f32)
    }
}

impl From<i16> for Px {
    fn from(source: i16) -> Self {
        Px(source as f32)
    }
}

impl From<i8> for Px {
    fn from(source: i8) -> Self {
        Px(source as f32)
    }
}

// Parent-relative
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}%", "_0 * 100.0")]
pub struct Percent(f32);

// Time units
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}ms", _0)]
pub struct Ms(f32);

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Display, From)]
#[display(fmt = "{}s", _0)]
pub struct Sec(f32);

macro_rules! construct_fn{
    ( $( $fn:ident($from_ty:ty) -> $ty:ident $(,)? )* ) => {
        $(
            pub fn $fn(value: impl Into<$from_ty>) -> $ty
            where
                $from_ty: Into<$ty>,
            {
                (value.into()).into()
            }
        )*
    }
}

construct_fn! {
    px(Px) -> Length,
    rem(Rem) -> Length,
    em(Em) -> Length,
    cm(Cm) -> Length,
    inch(In) -> Length,
    percent(Percent) -> Percent,
    pct(Percent) -> Percent,
    // time fns
    ms(Ms) -> Ms,
    sec(Sec) -> Sec,
}

// sum types

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Display, From)]
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

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum LengthPercent {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}
