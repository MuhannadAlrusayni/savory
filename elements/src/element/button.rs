use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(style(button, label(label::Style), icon(icon::Style)), events(button))]
pub struct Button {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[rich(read)]
    events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Button as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // button element properties
    #[rich(read)]
    #[element(config)]
    label: Option<Label>,
    #[rich(read)]
    #[element(config)]
    icon: Option<Icon<Msg>>,
    #[rich(read(copy))]
    #[element(config)]
    kind: Option<Kind>,
    #[rich(read(copy))]
    #[element(config(default = "false"))]
    ghost: bool,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default = "false"))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
}

pub enum Msg {
    Styler(Option<<Button as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Button>),
    Theme(Theme),
    Label(Option<Label>),
    Icon(Option<Icon<Msg>>),
    Kind(Option<Kind>),
    Ghost(bool),
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl Element for Button {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let events = || {
            Events::default().and_button(|conf| {
                conf.focus(|_| Msg::focus(true))
                    .blur(|_| Msg::focus(false))
                    .mouse_enter(|_| Msg::mouse_over(true))
                    .mouse_leave(|_| Msg::mouse_over(false))
            })
        };

        Button {
            id: config.id.unwrap_or_else(Id::generate),
            theme: config.theme,
            styler: config.styler,
            events: events.into(),
            label: config.label,
            icon: config.icon,
            kind: config.kind,
            ghost: config.ghost,
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
                    None => Some(val.update(self.theme.button())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Label(val) => self.label = val,
            Msg::Icon(val) => self.icon = val,
            Msg::Kind(val) => self.kind = val,
            Msg::Ghost(val) => self.ghost = val,
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
        }
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Button {
        Button::init(self, orders)
    }
}

impl Stylable for Button {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.button().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Button {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Button {
    fn styled_view(&self, style: Self::Style) -> Node<Msg> {
        let Style {
            button,
            label,
            icon,
        } = style;
        html::button()
            .class("button")
            .id(self.id.clone())
            .set(att::disabled(self.disabled))
            .set(&self.events.get().button)
            .set(button)
            .try_add(self.icon.as_ref().map(|el| el.styled_view(icon)))
            .try_add(self.label.as_ref().map(|el| el.styled_view(label)))
    }
}

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}

impl Msg {
    pub fn styler(val: <Button as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Button>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Button as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|v| v.into()))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn label(val: Label) -> Self {
        Self::try_label(Some(val))
    }

    pub fn try_label(val: Option<Label>) -> Self {
        Msg::Label(val)
    }

    pub fn icon(val: Icon<Msg>) -> Self {
        Self::try_icon(Some(val))
    }

    pub fn try_icon(val: Option<Icon<Msg>>) -> Self {
        Msg::Icon(val)
    }

    pub fn kind(val: Kind) -> Self {
        Self::try_kind(Some(val))
    }

    pub fn try_kind(val: Option<Kind>) -> Self {
        Msg::Kind(val)
    }

    pub fn ghost(val: bool) -> Self {
        Msg::Ghost(val)
    }

    pub fn ghost_on() -> Self {
        Self::ghost(true)
    }

    pub fn ghost_off() -> Self {
        Self::ghost(false)
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
}
