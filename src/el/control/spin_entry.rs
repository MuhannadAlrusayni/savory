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
    Entry(el::entry::Msg),
    IncBtn(el::button::Msg),
    DecBtn(el::button::Msg),
    Increment,
    Decrement,
}

#[derive(Clone)]
pub enum Formatter {
    Integer,
    Float(i8),
    Custom(Rc<dyn Fn(f32) -> Cow<'static, str>>),
}

#[derive(Rich)]
pub struct SpinEntry<PMsg> {
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    #[rich(write(take))]
    value: Option<f32>,
    #[rich(write(take))]
    default_value: Option<f32>,
    #[rich(write(take))]
    max: Option<f32>,
    #[rich(write(take))]
    min: Option<f32>,
    #[rich(write(take))]
    step: Option<f32>,
    #[rich(write(take))]
    disabled: bool,
    #[rich(value_fns(take) = {
        integer = Formatter::Integer,
        float = Formatter::Float(2),
    })]
    formatter: Formatter,
    #[rich(write(take, style = compose))]
    pub style: UserStyle,
    entry: el::Entry<Msg>,
    inc_btn: el::Button<Msg>,
    dec_btn: el::Button<Msg>,
}

impl<PMsg> SpinEntry<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        Self {
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            events: Events::default(),
            value: None,
            default_value: None,
            max: None,
            min: None,
            step: None,
            disabled: false,
            formatter: Formatter::Integer,
            style: UserStyle::default(),
            entry: el::Entry::new(Msg::Entry),
            inc_btn: el::Button::new(Msg::IncBtn).events(|conf| conf.click(|_| Msg::Increment)),
            dec_btn: el::Button::new(Msg::DecBtn).events(|conf| conf.click(|_| Msg::Decrement)),
        }
    }

    pub fn custom_format(mut self, formatter: impl Fn(f32) -> Cow<'static, str> + 'static) -> Self {
        self.formatter = Formatter::Custom(Rc::new(formatter));
        self
    }

    pub fn float_fraction(mut self, length: i8) -> Self {
        self.formatter = Formatter::Float(length);
        self
    }

    fn handle_increment(&mut self) {
        todo!()
    }

    fn handel_decrement(&mut self) {
        todo!()
    }
}

impl<GMsg: 'static, PMsg: 'static> Model<PMsg, GMsg> for SpinEntry<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut orders = orders.proxy(move |msg| (msg_mapper.clone())(msg));

        match msg {
            Msg::Increment => self.handle_increment(),
            Msg::Decrement => self.handel_decrement(),
            Msg::Entry(msg) => self.entry.update(msg, &mut orders),
            Msg::IncBtn(msg) => self.inc_btn.update(msg, &mut orders),
            Msg::DecBtn(msg) => self.dec_btn.update(msg, &mut orders),
        }
    }
}

/// This style used by users when they want to change styles of SpinEntry
#[derive(Clone, Default, Rich)]
pub struct UserStyle {
    #[rich(write(take, style = compose))]
    entry: el::entry::Style,
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
    pub entry: el::entry::Style,
    #[rich(write(take, style = compose))]
    pub increment_button: el::button::Style,
    #[rich(write(take, style = compose))]
    pub decrement_button: el::button::Style,
    #[rich(write(take))]
    pub increment_icon: el::Icon<Msg>,
    #[rich(write(take))]
    pub decrement_icon: el::Icon<Msg>,
}

impl<PMsg: 'static> Render<PMsg> for SpinEntry<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn render(&self, theme: &impl Theme) -> Self::View {
        let style = theme.spin_entry(self);

        todo!()
    }
}
