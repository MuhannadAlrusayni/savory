use crate::{
    css,
    el::{self},
    events::Events,
    model::Model,
    propertie::Size,
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
    #[rich(write(take, style = compose))]
    pub container: Events<Msg>,
    #[rich(write(take, style = compose))]
    pub input: Events<Msg>,
}

impl LocalEvents {
    pub fn remove_events(self) -> Self {
        Self::default()
    }
}

#[derive(Rich)]
pub struct ParentEvents<PMsg> {
    #[rich(write(take, style = compose))]
    pub container: Events<PMsg>,
    #[rich(write(take, style = compose))]
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
    #[rich(write(take, style = compose))]
    pub local_events: LocalEvents,
    #[rich(write(take, style = compose))]
    pub events: ParentEvents<PMsg>,
    #[rich(read(copy))]
    value: Option<f32>,
    #[rich(read(copy))]
    max: f32,
    #[rich(read(copy))]
    min: f32,
    #[rich(read(copy))]
    step: f32,
    #[rich(value_fns(take) = {
        small = Size::Small,
        medium = Size::Medium,
        large = Size::Large,
    })]
    pub size: Option<Size>,
    #[rich(write(take))]
    pub placeholder: Option<f32>,
    #[rich(write(take, style = compose))]
    pub style: UserStyle,
    #[rich(
        read(copy, rename = is_disabled),
    )]
    pub disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,

    // children elements
    #[rich(write(take, style = compose))]
    pub increment_button: el::Button<Msg>,
    #[rich(write(take, style = compose))]
    pub decrement_button: el::Button<Msg>,
}

impl<PMsg> SpinEntry<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        Self {
            el_ref: ElRef::default(),
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            local_events: LocalEvents::default()
                .input(|conf| {
                    conf.input(|_| Msg::Input)
                        .focus(|_| Msg::Focus)
                        .blur(|_| Msg::Blur)
                })
                .container(|conf| {
                    conf.mouse_enter(|_| Msg::MouseEnter)
                        .mouse_leave(|_| Msg::MouseLeave)
                }),
            events: ParentEvents::default(),
            value: None,
            max: 10.,
            min: 0.,
            step: 1.,
            placeholder: None,
            size: None,
            style: UserStyle::default(),
            disabled: false,
            focus: false,
            mouse_over: false,
            increment_button: el::Button::new(Msg::IncrementButton)
                .events(|conf| conf.click(|_| Msg::Increment)),
            decrement_button: el::Button::new(Msg::DecrementButton)
                .events(|conf| conf.click(|_| Msg::Decrement)),
        }
    }

    pub fn default_value(&self) -> f32 {
        self.min
    }

    pub fn value_or_default(&self) -> f32 {
        self.value.unwrap_or_else(|| self.default_value())
    }

    pub fn max(mut self, max: f32) -> Self {
        if max > self.min {
            self.max = max;
        } else {
            self.max = self.min;
            self.min = max;
        }
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        if min < self.max {
            self.min = min;
        } else {
            self.min = self.max;
            self.max = min;
        }
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        let range = self.min - self.max;
        self.step = if step > range { range } else { step };
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = match value {
            x if x > self.max => Some(self.max),
            x if x < self.min => Some(self.min),
            x => Some(x),
        };
        self
    }

    pub fn enable(mut self) -> Self {
        self.disabled = false;
        self.increment_button(|conf| conf.enable())
            .decrement_button(|conf| conf.enable())
    }

    pub fn disable(mut self) -> Self {
        self.disabled = true;
        self.increment_button(|conf| conf.disable())
            .decrement_button(|conf| conf.disable())
    }

    fn increment(&mut self) {
        let value = self.value_or_default();
        if value < self.max {
            let value = if self.max < value + self.step {
                self.max
            } else {
                value + self.step
            };
            self.value = Some(value);
        }
    }

    fn decrement(&mut self) {
        let value = self.value_or_default();
        if value > self.min {
            let value = if self.min > value - self.step {
                self.min
            } else {
                value - self.step
            };
            self.value = Some(value);
        }
    }

    fn handle_input(&mut self) {
        log!(self.el_ref.get());
        if let Some(input) = self.el_ref.get() {
            let value = input.value();
            // if value is empty then we set None to self.value
            if value.is_empty() {
                log!("value is empty");
                self.value = None;
            } else {
                // parse value to f32
                match value.parse::<f32>().ok() {
                    // check if value in accpeted range
                    Some(value) if value >= self.min && value <= self.max => {
                        self.value = Some(value)
                    }
                    // remove the input and set self.value as the value for input
                    _ => input.set_value(&self.value.map(|v| v.to_string()).unwrap_or("".into())),
                }
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
    #[rich(write(take, style = compose))]
    pub container: css::Style,
    #[rich(write(take, style = compose))]
    pub input: css::Style,
    #[rich(write(take, style = compose))]
    pub buttons_container: el::flexbox::Style,
    #[rich(write(take, style = compose))]
    pub increment_item: el::flexbox::ItemStyle,
    #[rich(write(take, style = compose))]
    pub decrement_item: el::flexbox::ItemStyle,
    #[rich(write(take, style = compose))]
    pub increment_button: el::button::Style,
    #[rich(write(take, style = compose))]
    pub decrement_button: el::button::Style,
    #[rich(write(take))]
    pub increment_icon: Option<el::Icon<Msg>>,
    #[rich(write(take))]
    pub decrement_icon: Option<el::Icon<Msg>>,
}

/// This style returned by the Theme and consumed by render function, thus the
/// icons must be returned by the theme
#[derive(Clone, Rich)]
pub struct Style {
    #[rich(write(take, style = compose))]
    pub container: css::Style,
    #[rich(write(take, style = compose))]
    pub input: css::Style,
    #[rich(write(take, style = compose))]
    pub buttons_container: el::flexbox::Style,
    #[rich(write(take, style = compose))]
    pub increment_item: el::flexbox::ItemStyle,
    #[rich(write(take, style = compose))]
    pub decrement_item: el::flexbox::ItemStyle,
    #[rich(write(take, style = compose))]
    pub increment_button: el::button::Style,
    #[rich(write(take, style = compose))]
    pub decrement_button: el::button::Style,
    // FIXME: should I use SvgIcon insted of Icon ?
    #[rich(write(take))]
    pub increment_icon: el::Icon<Msg>,
    #[rich(write(take))]
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
            .add(|item| {
                item.content(nodes![inc_btn])
                    .render_with_style(theme, increment_item)
            })
            .add(|item| {
                item.content(nodes![dec_btn])
                    .render_with_style(theme, decrement_item)
            })
            .render_with_style(theme, buttons_container)
            .map_msg(move |msg| (msg_mapper.clone())(msg));

        // input
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut input = input![
            el_ref(&self.el_ref),
            self.local_events.input.clone(),
            input,
            attrs![
                At::Value => self.value.map(|v| v.to_string()).unwrap_or("".into()),
                // TODO:
                //   At::Max => self.max,
                //   At::Min => self.min,
                //   At::Step => self.step,
                //   At::Placeholder => self.placeholder,
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
