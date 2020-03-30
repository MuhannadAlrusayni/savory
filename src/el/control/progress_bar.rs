use crate::{css, prelude::*};
use derive_rich::Rich;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Success,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Line {
        vertical: bool,
        internal_result: bool,
    },
    Circle,
}

impl Shape {
    pub fn line() -> Self {
        Self::Line {
            vertical: false,
            internal_result: false,
        }
    }

    pub fn circle() -> Self {
        Self::Circle
    }

    pub fn vertical(self) -> Self {
        match self {
            Self::Line {
                internal_result, ..
            } => Self::Line {
                vertical: true,
                internal_result,
            },
            _ => Self::Line {
                vertical: true,
                internal_result: false,
            },
        }
    }

    pub fn internal_result(self) -> Self {
        match self {
            Self::Line { vertical, .. } => Self::Line {
                vertical,
                internal_result: true,
            },
            _ => Self::Line {
                vertical: false,
                internal_result: true,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Msg {
    Increment(f64),
    Decrement(f64),
    UpdateValue(f64),
    ChangeState(State),
}

#[derive(Default, Rich)]
pub struct LocalEvents {
    #[rich(write(style = compose))]
    pub container: Events<Msg>,
    #[rich(write(style = compose))]
    pub indicator: Events<Msg>,
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(style = compose))]
    pub container: Events<PMsg>,
    #[rich(write(style = compose))]
    pub indicator: Events<PMsg>,
}

impl<PMsg> Default for ParentEvents<PMsg> {
    fn default() -> Self {
        Self {
            container: Events::default(),
            indicator: Events::default(),
        }
    }
}

#[derive(Rich)]
pub struct ProgressBar<PMsg> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: LocalEvents,
    #[rich(read, write(style = compose))]
    events: ParentEvents<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,
    // ProgressBar element properties
    #[rich(read(copy), write)]
    shape: Shape,
    #[rich(read(copy), write, value_fns = {
        success = State::Success,
        failure = State::Failure,
    })]
    state: State,
    #[rich(read(copy), write)]
    value: f64,
    #[rich(read(copy), write)]
    max: f64,
    #[rich(read(copy), write)]
    min: f64,
    #[rich(read(copy), write)]
    color: Option<css::Color>,
}

impl<PMsg> ProgressBar<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        Self {
            msg_mapper: msg_mapper.into(),
            local_events: LocalEvents::default(),
            events: ParentEvents::default(),
            user_style: UserStyle::default(),
            shape: Shape::line(),
            state: State::Normal,
            value: 0.0,
            max: 100.0,
            min: 0.0,
            color: None,
        }
    }
}

impl<PMsg: 'static, GMsg> Model<PMsg, GMsg> for ProgressBar<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::Increment(val) => {
                log!("inc value: ", val);
                if self.value < self.max {
                    self.value = match self.value + val {
                        x if x <= self.max => x,
                        _ => self.max,
                    };
                } else {
                    orders.skip();
                }
            }
            Msg::Decrement(val) => {
                if self.value > self.min {
                    self.value = match self.value - val {
                        x if x >= self.min => x,
                        _ => self.min,
                    };
                } else {
                    orders.skip();
                }
            }
            Msg::UpdateValue(val) => {
                let min = self.min;
                let max = self.max;
                let value = self.value;

                if val < min && value != min {
                    self.value = min;
                } else if val > max && value != max {
                    self.value = max;
                } else if val <= max && val >= min && val != value {
                    self.value = val;
                } else {
                    orders.skip();
                }
            }
            Msg::ChangeState(val) => {
                if self.state != val {
                    self.state = val;
                } else {
                    orders.skip();
                }
            }
        }
    }
}

#[derive(Clone, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    container: css::Style,
    #[rich(write(style = compose))]
    indicator: css::Style,
}

#[derive(Clone, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    container: css::Style,
    #[rich(write(style = compose))]
    indicator: css::Style,
}

impl<PMsg: 'static> Render<PMsg> for ProgressBar<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.progress_bar(self)
    }

    fn render_with_style(&self, _: &impl Theme, style: Self::Style) -> Self::View {
        let indicator = div!()
            .set(style.indicator)
            .set(&self.local_events.container)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.indicator);

        div!()
            .set(style.container)
            .set(&self.local_events.container)
            .add(att::class("progress-bar-container"))
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.container)
            .add(indicator)
    }
}
