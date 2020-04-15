use crate::css::{
    unit::{sec, Ms, Sec},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;
use indexmap::IndexMap;
use std::borrow::Cow;

/// ```
/// use savory::css::{values as val, Style, unit::{sec, ms}};
///
/// let mut style = Style::default();
/// style
///     .and_transition(|conf| {
///         conf
///             // transition for all properties
///             .all(|conf| {
///                 conf.duration(sec(0.3))
///                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
///             })
///             // or transition for specific properties (e.g. opacity only)
///             .add("opacity", |conf| {
///                 conf.duration(ms(150.))
///                     .ease()
///                     .delay(sec(0.5))
///             })
///         });
/// ```
#[derive(Default, Clone, Debug, PartialEq, From)]
pub struct Transition {
    pub transitions: IndexMap<Cow<'static, str>, TransitionValue>,
}

impl UpdateStyleValues for Transition {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
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

        values.add(St::Transition, transitions.join(", "))
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
    #[rich(write(rename = duration))]
    pub duration: Duration,
    #[rich(write(rename = timing_function), write(option, rename = try_timing_function), value_fns = {
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
    #[rich(write(rename = delay), write(option, rename = try_delay))]
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
    Ease(val::Ease),
    Linear(val::Linear),
    EaseIn(val::EaseIn),
    EaseOut(val::EaseOut),
    EaseInOut(val::EaseInOut),
    StepStart(val::StepStart),
    StepEnd(val::StepEnd),
    #[display(fmt = "steps({}, {})", _0, _1)]
    Steps(usize, StepsPos),
    #[display(fmt = "cubic-bezier({}, {}, {}, {})", _0, _1, _2, _3)]
    CubicBezier(f32, f32, f32, f32),
    Initial(val::Initial),
    Inherit(val::Inherit),
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum StepsPos {
    Start(val::Start),
    End(val::End),
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Duration {
    Initial(val::Initial),
    Inherit(val::Inherit),
    Ms(Ms),
    Sec(Sec),
}

impl From<std::time::Duration> for Duration {
    fn from(source: std::time::Duration) -> Self {
        sec(source.as_secs_f32()).into()
    }
}

pub type Delay = Duration;
