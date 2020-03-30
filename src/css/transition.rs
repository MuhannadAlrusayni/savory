use crate::css::{
    unit::{sec, Ms, Sec},
    values as val, St, StyleMap, ToStyleMap,
};
use derive_rich::Rich;
use indexmap::IndexMap;
use std::{
    borrow::Cow,
    ops::{Add, AddAssign},
};

/// ```
/// use khalas::css::{values as val, Style, unit::{sec, ms}};
///
/// let mut style = Style::default();
/// style
///     .and_transition(|conf| {
///         conf
///             // transition for all properties
///             .all(|conf| {
///                 conf.set_duration(sec(0.3))
///                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
///             })
///             // or transition for specific properties (e.g. opacity only)
///             .add("opacity", |conf| {
///                 conf.set_duration(ms(150.))
///                     .ease()
///                     .set_delay(sec(0.5))
///             })
///         });
/// ```
#[derive(Default, Clone, Debug, PartialEq, From)]
pub struct Transition {
    pub transitions: IndexMap<Cow<'static, str>, TransitionValue>,
}

impl Add for Transition {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign for Transition {
    fn add_assign(&mut self, other: Self) {
        for (name, val) in other.transitions.into_iter() {
            self.transitions.insert(name, val);
        }
    }
}

impl ToStyleMap for Transition {
    fn style_map(&self) -> StyleMap {
        let mut transitions = vec![];
        for (property, value) in self.transitions.iter() {
            let mut trans = vec![property.to_string()];
            trans.push(value.duration.to_string());
            if let Some(timing_fn) = value.timing_function {
                trans.push(timing_fn.to_string());
            }
            if let Some(delay) = value.delay {
                trans.push(delay.to_string())
            }
            transitions.push(trans.join(" "));
        }

        StyleMap::default().add(St::Transition, transitions.join(", "))
    }
}

impl Transition {
    pub fn new() -> Self {
        Self {
            transitions: IndexMap::default(),
        }
    }

    pub fn all(mut self, get_trans: impl Fn(TransitionValue) -> TransitionValue) -> Self {
        self.transitions
            .insert("all".into(), get_trans(TransitionValue::new(sec(1.))));
        self
    }

    pub fn add(
        mut self,
        property: impl Into<Cow<'static, str>>,
        get_trans: impl Fn(TransitionValue) -> TransitionValue,
    ) -> Self {
        self.transitions
            .insert(property.into(), get_trans(TransitionValue::new(sec(1.))));
        self
    }
}

#[derive(Rich, Clone, Debug, PartialEq, From)]
pub struct TransitionValue {
    #[rich(write)]
    pub duration: Duration,
    #[rich(value_fns = {
        ease = val::Ease,
        linear = val::Linear,
        ease_in = val::EaseIn,
        ease_out = val::EaseOut,
        ease_in_out = val::EaseInOut,
        step_start = val::StepStart,
        step_end = val::StepEnd,
        initial = val::Initial,
        inherit = val::Inherit,
    })]
    pub timing_function: Option<TimingFunction>,
    #[rich(write)]
    pub delay: Option<Delay>,
}

impl TransitionValue {
    pub fn new(duration: impl Into<Duration>) -> Self {
        Self {
            duration: duration.into(),
            timing_function: None,
            delay: None,
        }
    }

    pub fn steps(mut self, intervals: usize, pos: impl Into<StepsPos>) -> Self {
        self.timing_function = Some(TimingFunction::Steps(intervals, pos.into()));
        self
    }

    pub fn cubic_bezier(mut self, n1: f32, n2: f32, n3: f32, n4: f32) -> Self {
        self.timing_function = Some(TimingFunction::CubicBezier(n1, n2, n3, n4));
        self
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum TimingFunction {
    #[from]
    Ease(val::Ease),
    #[from]
    Linear(val::Linear),
    #[from]
    EaseIn(val::EaseIn),
    #[from]
    EaseOut(val::EaseOut),
    #[from]
    EaseInOut(val::EaseInOut),
    #[from]
    StepStart(val::StepStart),
    #[from]
    StepEnd(val::StepEnd),
    #[display(fmt = "steps({}, {})", _0, _1)]
    Steps(usize, StepsPos),
    #[display(fmt = "cubic-bezier({}, {}, {}, {})", _0, _1, _2, _3)]
    CubicBezier(f32, f32, f32, f32),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum StepsPos {
    #[from]
    Start(val::Start),
    #[from]
    End(val::End),
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Duration {
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
    #[from]
    Ms(Ms),
    #[from]
    Sec(Sec),
}

impl From<std::time::Duration> for Duration {
    fn from(source: std::time::Duration) -> Self {
        sec(source.as_secs_f32()).into()
    }
}

pub type Delay = Duration;
