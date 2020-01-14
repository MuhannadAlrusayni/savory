use crate::css::{
    self,
    unit::{Ms, Sec},
    St, ToStyle,
};
use derive_rich::Rich;

pub struct TransitionGroup {
    pub transitions: Vec<Transition>,
}

#[derive(Rich, Clone, Debug, PartialEq, Display, From)]
#[display(fmt = "transition: {} {} {} {}")]
pub struct Transition {
    #[rich(write(take))]
    pub property: Cow<'static, str>,
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

impl Transition {
    pub fn steps(mut self, intervals: usize, pos: impl Into<StepsPos>) -> Self {
        self.timing_function = TimingFunction::Steps(intervals, pos.into());
        self
    }

    pub fn cubic_bezier(mut self, n1: f32, n2: f32, n3: f32, n4: f32) -> Self {
        self.timing_function = TimingFunction::CubicBezier(n1, n2, n3, n4);
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

pub type Delay = Duration;
