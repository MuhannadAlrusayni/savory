use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Element, Rich)]
#[element(style(indicator, progress_bar), events(indicator, progress_bar))]
pub struct ProgressBar<PMsg> {
    // general element properties
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    // local_events: Events<Msg>,
    #[rich(read)]
    #[element(props(default))]
    events: Events<PMsg>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    // ProgressBar element properties
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "Shape::HLine"))]
    shape: Shape,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "State::Normal"))]
    state: State,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "0.0"))]
    value: f64,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "100.0"))]
    max: f64,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "0.0"))]
    min: f64,
    #[rich(read(copy))]
    #[element(theme_lens, props)]
    color: Option<css::Color>,
}

pub enum Msg {
    SetTheme(Theme),
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
    TrySetColor(Option<css::Color>),
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for ProgressBar<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        Self {
            msg_mapper: props.msg_mapper,
            // local_events: Events::default(),
            events: props.events,
            styler: props.styler,
            theme: props.theme,
            shape: props.shape,
            state: props.state,
            value: props.value,
            max: props.max,
            min: props.min,
            color: props.color,
        }
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetTheme(val) => self.theme = val,
            Msg::UseHLine => self.shape = Shape::HLine,
            Msg::UseVLine => self.shape = Shape::VLine,
            Msg::UseCircle => self.shape = Shape::Circle,
            Msg::SetState(val) => self.state = val,
            Msg::UseNormalState => self.state = State::Normal,
            Msg::UseSuccessState => self.state = State::Success,
            Msg::UseFailureState => self.state = State::Failure,
            Msg::SetValue(val) => self.set_value(val, &mut orders),
            Msg::Increment(val) => self.set_value(self.value + val, &mut orders),
            Msg::Decrement(val) => self.set_value(self.value - val, &mut orders),
            Msg::SetMax(val) => self.set_max(val, &mut orders),
            Msg::SetMin(val) => self.set_min(val, &mut orders),
            Msg::SetColor(val) => self.color = Some(val),
            Msg::TrySetColor(val) => self.color = val,
        }
    }
}

impl<PMsg: 'static> View for ProgressBar<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.progress_bar()(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for ProgressBar<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let indicator = html::div()
            .class("indicator")
            .set(&style.indicator)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.indicator);

        html::div()
            .class("progress-bar")
            .set(&style.progress_bar)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.progress_bar)
            .add(indicator)
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init<GMsg: 'static>(self, orders: &mut impl Orders<PMsg, GMsg>) -> ProgressBar<PMsg> {
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
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn try_set_styler<GMsg: 'static>(
        &mut self,
        val: Option<impl Into<Styler<PMsg>>>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler<GMsg: 'static>(
        &mut self,
        val: impl Into<Styler<PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.try_set_styler(Some(val), orders)
    }

    fn set_value<GMsg>(&mut self, val: f64, _: &mut impl Orders<Msg, GMsg>) {
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
            // do nothing
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

pub type Styler<PMsg> = theme::Styler<ProgressBar<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<ProgressBarLens<'a>, Style>;
