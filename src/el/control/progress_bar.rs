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

#[derive(Rich, Element)]
pub struct ProgressBar<PMsg> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: Events<Msg>,
    #[rich(read, write(style = compose))]
    events: Events<PMsg>,
    #[rich(read, write(style = compose))]
    #[element(theme_lens)]
    user_style: Style,
    // ProgressBar element properties
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    shape: Shape,
    #[rich(read(copy), write, value_fns = {
        success = State::Success,
        failure = State::Failure,
    })]
    #[element(theme_lens)]
    state: State,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    value: f64,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    max: f64,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    min: f64,
    #[rich(read(copy), write)]
    #[element(theme_lens)]
    color: Option<css::Color>,
}

impl<PMsg> ProgressBar<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        Self {
            msg_mapper: msg_mapper.into(),
            local_events: Events::default(),
            events: Events::default(),
            user_style: Style::default(),
            shape: Shape::line(),
            state: State::Normal,
            value: 0.0,
            max: 100.0,
            min: 0.0,
            color: None,
        }
    }
}

impl<PMsg: 'static> Model<PMsg> for ProgressBar<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg>) {
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

impl<PMsg> Render for ProgressBar<PMsg> {
    type View = Node<PMsg>;

    fn style(&self, theme: &Theme) -> Style {
        theme.progress_bar(self.theme_lens())
    }

    fn render_with_style(&self, _: &Theme, style: Style) -> Self::View {
        todo!()
        // let indicator = div!()
        //     .set(style["indicator"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("indicator"));

        // div!()
        //     .set(style["progress-bar"])
        //     .add(att::class("progress-bar"))
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("progress-bar"))
        //     .add(indicator)
    }
}
