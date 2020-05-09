use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Element, Rich)]
#[element(style(indicator, progress_bar), events(indicator, progress_bar))]
pub struct ProgressBar {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[rich(read)]
    #[element(config)]
    styler: Option<<ProgressBar as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // ProgressBar element properties
    #[rich(read(copy))]
    #[element(config(default = "Shape::HLine"))]
    shape: Shape,
    #[rich(read(copy))]
    #[element(config(default = "State::Normal"))]
    state: State,
    #[rich(read(copy))]
    #[element(config(default = "0.0"))]
    value: f64,
    #[rich(read(copy))]
    #[element(config(default = "100.0"))]
    max: f64,
    #[rich(read(copy))]
    #[element(config(default = "0.0"))]
    min: f64,
    #[rich(read(copy))]
    #[element(config)]
    color: Option<css::Color>,
}

pub enum Msg {
    Styler(Option<<ProgressBar as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<ProgressBar>),
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

impl Element for ProgressBar {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        Self {
            id: config.id.unwrap_or_else(Id::generate),
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

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.progress_bar())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Shape(val) => self.shape = val,
            Msg::State(val) => self.state = val,
            Msg::Value(val) => self.set_value(val, orders),
            Msg::Increment(val) => self.set_value(self.value + val, orders),
            Msg::Decrement(val) => self.set_value(self.value - val, orders),
            Msg::Max(val) => self.set_max(val, orders),
            Msg::Min(val) => self.set_min(val, orders),
            Msg::Color(val) => self.color = val,
        }
    }
}

impl Stylable for ProgressBar {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.progress_bar().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for ProgressBar {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for ProgressBar {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let indicator = html::div().class("indicator").set(style.indicator);

        html::div()
            .id(self.id.clone())
            .class("progress-bar")
            .set(style.progress_bar)
            .add(indicator)
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> ProgressBar {
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

impl ProgressBar {
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

impl Msg {
    pub fn styler(val: <ProgressBar as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<ProgressBar>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<ProgressBar as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|s| s.into()))
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

    pub fn try_color(val: Option<impl Into<css::Color>>) -> Self {
        Msg::Color(val.map(|c| c.into()))
    }

    pub fn color(val: impl Into<css::Color>) -> Self {
        Self::try_color(Some(val))
    }
}
