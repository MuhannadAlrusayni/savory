use crate::{
    unit::{sec, Ms, Sec},
    values as val, St, StyleValues, UpdateStyleValues,
};
use derive_rich::Rich;
use indexmap::IndexMap;
use savory::prelude::DeclarativeConfig;
use std::borrow::Cow;

/// ```
/// use savory_style::{values as val, Style, unit::{sec, ms}};
///
/// Style::default()
///     // transition for all properties
///     .and_transition(|conf| conf.duration(sec(1.2)).ease())
///     // transitions for specific properties
///     .and_transition(|conf| {
///         conf.add("opacity", |conf| conf.duration(ms(150.)).ease().delay(sec(0.5)))
///             .add("width", |conf| conf.duration(ms(450.)).ease_in())
///     });
/// ```
#[derive(Clone, Debug, PartialEq, From)]
pub enum Transition {
    One(TransitionValue),
    Multiple(IndexMap<Cow<'static, str>, TransitionValue>),
    Initial(val::Initial),
    Inherit(val::Inherit),
    None(val::None),
    Unset(val::Unset),
}

impl DeclarativeConfig for Transition {}

impl Default for Transition {
    fn default() -> Self {
        Transition::Multiple(IndexMap::default())
    }
}

impl UpdateStyleValues for Transition {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        let to_string = |prop: Option<Cow<'static, str>>, val: TransitionValue| {
            let mut vals = vec![];

            if let Some(name) = prop {
                vals.push(name.to_string());
            }

            vals.push(val.duration.to_string());

            if let Some(timing_fn) = val.timing_function {
                vals.push(timing_fn.to_string());
            }

            if let Some(delay) = val.delay {
                vals.push(delay.to_string())
            }

            vals.join(" ")
        };

        let val = match self {
            Self::Initial(val) => val.to_string(),
            Self::Inherit(val) => val.to_string(),
            Self::None(val) => val.to_string(),
            Self::Unset(val) => val.to_string(),
            Self::One(transition) => to_string(None, transition),
            Self::Multiple(map) => {
                let val = map
                    .into_iter()
                    .map(|(prop, val)| to_string(Some(prop), val))
                    .collect::<Vec<_>>()
                    .join(", ");
                // if no shadow value added we return values without any updates
                if val.is_empty() {
                    return values;
                }
                val
            }
        };

        values.add(St::Transition, val)
    }
}

impl Transition {
    fn transition(mut self, conf: impl FnOnce(TransitionValue) -> TransitionValue) -> Self {
        self = match self {
            Self::One(val) => Self::One(conf(val)),
            _ => Self::One(conf(TransitionValue::default())),
        };
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn duration(self, val: impl Into<Duration>) -> Self {
        self.transition(|t| t.duration(val))
    }

    pub fn timing_function(self, val: impl Into<TimingFunction>) -> Self {
        self.transition(|t| t.timing_function(val))
    }

    pub fn try_timing_function(self, val: Option<impl Into<TimingFunction>>) -> Self {
        self.transition(|t| t.try_timing_function(val))
    }

    pub fn ease(self) -> Self {
        self.transition(|t| t.ease())
    }

    pub fn linear(self) -> Self {
        self.transition(|t| t.linear())
    }

    pub fn ease_in(self) -> Self {
        self.transition(|t| t.ease_in())
    }

    pub fn ease_out(self) -> Self {
        self.transition(|t| t.ease_out())
    }

    pub fn ease_in_out(self) -> Self {
        self.transition(|t| t.ease_in_out())
    }

    pub fn step_start(self) -> Self {
        self.transition(|t| t.step_start())
    }

    pub fn step_end(self) -> Self {
        self.transition(|t| t.step_end())
    }

    pub fn steps(self, intervals: usize, pos: impl Into<StepsPos>) -> Self {
        self.transition(|t| t.steps(intervals, pos))
    }

    pub fn cubic_bezier(self, n1: f32, n2: f32, n3: f32, n4: f32) -> Self {
        self.transition(|t| t.cubic_bezier(n1, n2, n3, n4))
    }

    pub fn delay(self, val: impl Into<Delay>) -> Self {
        self.transition(|t| t.delay(val))
    }

    pub fn try_delay(self, val: Option<impl Into<Delay>>) -> Self {
        self.transition(|t| t.try_delay(val))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(
        mut self,
        property: impl Into<Cow<'static, str>>,
        get_val: impl FnOnce(TransitionValue) -> TransitionValue,
    ) -> Self {
        let val = get_val(TransitionValue::default());
        self = match self {
            Self::Multiple(mut map) => {
                map.insert(property.into(), val);
                Self::Multiple(map)
            }
            _ => {
                let mut map = IndexMap::default();
                map.insert(property.into(), val);
                Self::Multiple(map)
            }
        };
        self
    }
}

#[derive(Rich, Clone, Debug, PartialEq, From)]
pub struct TransitionValue {
    #[rich(write)]
    pub duration: Duration,
    #[rich(write(rename = timing_function), write(option, rename = try_timing_function), value_fns = {
        ease = val::Ease,
        linear = val::Linear,
        ease_in = val::EaseIn,
        ease_out = val::EaseOut,
        ease_in_out = val::EaseInOut,
        step_start = val::StepStart,
        step_end = val::StepEnd,
    })]
    pub timing_function: Option<TimingFunction>,
    #[rich(write, write(option))]
    pub delay: Option<Delay>,
}

impl Default for TransitionValue {
    fn default() -> Self {
        Self::new(val::Unset)
    }
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
    Unset(val::Unset),
    Ms(Ms),
    Sec(Sec),
}

impl From<std::time::Duration> for Duration {
    fn from(source: std::time::Duration) -> Self {
        sec(source.as_secs_f32()).into()
    }
}

pub type Delay = Duration;
