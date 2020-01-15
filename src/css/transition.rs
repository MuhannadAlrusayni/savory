use crate::css::{
    self,
    unit::{sec, Ms, Sec},
    St, Style, ToStyle,
};
use derive_rich::Rich;
use std::{borrow::Cow, collections::HashMap};

#[derive(Default, Clone, Debug, PartialEq, From)]
pub struct Transition {
    pub transitions: HashMap<Cow<'static, str>, TransitionValue>,
}

impl ToStyle for Transition {
    fn to_style(&self) -> Style {
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

        Style::default().add(St::Transition, transitions.join(", "))
    }
}

impl Transition {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::default(),
        }
    }

    pub fn all(mut self, get_trans: impl Fn(TransitionValue) -> TransitionValue) -> Self {
        let trans_value = TransitionValue::new(sec(1.));
        self.transitions
            .insert("all".into(), get_trans(trans_value));
        self
    }

    pub fn add(
        mut self,
        property: impl Into<Cow<'static, str>>,
        get_trans: impl Fn(TransitionValue) -> TransitionValue,
    ) -> Self {
        let trans_value = TransitionValue::new(sec(1.));
        self.transitions
            .insert(property.into(), get_trans(trans_value));
        self
    }
}

#[derive(Rich, Clone, Debug, PartialEq, From)]
pub struct TransitionValue {
    #[rich(write(take))]
    pub duration: Duration,
    #[rich(value_fns(take) = {
        ease = css::Ease,
        linear = css::Linear,
        ease_in = css::EaseIn,
        ease_out = css::EaseOut,
        ease_in_out = css::EaseInOut,
        step_start = css::StepStart,
        step_end = css::StepEnd,
        initial = css::Initial,
        inherit = css::Inherit,
    })]
    pub timing_function: Option<TimingFunction>,
    #[rich(write(take))]
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
    Ease(css::Ease),
    #[from]
    Linear(css::Linear),
    #[from]
    EaseIn(css::EaseIn),
    #[from]
    EaseOut(css::EaseOut),
    #[from]
    EaseInOut(css::EaseInOut),
    #[from]
    StepStart(css::StepStart),
    #[from]
    StepEnd(css::StepEnd),
    #[display(fmt = "steps({}, {})", _0, _1)]
    Steps(usize, StepsPos),
    #[display(fmt = "cubic-bezier({}, {}, {}, {})", _0, _1, _2, _3)]
    CubicBezier(f32, f32, f32, f32),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum StepsPos {
    #[from]
    Start(css::Start),
    #[from]
    End(css::End),
}

#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Duration {
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
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
