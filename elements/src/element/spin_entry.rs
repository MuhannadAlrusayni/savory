use crate::{button::ButtonLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

// TODO: add way to accept custom format (e.g. `100%`, `45$`)
#[derive(Rich, Element)]
#[element(
    style(
        input,
        spin_entry,
        increment_button(button::Style),
        decrement_button(button::Style),
    ),
    events(input, spin_entry)
)]
pub struct SpinEntry<PMsg> {
    #[rich(read)]
    #[element(config)]
    id: Id,
    el_ref: ElRef<web_sys::HtmlInputElement>,
    #[element(config(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<SpinEntry<PMsg> as Stylable>::Styler>,
    #[rich(read)]
    #[element(theme_lens, config(default))]
    theme: Theme,

    #[rich(read(copy))]
    #[element(config)]
    value: Option<f64>,
    // this value is an internal API, and shouldn't get exposed
    vis_value: String,
    #[rich(read(copy))]
    #[element(config)]
    max: Option<f64>,
    #[rich(read(copy))]
    #[element(config)]
    min: Option<f64>,
    #[rich(read(copy))]
    #[element(config(default = "1.0"))]
    step: f64,
    #[rich(read(copy))]
    #[element(config)]
    placeholder: Option<f64>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, config(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,

    // children elements
    #[rich(read)]
    #[element(theme_lens(nested), config(nested, default = "inc_btn_config()"))]
    increment_button: Button<Msg>,
    #[rich(read)]
    #[element(theme_lens(nested), config(nested, default = "dec_btn_config()"))]
    decrement_button: Button<Msg>,
}

pub enum Msg {
    // EventsStore<Events<PMsg>>
    EventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    UpdateEventsStore(Rc<dyn Any>),
    // Option<<Self as Stylable>::Styler>
    Styler(Rc<dyn Any>),
    // Box<dyn Fn(<Self as Stylable>::Styler) -> <Self as Stylable>::Styler>
    UpdateStyler(Rc<dyn Any>),
    Theme(Theme),
    Value(Option<f64>),
    Min(Option<f64>),
    Max(Option<f64>),
    Step(f64),
    Placeholder(Option<f64>),
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
    Increment,
    Decrement,
    Input,
    IncrementButton(button::Msg),
    DecrementButton(button::Msg),
}

impl<PMsg: 'static> Element<PMsg> for SpinEntry<PMsg> {
    type Message = Msg;

    fn init(config: Self::Config, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&config.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::Theme(theme.0));

        let local_events = || {
            events()
                .and_input(|conf| {
                    conf.input(|_| Msg::input())
                        .focus(|_| Msg::focus(true))
                        .blur(|_| Msg::focus(false))
                })
                .and_spin_entry(|conf| {
                    conf.mouse_enter(|_| Msg::mouse_over(true))
                        .mouse_leave(|_| Msg::mouse_over(false))
                })
        };

        let mut spin_entry = Self {
            id: config.id.unwrap_or_else(Id::generate),
            el_ref: ElRef::default(),
            msg_mapper: config.msg_mapper,
            local_events: local_events.into(),
            events: config.events,
            styler: config.styler,
            theme: config.theme,
            value: config.value,
            vis_value: config
                .value
                .map(|v| v.to_string())
                .unwrap_or_else(Default::default),
            max: config.max,
            min: config.min,
            step: config.step,
            placeholder: config.placeholder,
            disabled: config.disabled,
            focused: false,
            mouse_over: false,
            increment_button: config.increment_button.init(&mut orders),
            decrement_button: config.decrement_button.init(&mut orders),
        };

        // Fix invalid values
        spin_entry.try_set_max(config.max, &mut orders);
        spin_entry.try_set_min(config.min, &mut orders);
        spin_entry.try_set_value(config.value, &mut orders);
        spin_entry.try_set_placeholder(config.placeholder, &mut orders);
        spin_entry
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg>) {
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
                if let Ok(val) = val.downcast::<Option<<Self as Stylable>::Styler>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(<Self as Stylable>::Styler) -> <Self as Stylable>::Styler>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Value(val) => self.try_set_value(val, &mut orders),
            Msg::Min(val) => self.try_set_min(val, &mut orders),
            Msg::Max(val) => self.try_set_max(val, &mut orders),
            Msg::Step(val) => self.set_step(val, &mut orders),
            Msg::Placeholder(val) => self.try_set_placeholder(val, &mut orders),
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::Increment => {
                self.try_set_value(Some(self.get_value_or_default() + self.step), &mut orders)
            }
            Msg::Decrement => {
                self.try_set_value(Some(self.get_value_or_default() - self.step), &mut orders)
            }
            Msg::Input => self.input(&mut orders),
            Msg::IncrementButton(msg) => self.increment_button.update(msg, &mut orders),
            Msg::DecrementButton(msg) => self.decrement_button.update(msg, &mut orders),
        }
    }
}

impl<PMsg> Stylable for SpinEntry<PMsg> {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.spin_entry().get(&s.theme_lens())).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl<PMsg: 'static> View for SpinEntry<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(self.style())
    }
}

impl<PMsg: 'static> StyledView for SpinEntry<PMsg> {
    fn styled_view(&self, style: Self::Style) -> Self::Output {
        let local_events = self.local_events.get();
        let events = self.events.get();

        let inc_btn = self
            .increment_button
            .styled_view(style.increment_button)
            .map_msg_with(&self.msg_mapper);
        let dec_btn = self
            .decrement_button
            .styled_view(style.decrement_button)
            .map_msg_with(&self.msg_mapper);

        // input
        let input = html::input()
            .el_ref(&self.el_ref)
            .set(&local_events.input)
            .set(style.input)
            .and_attributes(|conf| {
                conf.class("input")
                    .input_mode(att::InputMode::Decimal)
                    .disabled(self.disabled)
                    .value(self.vis_value.clone())
                    .step(self.step)
                    .try_max(self.max)
                    .try_min(self.min)
                    .try_placeholder(self.placeholder.as_ref().map(ToString::to_string))
            })
            .map_msg_with(&self.msg_mapper)
            .add(&events.input);

        // spin_entry
        html::div()
            .id(self.id.clone())
            .class("spin-entry")
            .set(style.spin_entry)
            .set(&local_events.spin_entry)
            .map_msg_with(&self.msg_mapper)
            .add(&events.spin_entry)
            .add(vec![input, inc_btn, dec_btn])
    }
}

impl<PMsg: 'static> Config<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> SpinEntry<PMsg> {
        SpinEntry::init(self, orders)
    }
}

impl<PMsg: 'static> SpinEntry<PMsg> {
    fn try_set_value(&mut self, val: Option<f64>, _: &mut impl Orders<Msg>) {
        let val = match (val, self.min, self.max) {
            (Some(val), _, Some(max)) if val > max => Some(max),
            (Some(val), Some(min), _) if val < min => Some(min),
            _ => val,
        };
        if self.value != val {
            self.value = val;
            self.vis_value = val.map(|v| v.to_string()).unwrap_or_else(|| "".into());
        }
    }

    fn try_set_placeholder(&mut self, val: Option<f64>, _: &mut impl Orders<Msg>) {
        let val = match (val, self.min, self.max) {
            (Some(val), Some(min), Some(max)) if max >= val && val >= min => Some(val),
            (Some(val), _, Some(max)) if val > max => Some(max),
            (Some(val), Some(min), _) if val < min => Some(min),
            (val, _, _) => val,
        };
        self.placeholder = val;
    }

    fn try_set_max(&mut self, val: Option<f64>, orders: &mut impl Orders<Msg>) {
        match (val, self.min) {
            (Some(val), Some(min)) if val < min => {
                self.max = self.min;
                self.min = Some(val);
            }
            _ => self.max = val,
        }
        // re-calc step and placeholder again
        self.set_step(self.step, orders);
        self.try_set_value(self.value, orders);
        self.try_set_placeholder(self.placeholder, orders);
    }

    fn try_set_min(&mut self, val: Option<f64>, orders: &mut impl Orders<Msg>) {
        match (val, self.max) {
            (Some(val), Some(max)) if val > max => {
                self.min = self.max;
                self.max = Some(val);
            }
            _ => self.min = val,
        }
        // re-calc step and placeholder again
        self.set_step(self.step, orders);
        self.try_set_value(self.value, orders);
        self.try_set_placeholder(self.placeholder, orders);
    }

    fn set_step(&mut self, val: f64, _: &mut impl Orders<Msg>) {
        self.step = match (val, self.min, self.max) {
            (step, Some(min), Some(max)) if step.abs() > (min).abs() + (max).abs() => {
                (min).abs() + (max).abs()
            }
            _ => val.abs(),
        };
    }

    fn get_value_or_default(&self) -> f64 {
        match (self.value, self.min, self.max) {
            (Some(value), _, _) => value,
            (None, Some(min), Some(max)) if min <= 0.0 && max >= 0.0 => 0.0,
            (None, Some(min), None) if min <= 0.0 => 0.0,
            (None, Some(min), _) => min,
            (None, None, Some(max)) if max >= 0. => 0.,
            (None, None, Some(max)) => max,
            _ => 0.,
        }
    }

    fn input(&mut self, orders: &mut impl Orders<Msg>) {
        if let Some(input) = self.el_ref.get() {
            let value = input.value();
            // if value is empty then we set None to self.value
            if value.is_empty() {
                self.try_set_value(None, orders);
                return;
            }

            match value.as_str() {
                // these are the only allowed text when there is no number
                // in the input, we don't store these in self.value, but we sotre
                // them in self.vis_value
                "." => {
                    self.try_set_value(Some(0.0), orders);
                    self.vis_value = "0.".into();
                    self.el_ref.get_then(|el| el.set_value(&self.vis_value));
                }
                "-." => {
                    self.try_set_value(Some(-0.0), orders);
                    self.vis_value = "-0.".into();
                    self.el_ref.get_then(|el| el.set_value(&self.vis_value));
                }
                "-" => {
                    self.try_set_value(Some(-0.0), orders);
                    self.vis_value = value;
                    self.el_ref.get_then(|el| el.set_value(&self.vis_value));
                }
                _ => {
                    let v_f64 = value.parse::<f64>().ok();
                    if v_f64.is_some() {
                        self.try_set_value(v_f64, orders);
                        if self.value == v_f64 {
                            self.vis_value = value;
                        }
                    }
                }
            };
        }
    }
}

fn inc_btn_config() -> button::Config<Msg> {
    Button::config(Msg::increment_button)
        .label("+")
        .events(|| button::events().and_button(|conf| conf.click(|_| Msg::increment())))
}

fn dec_btn_config() -> button::Config<Msg> {
    Button::config(Msg::decrement_button)
        .label("-")
        .events(|| button::events().and_button(|conf| conf.click(|_| Msg::decrement())))
}

pub type ThemeStyler<'a> = Styler<SpinEntryLens<'a>, Style>;

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static>(val: <SpinEntry<PMsg> as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static>(
        val: impl Fn(<SpinEntry<PMsg> as Stylable>::Styler) -> <SpinEntry<PMsg> as Stylable>::Styler
            + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static>(val: Option<<SpinEntry<PMsg> as Stylable>::Styler>) -> Self {
        Msg::Styler(Rc::new(val))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn value(val: f64) -> Self {
        Msg::try_value(Some(val))
    }

    pub fn try_value(val: Option<f64>) -> Self {
        Msg::Value(val)
    }

    pub fn max(val: f64) -> Self {
        Msg::Max(Some(val))
    }

    pub fn try_max(val: Option<f64>) -> Self {
        Msg::Max(val)
    }

    pub fn min(val: f64) -> Self {
        Msg::Min(Some(val))
    }

    pub fn try_min(val: Option<f64>) -> Self {
        Msg::Min(val)
    }

    pub fn step(val: f64) -> Self {
        Msg::Step(val)
    }

    pub fn placeholder(val: f64) -> Self {
        Msg::try_placeholder(Some(val))
    }

    pub fn try_placeholder(val: Option<f64>) -> Self {
        Msg::Placeholder(val)
    }

    pub fn disabled(val: bool) -> Self {
        Msg::Disabled(val)
    }

    pub fn disable() -> Self {
        Self::disabled(true)
    }

    pub fn focus(val: bool) -> Self {
        Msg::Focus(val)
    }

    pub fn mouse_over(val: bool) -> Self {
        Msg::MouseOver(val)
    }

    pub fn increment() -> Self {
        Msg::Increment
    }

    pub fn decrement() -> Self {
        Msg::Decrement
    }

    fn input() -> Self {
        Msg::Input
    }

    fn increment_button(val: button::Msg) -> Self {
        Msg::IncrementButton(val)
    }

    fn decrement_button(val: button::Msg) -> Self {
        Msg::DecrementButton(val)
    }
}
