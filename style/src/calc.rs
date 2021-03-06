use std::fmt;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Calc<T> {
    Val(T),
    Sum(Box<Calc<T>>, T),
    Sub(Box<Calc<T>>, T),
    Mul(Box<Calc<T>>, T),
    Div(Box<Calc<T>>, T),
}

impl<T: Clone> Clone for Calc<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Val(v) => Self::Val(v.clone()),
            Self::Sum(c, v) => Self::Sum(c.clone(), v.clone()),
            Self::Sub(c, v) => Self::Sub(c.clone(), v.clone()),
            Self::Mul(c, v) => Self::Mul(c.clone(), v.clone()),
            Self::Div(c, v) => Self::Div(c.clone(), v.clone()),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Calc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_calc<T: fmt::Display>(calc: &Calc<T>) -> String {
            match calc {
                Calc::Val(val) => format!("{}", val),
                Calc::Sum(c, val) => format!("({} + {})", fmt_calc(c), val),
                Calc::Sub(c, val) => format!("({} - {})", fmt_calc(c), val),
                Calc::Mul(c, val) => format!("({} * {})", fmt_calc(c), val),
                Calc::Div(c, val) => format!("({} / {})", fmt_calc(c), val),
            }
        }
        match self {
            Self::Val(_) => write!(f, "calc({})", fmt_calc(self)),
            _ => write!(f, "calc{}", fmt_calc(self)),
        }
    }
}

impl<T> Calc<T> {
    pub fn sum(self, val: impl Into<T>) -> Self {
        Self::Sum(Box::new(self), val.into())
    }
    pub fn sub(self, val: impl Into<T>) -> Self {
        Self::Sub(Box::new(self), val.into())
    }
    pub fn mul(self, val: impl Into<T>) -> Self {
        Self::Mul(Box::new(self), val.into())
    }
    pub fn div(self, val: impl Into<T>) -> Self {
        Self::Div(Box::new(self), val.into())
    }
}

pub fn calc<T>(val: impl Into<T>, f: impl FnOnce(Calc<T>) -> Calc<T>) -> Calc<T> {
    f(Calc::Val(val.into()))
}
