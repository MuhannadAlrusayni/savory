use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

#[derive(Element)]
pub struct ProgressBar<PMsg> {
    // general element properties
    msg_mapper: MsgMapper<Msg<PMsg>, PMsg>,
    local_events: Events<Msg<PMsg>>,
    events: Events<PMsg>,
    style: Option<Style>,
    theme: Theme,
    // ProgressBar element properties
    #[element(theme_lens)]
    shape: Shape,
    #[element(theme_lens)]
    state: State,
    #[element(theme_lens)]
    value: f64,
    #[element(theme_lens)]
    max: f64,
    #[element(theme_lens)]
    min: f64,
    #[element(theme_lens)]
    color: Option<css::Color>,
}

crate::style_type! {
    indicator,
    progress_bar,
}

crate::events_type! {
    indicator,
    progress_bar,
}

pub enum Msg<PMsg> {
    SetStyle(Box<dyn FnOnce(Style) -> Style>),
    SetEvents(Box<dyn FnOnce(Events<PMsg>) -> Events<PMsg>>),
    // shape msgs
    UseLine,
    UseVerticalLine,
    // state msgs
    SetState(State),
    UseNormalState,
    UseSuccessState,
    UseFailureState,
    // value msgs
    SetValue(f64),
    Increment(f64),
    Decrement(f64),
    // max & min msgs
    SetMax(f64),
    SetMin(f64),
    // color msg
    SetColor(css::Color),
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for ProgressBar<PMsg> {
    type Message = Msg<PMsg>;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg<PMsg>, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        orders.subscribe(ThemeChanged);

        Self {
            msg_mapper: msg_mapper.into(),
            local_events: Events::default(),
            events: Events::default(),
            style: None,
            theme: Theme::default(),
            shape: Shape::HorizontalLine,
            state: State::Normal,
            value: 0.0,
            max: 100.0,
            min: 0.0,
            color: None,
        }
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy(self.msg_mapper.map_msg_once());

        match msg {
            Msg::SetStyle(get_style) => {
                self.style = Some(get_style(self.theme.progress_bar(self.theme_lens())));
            }
            Msg::SetEvents(get_events) => self.events = get_events(self.events.clone()),
            Msg::UseLine => self.shape = Shape::HorizontalLine,
            Msg::UseVerticalLine => self.shape = Shape::VerticalLine,
            Msg::SetState(val) => self.set_state(val, &mut orders),
            Msg::UseNormalState => self.set_state(State::Normal, &mut orders),
            Msg::UseSuccessState => self.set_state(State::Success, &mut orders),
            Msg::UseFailureState => self.set_state(State::Failure, &mut orders),
            Msg::SetValue(val) => self.set_value(val, &mut orders),
            Msg::Increment(val) => self.increment(val, &mut orders),
            Msg::Decrement(val) => self.decrement(val, &mut orders),
            Msg::SetMax(val) => self.set_max(val, &mut orders),
            Msg::SetMin(val) => self.set_min(val, &mut orders),
            Msg::SetColor(val) => self.set_color(val, &mut orders),
        }
    }
}

impl<PMsg> Render for ProgressBar<PMsg> {
    type Output = Node<PMsg>;

    fn render(&self) -> Self::Output {
        // let indicator = html::div()
        //     .set(self.style.indicator)
        //     .map_msg_with(&self.msg_mapper)
        //     .add()
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

impl<PMsg: 'static> ProgressBar<PMsg> {
    fn set_state<GMsg>(&mut self, val: State, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        if self.state != val {
            self.state = val;
        } else {
            orders.skip();
        }
    }

    fn set_shape<GMsg>(&mut self, val: Shape, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        if self.shape != val {
            self.shape = val;
        } else {
            orders.skip();
        }
    }

    fn set_value<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
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

    fn increment<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        self.set_value(self.value + val, orders);
        // if self.value < self.max {
        //     self.value = match self.value + val {
        //         x if x <= self.max => x,
        //         _ => self.max,
        //     };
        // } else {
        //     orders.skip();
        // }
    }

    fn decrement<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        self.set_value(self.value - val, orders);
        // if self.value > self.min {
        //     self.value = match self.value - val {
        //         x if x >= self.min => x,
        //         _ => self.min,
        //     };
        // } else {
        //     orders.skip();
        // }
    }

    fn set_max<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        let min = self.min;
        let max = self.max;

        if val > min && val != max {
            self.max = val;
            self.set_value(self.value, orders);
        } else if val < min {
            self.max = self.min;
            self.set_min(val, orders);
        } else if val == min {
            self.max = self.min + 1.0;
            self.set_min(val, orders);
        } else {
            orders.skip();
        }
    }

    fn set_min<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        let min = self.min;
        let max = self.max;

        if val < max && val != min {
            self.min = val;
            self.set_value(self.value, orders);
        } else if val > max {
            self.min = self.max;
            self.set_max(val, orders);
        } else if val == max {
            self.min = self.max - 1.0;
            self.set_max(val, orders);
        } else {
            orders.skip();
        }
    }

    fn set_color<GMsg>(&mut self, val: css::Color, orders: &mut impl Orders<Msg<PMsg>, GMsg>) {
        match self.color {
            Some(color) if color != val => self.color = Some(val),
            None => self.color = Some(val),
            _ => {
                orders.skip();
            }
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Success,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    HorizontalLine,
    VerticalLine,
    Circle,
}
