use crate::{button::ButtonLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

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
    el_ref: ElRef<web_sys::HtmlInputElement>,
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: Events<Msg>,
    #[rich(read)]
    #[element(props(default))]
    events: Events<PMsg>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    #[rich(read(copy))]
    #[element(props)]
    value: Option<f64>,
    // this value is an internal API, and shouldn't get exposed
    vis_value: String,
    #[rich(read(copy))]
    #[element(props)]
    max: Option<f64>,
    #[rich(read(copy))]
    #[element(props)]
    min: Option<f64>,
    #[rich(read(copy))]
    #[element(props(default = "1.0"))]
    step: f64,
    #[rich(read(copy))]
    #[element(props)]
    placeholder: Option<f64>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, props(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,

    // children elements
    #[rich(read)]
    #[element(theme_lens(nested))]
    increment_button: Button<Msg>,
    #[rich(read)]
    #[element(theme_lens(nested))]
    decrement_button: Button<Msg>,
}

pub enum Msg {
    SetTheme(Theme),
    SetValue(f64),
    TrySetValue(Option<f64>),
    SetMin(f64),
    TrySetMin(Option<f64>),
    SetMax(f64),
    TrySetMax(Option<f64>),
    SetStep(f64),
    SetPlaceholder(f64),
    TrySetPlaceholder(Option<f64>),
    SetDisabled(bool),
    Disable,
    Enable,
    SetFocus(bool),
    SetMouseOver(bool),
    Increment,
    Decrement,
    Input,
    IncrementButton(button::Msg),
    DecrementButton(button::Msg),
}

impl<PMsg: 'static> Element<PMsg> for SpinEntry<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default()
            .and_input(|conf| {
                conf.input(|_| Msg::Input)
                    .focus(|_| Msg::SetFocus(true))
                    .blur(|_| Msg::SetFocus(false))
            })
            .and_spin_entry(|conf| {
                conf.mouse_enter(|_| Msg::SetMouseOver(true))
                    .mouse_leave(|_| Msg::SetMouseOver(false))
            });

        let increment_button = Button::build(Msg::IncrementButton)
            .label("+")
            .events(button::Events::default().and_button(|conf| conf.click(|_| Msg::Increment)))
            .init(&mut orders);

        let decrement_button = Button::build(Msg::DecrementButton)
            .label("-")
            .events(button::Events::default().and_button(|conf| conf.click(|_| Msg::Decrement)))
            .init(&mut orders);

        let mut spin_entry = Self {
            el_ref: ElRef::default(),
            msg_mapper: props.msg_mapper,
            local_events,
            events: props.events,
            styler: props.styler,
            theme: props.theme,
            value: props.value,
            vis_value: props
                .value
                .map(|v| v.to_string())
                .unwrap_or_else(Default::default),
            max: props.max,
            min: props.min,
            step: props.step,
            placeholder: props.placeholder,
            disabled: props.disabled,
            focused: false,
            mouse_over: false,
            increment_button,
            decrement_button,
        };

        spin_entry.try_set_max(props.max, &mut orders);
        spin_entry.try_set_min(props.min, &mut orders);
        spin_entry.try_set_value(props.value, &mut orders);
        spin_entry.try_set_placeholder(props.placeholder, &mut orders);
        spin_entry
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetValue(val) => self.try_set_value(Some(val), &mut orders),
            Msg::TrySetValue(val) => self.try_set_value(val, &mut orders),
            Msg::SetMin(val) => self.try_set_min(Some(val), &mut orders),
            Msg::TrySetMin(val) => self.try_set_min(val, &mut orders),
            Msg::SetMax(val) => self.try_set_max(Some(val), &mut orders),
            Msg::TrySetMax(val) => self.try_set_max(val, &mut orders),
            Msg::SetStep(val) => self.set_step(val, &mut orders),
            Msg::SetPlaceholder(val) => self.try_set_placeholder(Some(val), &mut orders),
            Msg::TrySetPlaceholder(val) => self.try_set_placeholder(val, &mut orders),
            Msg::SetDisabled(val) => self.disabled = val,
            Msg::Disable => self.disabled = true,
            Msg::Enable => self.disabled = false,
            Msg::SetFocus(val) => self.focused = val,
            Msg::SetMouseOver(val) => self.mouse_over = val,
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

impl<PMsg: 'static> View for SpinEntry<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.spin_entry()(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for SpinEntry<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Self::Style) -> Self::Output {
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
            .set(&self.local_events.input)
            .set(style.input)
            .and_attributes(|conf| {
                conf.set_class("input")
                    .set_input_mode(att::InputMode::Decimal)
                    .set_disabled(self.disabled)
                    .set_value(self.vis_value.clone())
                    .set_step(self.step)
                    .try_set_max(self.max)
                    .try_set_min(self.min)
                    .try_set_placeholder(self.placeholder.as_ref().map(ToString::to_string))
            })
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.input);

        // spin_entry
        html::div()
            .class("spin-entry")
            .set(style.spin_entry)
            .set(&self.local_events.spin_entry)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.spin_entry)
            .add(vec![input, inc_btn, dec_btn])
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> SpinEntry<PMsg> {
        SpinEntry::init(self, orders)
    }
}

impl<PMsg: 'static> SpinEntry<PMsg> {
    pub fn and_events(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn try_set_styler(
        &mut self,
        val: Option<impl Into<Styler<PMsg>>>,
        _: &mut impl Orders<PMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler(&mut self, val: impl Into<Styler<PMsg>>, orders: &mut impl Orders<PMsg>) {
        self.try_set_styler(Some(val), orders)
    }

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

pub type Styler<PMsg> = theme::Styler<SpinEntry<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<SpinEntryLens<'a>, Style>;
