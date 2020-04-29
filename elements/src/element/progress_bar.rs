use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

#[derive(Element, Rich)]
#[element(style(indicator, progress_bar), events(indicator, progress_bar))]
pub struct ProgressBar<PMsg> {
    // general element properties
    #[element(config(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    #[element(config(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, config(default))]
    theme: Theme,

    // ProgressBar element properties
    #[rich(read(copy))]
    #[element(theme_lens, config(default = "Shape::HLine"))]
    shape: Shape,
    #[rich(read(copy))]
    #[element(theme_lens, config(default = "State::Normal"))]
    state: State,
    #[rich(read(copy))]
    #[element(theme_lens, config(default = "0.0"))]
    value: f64,
    #[rich(read(copy))]
    #[element(theme_lens, config(default = "100.0"))]
    max: f64,
    #[rich(read(copy))]
    #[element(theme_lens, config(default = "0.0"))]
    min: f64,
    #[rich(read(copy))]
    #[element(theme_lens, config)]
    color: Option<css::Color>,
}

pub enum Msg {
    // EventsStore<Events<PMsg>>
    EventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    UpdateEventsStore(Rc<dyn Any>),
    // Option<Styler<PMsg>>
    Styler(Rc<dyn Any>),
    // Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>
    UpdateStyler(Rc<dyn Any>),
    Theme(Theme),
    Shape(Shape),
    State(State),
    Value(f64),
    Increment(f64),
    Decrement(f64),
    Max(f64),
    Min(f64),
    Color(Option<css::Color>),
}

impl<PMsg: 'static> Element<PMsg> for ProgressBar<PMsg> {
    type Message = Msg;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        Self {
            msg_mapper: config.msg_mapper,
            events: config.events,
            styler: config.styler,
            theme: config.theme,
            shape: config.shape,
            state: config.state,
            value: config.value,
            max: config.max,
            min: config.min,
            color: config.color,
        }
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::EventsStore(val) => {
                if let Ok(val) = val.downcast::<EventsStore<Events<PMsg>>>() {
                    self.events = val.into();
                }
            }
            Msg::UpdateEventsStore(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>>() {
                    self.events = val(self.events.clone());
                }
            }
            Msg::Styler(val) => {
                if let Ok(val) = val.downcast::<Option<Styler<PMsg>>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Shape(val) => self.shape = val,
            Msg::State(val) => self.state = val,
            Msg::Value(val) => self.set_value(val, &mut orders),
            Msg::Increment(val) => self.set_value(self.value + val, &mut orders),
            Msg::Decrement(val) => self.set_value(self.value - val, &mut orders),
            Msg::Max(val) => self.set_max(val, &mut orders),
            Msg::Min(val) => self.set_min(val, &mut orders),
            Msg::Color(val) => self.color = val,
        }
    }
}

impl<PMsg: 'static> View for ProgressBar<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.progress_bar().get(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for ProgressBar<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let events = self.events.get();
        let indicator = html::div()
            .class("indicator")
            .set(style.indicator)
            .map_msg_with(&self.msg_mapper)
            .add(&events.indicator);

        html::div()
            .class("progress-bar")
            .set(style.progress_bar)
            .map_msg_with(&self.msg_mapper)
            .add(&events.progress_bar)
            .add(indicator)
    }
}

impl<PMsg: 'static> Config<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> ProgressBar<PMsg> {
        ProgressBar::init(self, orders)
    }

    pub fn failure(mut self) -> Self {
        self.state = State::Failure;
        self
    }

    pub fn success(mut self) -> Self {
        self.state = State::Success;
        self
    }
}

impl<PMsg: 'static> ProgressBar<PMsg> {
    fn set_value(&mut self, val: f64, _: &mut impl Orders<Msg>) {
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
            // do nothing
        }
    }

    fn set_max(&mut self, val: f64, orders: &mut impl Orders<Msg>) {
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
            // do nothing
        }
    }

    fn set_min(&mut self, val: f64, orders: &mut impl Orders<Msg>) {
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
            // do nothing
        }
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

pub fn events<PMsg>() -> Events<PMsg> {
    Events::default()
}

pub fn style() -> Style {
    Style::default()
}

pub type Styler<PMsg> = theme::Styler<ProgressBar<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<ProgressBarLens<'a>, Style>;

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static>(val: Styler<PMsg>) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static>(
        val: impl Fn(Styler<PMsg>) -> Styler<PMsg> + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static>(val: Option<Styler<PMsg>>) -> Self {
        Msg::Styler(Rc::new(val))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn shape(val: Shape) -> Self {
        Msg::Shape(val)
    }

    pub fn horizontal_line() -> Self {
        Msg::Shape(Shape::HLine)
    }

    pub fn hline() -> Self {
        Msg::horizontal_line()
    }

    pub fn vertical_line() -> Self {
        Msg::shape(Shape::VLine)
    }

    pub fn vline() -> Self {
        Msg::vertical_line()
    }

    pub fn circle() -> Self {
        Msg::shape(Shape::Circle)
    }

    pub fn state(val: State) -> Self {
        Msg::State(val)
    }

    pub fn normal() -> Self {
        Msg::state(State::Normal)
    }

    pub fn success() -> Self {
        Msg::state(State::Success)
    }

    pub fn failure() -> Self {
        Msg::state(State::Failure)
    }

    pub fn increment(val: f64) -> Self {
        Msg::Increment(val)
    }

    pub fn decrement(val: f64) -> Self {
        Msg::Decrement(val)
    }

    pub fn max(val: f64) -> Self {
        Msg::Max(val)
    }

    pub fn min(val: f64) -> Self {
        Msg::Min(val)
    }

    pub fn try_color(val: Option<css::Color>) -> Self {
        Msg::Color(val)
    }

    pub fn color(val: css::Color) -> Self {
        Self::try_color(Some(val))
    }
}
