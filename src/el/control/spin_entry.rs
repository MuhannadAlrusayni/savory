use crate::{
    css,
    el::{self},
    events::Events,
    model::Model,
    render::Render,
    theme::Theme,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Increment,
    Decrement,
    Input,
    IncrementButton(el::button::Msg),
    DecrementButton(el::button::Msg),
}

#[derive(Default, Rich)]
pub struct LocalEvents {
    #[rich(write(style = compose))]
    pub container: Events<Msg>,
    #[rich(write(style = compose))]
    pub input: Events<Msg>,
}

impl LocalEvents {
    pub fn remove_events(self) -> Self {
        Self::default()
    }
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(style = compose))]
    pub container: Events<PMsg>,
    #[rich(write(style = compose))]
    pub input: Events<PMsg>,
}

impl<PMsg> Default for ParentEvents<PMsg> {
    fn default() -> Self {
        Self {
            container: Events::default(),
            input: Events::default(),
        }
    }
}

// TODO: add way to accept custom format (e.g. `100%`, `45$`)
#[derive(Rich)]
pub struct SpinEntry<PMsg> {
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    #[rich(
        read(
            /// Return reference for the local events.
        ),
        write(
            /// add and remove local events, changes will be applied to the DOM
            /// after next render
            style = compose
        ))]
    local_events: LocalEvents,
    #[rich(
        read(
            /// Return reference for the storde parent events.
        ),
        write(
            /// add and remove parent events, changes will be applied to the DOM
            /// after next render
            style = compose
        ))]
    events: ParentEvents<PMsg>,
    #[rich(read(
        /// Return the current value if there is any.
        copy
    ))]
    value: Option<f32>,
    // this is internal API, and shouldn't get exposed
    vis_value: String,
    #[rich(read(
        /// Return the max value if there is any.
        copy
    ))]
    max: Option<f32>,
    #[rich(read(
        /// Return the min value if there is any.
        copy
    ))]
    min: Option<f32>,
    #[rich(read(
        /// Return the step value.
        copy
    ))]
    step: f32,
    #[rich(read(
        /// Return the used placeholder if there is any.
        copy
    ))]
    placeholder: Option<f32>,
    #[rich(
        read(
            /// Return reference to the user style.
        ),
        write(
            /// User style, used to customize the spin entry if you need to,
            /// changes on this style will overrides the theme style.
            style = compose
        ))]
    style: UserStyle,
    #[rich(read(
        /// Return `true` if spin entry is disabled
        copy, rename = is_disabled
    ),)]
    disabled: bool,
    #[rich(read(
        /// Return `true` if spin entry is foucsed
        copy, rename = is_focused
    ))]
    focus: bool,
    #[rich(read(
        /// Return `true` if mouse is over
        copy, rename = is_mouse_over
    ))]
    mouse_over: bool,

    // children elements
    #[rich(write(
        /// Customize the increment button. Note by doing so you may override
        /// the default behavior for this button.
        style = compose
    ))]
    pub increment_button: el::Button<Msg>,
    #[rich(write(
        /// Customize the decrement button. Note by doing so you may override
        /// the default behavior for this button.
        style = compose
    ))]
    pub decrement_button: el::Button<Msg>,
}

impl<PMsg> SpinEntry<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        let mut local_events = LocalEvents::default();
        local_events
            .input(|conf| {
                conf.input(|_| Msg::Input)
                    .focus(|_| Msg::Focus)
                    .blur(|_| Msg::Blur)
            })
            .container(|conf| {
                conf.mouse_enter(|_| Msg::MouseEnter)
                    .mouse_leave(|_| Msg::MouseLeave)
            });

        let mut increment_button = el::Button::new(Msg::IncrementButton);
        increment_button.events(|conf| conf.click(|_| Msg::Increment));

        let mut decrement_button = el::Button::new(Msg::DecrementButton);
        decrement_button.events(|conf| conf.click(|_| Msg::Decrement));

        Self {
            el_ref: ElRef::default(),
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            local_events,
            events: ParentEvents::default(),
            value: None,
            vis_value: "".into(),
            max: None,
            min: None,
            step: 1.,
            placeholder: None,
            style: UserStyle::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
            increment_button,
            decrement_button,
        }
    }

    pub fn max(&mut self, max: f32) -> &mut Self {
        match (max, self.min) {
            (max, Some(min)) if max < min => {
                self.max = self.min;
                self.min = Some(max);
            }
            _ => self.max = Some(max),
        }
        // re-calc step and placeholder again
        self.step(self.step);
        if let Some(placeholder) = self.placeholder {
            self.placeholder(placeholder);
        }
        self
    }

    pub fn min(&mut self, min: f32) -> &mut Self {
        match (min, self.max) {
            (min, Some(max)) if min > max => {
                self.min = self.max;
                self.max = Some(min);
            }
            _ => self.min = Some(min),
        }
        // re-calc step and placeholder again
        self.step(self.step);
        if let Some(placeholder) = self.placeholder {
            self.placeholder(placeholder);
        }
        self
    }

    pub fn step(&mut self, step: f32) -> &mut Self {
        self.step = match (step, self.min, self.max) {
            (step, Some(min), Some(max)) if step.abs() > (min).abs() + (max).abs() => {
                (min).abs() + (max).abs()
            }
            _ => step.abs(),
        };
        self
    }

    pub fn placeholder(&mut self, value: impl Into<f32>) -> &mut Self {
        let placeholder = match (value.into(), self.min, self.max) {
            (value, _, Some(max)) if value > max => max,
            (value, Some(min), _) if value < min => min,
            (value, _, _) => value,
        };
        self.placeholder = Some(placeholder);
        if let Some(input) = self.el_ref.get() {
            input.set_placeholder(&placeholder.to_string());
        }
        self
    }

    pub fn value(&mut self, value: f32) -> &mut Self {
        let value = match (value, self.min, self.max) {
            (value, _, Some(max)) if value > max => max,
            (value, Some(min), _) if value < min => min,
            _ => value,
        };
        self.value = Some(value);
        self.vis_value = value.to_string();
        if let Some(input) = self.el_ref.get() {
            input.set_value(&self.vis_value);
        }
        self
    }

    pub fn unset_value(&mut self) -> &mut Self {
        self.value = None;
        self.vis_value = "".into();
        self
    }

    pub fn enable(&mut self) -> &mut Self {
        self.disabled = false;
        self.increment_button(|conf| conf.enable())
            .decrement_button(|conf| conf.enable())
    }

    pub fn disable(&mut self) -> &mut Self {
        self.disabled = true;
        self.increment_button(|conf| conf.disable())
            .decrement_button(|conf| conf.disable())
    }

    fn calc_reasonable_value(&self) -> f32 {
        match (self.value, self.min, self.max) {
            (Some(value), _, _) => value,
            (None, Some(min), Some(max)) => (min + max) * 0.5,
            (None, Some(min), None) if min < 0. => 0.,
            (None, Some(min), None) => min,
            (None, None, Some(max)) if max > 0. => 0.,
            (None, None, Some(max)) => max,
            _ => 0.,
        }
    }

    pub fn increment(&mut self) {
        self.value(self.calc_reasonable_value() + self.step);
    }

    pub fn decrement(&mut self) {
        self.value(self.calc_reasonable_value() - self.step);
    }

    fn handle_input(&mut self) {
        if let Some(input) = self.el_ref.get() {
            let value = input.value();
            // if value is empty then we set None to self.value
            if value.is_empty() {
                self.unset_value();
            } else {
                match value.as_str() {
                    // these are the only allowed text when there is no number
                    // in the input, we don't store these in self.value, but we sotre
                    // them in self.vis_value
                    "." => {
                        self.vis_value = "0.".into();
                        input.set_value(&self.vis_value);
                    }
                    "-." => {
                        self.vis_value = "-0.".into();
                        input.set_value(&self.vis_value);
                    }
                    "-" => {
                        self.vis_value = value;
                        input.set_value(&self.vis_value);
                    }
                    _ => {
                        // parse value to f32
                        if let Some(v_f32) = value.parse::<f32>().ok() {
                            self.value(v_f32);
                            // check if value is eq to v_f32
                            if self.value == Some(v_f32) && value.ends_with(".") {
                                // use the input value as vis_value if so, this
                                // helps keep the last dot when user enter e.g.
                                // `5.`, without this, the input will be
                                // converted to `5`
                                self.vis_value = value;
                                input.set_value(&self.vis_value);
                            }
                        }
                    }
                };
            }
        }
    }
}

impl<GMsg: 'static, PMsg: 'static> Model<PMsg, GMsg> for SpinEntry<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut orders = orders.proxy(move |msg| (msg_mapper.clone())(msg));

        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Increment => self.increment(),
            Msg::Decrement => self.decrement(),
            Msg::Input => self.handle_input(),
            Msg::IncrementButton(msg) => self.increment_button.update(msg, &mut orders),
            Msg::DecrementButton(msg) => self.decrement_button.update(msg, &mut orders),
        }
    }
}

/// This style used by users when they want to change styles of SpinEntry
#[derive(Clone, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
    pub input: css::Style,
    #[rich(write(style = compose))]
    pub buttons_container: el::flexbox::Style,
    #[rich(write(style = compose))]
    pub increment_item: el::flexbox::ItemStyle,
    #[rich(write(style = compose))]
    pub decrement_item: el::flexbox::ItemStyle,
    #[rich(write(style = compose))]
    pub increment_button: el::button::Style,
    #[rich(write(style = compose))]
    pub decrement_button: el::button::Style,
    #[rich(write)]
    pub increment_icon: Option<el::Icon<Msg>>,
    #[rich(write)]
    pub decrement_icon: Option<el::Icon<Msg>>,
}

/// This style returned by the Theme and consumed by render function, thus the
/// icons must be returned by the theme
#[derive(Clone, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub container: css::Style,
    #[rich(write(style = compose))]
    pub input: css::Style,
    #[rich(write(style = compose))]
    pub buttons_container: el::flexbox::Style,
    #[rich(write(style = compose))]
    pub increment_item: el::flexbox::ItemStyle,
    #[rich(write(style = compose))]
    pub decrement_item: el::flexbox::ItemStyle,
    #[rich(write(style = compose))]
    pub increment_button: el::button::Style,
    #[rich(write(style = compose))]
    pub decrement_button: el::button::Style,
    // FIXME: should I use SvgIcon insted of Icon ?
    #[rich(write)]
    pub increment_icon: el::Icon<Msg>,
    #[rich(write)]
    pub decrement_icon: el::Icon<Msg>,
}

impl<PMsg: 'static> Render<PMsg> for SpinEntry<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.spin_entry(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        let Style {
            container,
            input,
            buttons_container,
            increment_item,
            decrement_item,
            increment_button,
            decrement_button,
            increment_icon,
            decrement_icon,
        } = style;

        let mut inc_btn = self
            .increment_button
            .render_with_style(theme, increment_button);
        let mut dec_btn = self
            .decrement_button
            .render_with_style(theme, decrement_button);

        // FIXME: try to use better way to add icons to the buttons, this way is
        // pretty hacky and wouldn't work if el::Button::render(theme) return
        // nasted nodes
        inc_btn.add_child(increment_icon.render(theme));
        dec_btn.add_child(decrement_icon.render(theme));

        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let btns_container = el::Flexbox::new()
            .add(el::Flexbox::item_with(nodes![inc_btn]).render_with_style(theme, increment_item))
            .add(el::Flexbox::item_with(nodes![dec_btn]).render_with_style(theme, decrement_item))
            .render_with_style(theme, buttons_container)
            .map_msg(move |msg| (msg_mapper.clone())(msg));

        fn att_map(att: Option<impl ToString>) -> AtValue {
            att.map(|a| a.to_string().into()).unwrap_or(AtValue::None)
        }
        // input
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut input = input![
            el_ref(&self.el_ref),
            self.local_events.input.clone(),
            input,
            attrs![
                At::Value => self.vis_value,
                At::Max => att_map(self.max),
                At::Min => att_map(self.min),
                At::Step => self.step,
                At::Placeholder => att_map(self.placeholder),
                At::Disabled => self.disabled.as_at_value(),
            ]
        ]
        .map_msg(move |msg| (msg_mapper.clone())(msg));

        for event in self.events.input.events.clone().into_iter() {
            input.add_listener(event);
        }

        // container
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut container = div![self.local_events.container.clone(), container,]
            .map_msg(move |msg| (msg_mapper.clone())(msg));

        for event in self.events.container.events.clone().into_iter() {
            container.add_listener(event);
        }

        container.add_child(input).add_child(btns_container);
        container
    }
}
