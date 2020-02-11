use crate::{
    css,
    el::{self},
    events::Events,
    model::Model,
    propertie::{Shape, Size},
    render::Render,
    theme::Theme,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::{borrow::Cow, rc::Rc};

#[derive(Debug, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Increment,
    Decrement,
    Input(String),
    KeyDown(web_sys::KeyboardEvent),
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
    pub fn remove_events(mut self) -> Self {
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
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            local_events: LocalEvents::default()
                .input(|conf| {
                    conf.key_down(Msg::KeyDown)
                        .input(Msg::Input)
                        .focus(|_| Msg::Focus)
                        .blur(|_| Msg::Blur)
                })
                .container(|conf| {
                    conf.mouse_enter(|_| Msg::MouseEnter)
                        .mouse_leave(|_| Msg::MouseLeave)
                }),
            events: ParentEvents::default(),
            value: None,
            max: 0.,
            min: 10.,
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

    fn handle_increment(&mut self) {
        let value = self.value.unwrap_or(self.min);
        if value < self.max {
            self.value = Some(value + self.step);
        }
    }

    fn handle_decrement(&mut self) {
        let value = self.value.unwrap_or(self.min);
        if value < self.max {
            self.value = Some(value + self.step);
        }
    }

    fn handle_key_down(&mut self, event: web_sys::KeyboardEvent) {
        log!(event.key());
        if event.key().chars().any(|c| !c.is_ascii_digit() && c != '.') {
            let raw_event: &web_sys::Event = event.as_ref();
            raw_event.prevent_default();
        }
    }

    // FIXME: this should call prevent_default() on the input event somehow
    fn handle_input(&mut self, input: String) {
        if !input.chars().any(|c| !c.is_ascii_digit() && c != '.') {
            if let Ok(value) = input.parse::<f32>() {
                self.value = Some(value)
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
            Msg::Increment => self.handle_increment(),
            Msg::Decrement => self.handle_decrement(),
            Msg::Input(input) => self.handle_input(input),
            Msg::KeyDown(event) => self.handle_key_down(event),
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
            self.local_events.input.clone(),
            input,
            // TODO: At::Max, At::Min ..etc
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
