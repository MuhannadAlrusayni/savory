use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

#[derive(Element, Rich)]
pub struct ProgressBar<PMsg> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    // local_events: Events<Msg>,
    #[rich(read)]
    events: Events<PMsg>,
    #[rich(read)]
    style: Option<Style>,
    #[rich(read)]
    theme: Theme,

    // ProgressBar element properties
    #[rich(read(copy))]
    #[element(theme_lens)]
    shape: Shape,
    #[rich(read(copy))]
    #[element(theme_lens)]
    state: State,
    #[rich(read(copy))]
    #[element(theme_lens)]
    value: f64,
    #[rich(read(copy))]
    #[element(theme_lens)]
    max: f64,
    #[rich(read(copy))]
    #[element(theme_lens)]
    min: f64,
    #[rich(read(copy))]
    #[element(theme_lens)]
    color: Option<css::Color>,
}

pub enum Msg {
    SetTheme(Theme),
    SetStyle(Box<dyn FnOnce(Style) -> Style>),
    // shape msgs
    UseHLine,
    UseVLine,
    UseCircle,
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
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        let msg_mapper = msg_mapper.into();
        let mut orders = orders.proxy_with(&msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        Self {
            msg_mapper: msg_mapper,
            // local_events: Events::default(),
            events: Events::default(),
            style: None,
            theme: Theme::default(),
            shape: Shape::HLine,
            state: State::Normal,
            value: 0.0,
            max: 100.0,
            min: 0.0,
            color: None,
        }
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetTheme(val) => self.set_theme(val, &mut orders),
            Msg::SetStyle(val) => self.set_style(val, &mut orders),
            Msg::UseHLine => self.set_shape(Shape::HLine, &mut orders),
            Msg::UseVLine => self.set_shape(Shape::VLine, &mut orders),
            Msg::UseCircle => self.set_shape(Shape::Circle, &mut orders),
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

crate::style_type! {
    indicator,
    progress_bar,
}

crate::events_type! {
    indicator,
    progress_bar,
}

impl<PMsg: 'static> View for ProgressBar<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        let view = |style: &Style| {
            let indicator = html::div()
                .set(att::class("indicator"))
                .set(&style.indicator)
                .map_msg_with(&self.msg_mapper)
                .add(&self.events.indicator);

            html::div()
                .set(att::class("progress_bar"))
                .set(&style.progress_bar)
                .map_msg_with(&self.msg_mapper)
                .add(&self.events.progress_bar)
                .add(indicator)
        };

        match self.style {
            Some(ref style) => view(&style),
            None => view(&self.theme.progress_bar(self.theme_lens())),
        }
    }
}

impl<PMsg: 'static> ProgressBar<PMsg> {
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    fn set_theme<GMsg: 'static>(&mut self, val: Theme, orders: &mut impl Orders<Msg, GMsg>) {
        self.theme = val;
    }

    fn set_style<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Style) -> Style,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        // FIXME: finder better way, that doesn't need to clone the style
        self.style = match self.style {
            Some(ref style) => Some(get_val(style.clone())),
            None => Some(get_val(self.theme.progress_bar(self.theme_lens()))),
        };
    }

    fn set_state<GMsg>(&mut self, val: State, orders: &mut impl Orders<Msg, GMsg>) {
        if self.state != val {
            self.state = val;
        } else {
            orders.skip();
        }
    }

    fn set_shape<GMsg>(&mut self, val: Shape, orders: &mut impl Orders<Msg, GMsg>) {
        if self.shape != val {
            self.shape = val;
        } else {
            orders.skip();
        }
    }

    fn set_value<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg, GMsg>) {
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

    fn increment<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg, GMsg>) {
        self.set_value(self.value + val, orders);
    }

    fn decrement<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg, GMsg>) {
        self.set_value(self.value - val, orders);
    }

    fn set_max<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg, GMsg>) {
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

    fn set_min<GMsg>(&mut self, val: f64, orders: &mut impl Orders<Msg, GMsg>) {
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

    fn set_color<GMsg>(&mut self, val: css::Color, orders: &mut impl Orders<Msg, GMsg>) {
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
    HLine,
    VLine,
    Circle,
}
