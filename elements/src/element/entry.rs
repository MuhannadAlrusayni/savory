use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(input, container), events(input, container))]
pub struct Entry {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    el_ref: ElRef<web_sys::HtmlInputElement>,
    #[rich(read)]
    events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Entry as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // entry element properties
    #[rich(read)]
    #[element(config)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy))]
    #[element(config)]
    max_length: Option<att::MaxLength>,
    #[rich(read)]
    #[element(config)]
    placeholder: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
}

pub enum Msg {
    Styler(Option<<Entry as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Entry>),
    Theme(Theme),
    Text(Option<Cow<'static, str>>),
    MaxLength(Option<att::MaxLength>),
    Placeholder(Option<Cow<'static, str>>),
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
    ResyncText,
}

impl Element for Entry {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let events = || {
            Events::default().and_input(|conf| {
                conf.focus(|_| Msg::focus(true))
                    .blur(|_| Msg::focus(false))
                    .mouse_enter(|_| Msg::mouse_over(true))
                    .mouse_leave(|_| Msg::mouse_over(false))
                    .input(|_| Msg::resync_text())
            })
        };

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            el_ref: ElRef::default(),
            events: events.into(),
            styler: config.styler,
            theme: config.theme,
            text: config.text,
            max_length: config.max_length,
            placeholder: config.placeholder,
            disabled: config.disabled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.entry())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Text(val) => self.text = val,
            Msg::MaxLength(val) => self.max_length = val,
            Msg::Placeholder(val) => self.placeholder = val,
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
            Msg::ResyncText => {
                if let Some(input) = self.el_ref.get() {
                    self.text = Some(input.value().into());
                }
            }
        }
    }
}

impl Stylable for Entry {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.entry().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Entry {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Entry {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let events = self.events.get();

        let input = html::input()
            .set(&events.input)
            .set(style.input)
            .and_attributes(|conf| {
                conf.class("input")
                    .disabled(self.disabled)
                    .try_value(self.text.clone())
                    .try_max_length(self.max_length)
                    .try_placeholder(self.placeholder.clone())
            });

        html::div()
            .id(self.id.clone())
            .class("entry")
            .set(style.container)
            .set(&events.container)
            .add(input)
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Entry {
        Entry::init(self, orders)
    }
}

impl Msg {
    pub fn styler(val: <Entry as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Entry>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Entry as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|v| v.into()))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn try_text(val: Option<Cow<'static, str>>) -> Self {
        Msg::Text(val)
    }

    pub fn text(val: Cow<'static, str>) -> Self {
        Msg::try_text(Some(val))
    }

    pub fn try_max_length(val: Option<impl Into<att::MaxLength>>) -> Self {
        Msg::MaxLength(val.map(|val| val.into()))
    }

    pub fn max_length(val: impl Into<att::MaxLength>) -> Self {
        Msg::try_max_length(Some(val))
    }

    pub fn try_placeholder(val: Option<Cow<'static, str>>) -> Self {
        Msg::Placeholder(val)
    }

    pub fn placeholder(val: Cow<'static, str>) -> Self {
        Msg::try_placeholder(Some(val))
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

    fn resync_text() -> Self {
        Msg::ResyncText
    }
}
