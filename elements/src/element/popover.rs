use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

// TODO: add placement property
#[derive(Clone, Rich, Element)]
#[element(style(panel, popover), events(panel, popover))]
pub struct Popover {
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Popover as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    #[rich(read)]
    #[element(config(nested, default = "Toggle::config().close_after(400)"))]
    toggle: Toggle,
    #[rich(read(copy))]
    #[element(config(default = "0"))]
    offset: i8,
}

pub enum Msg {
    Styler(Option<<Popover as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Popover>),
    Theme(Theme),
    Toggle(toggle::Msg),
    Offset(i8),
}

impl Element for Popover {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            styler: config.styler,
            theme: config.theme,
            toggle: config.toggle.init(&mut orders.proxy(Msg::Toggle)),
            offset: config.offset,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.popover())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Offset(val) => self.offset = val,
            Msg::Toggle(msg) => self.toggle.update(msg, &mut orders.proxy(Msg::Toggle)),
        }
    }
}

impl Stylable for Popover {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.popover().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Popover {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Popover {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let panel = html::div()
            .class("panel")
            .set(style.panel)
            // placeholder node
            .add(html::div().class("popover-content"));

        html::div()
            .id(self.id.clone())
            .class("popover")
            .set(style.popover)
            // placeholder node
            .add(html::div().class("popover-target"))
            .add(panel)
    }
}

impl Popover {
    pub fn is_toggled(&self) -> bool {
        self.toggle.is_toggled()
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Popover {
        Popover::init(self, orders)
    }
}

impl Msg {
    pub fn styler(val: <Popover as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Popover>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Popover as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|v| v.into()))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn toggled(val: bool) -> Self {
        Msg::Toggle(toggle::Msg::Toggled(val))
    }

    pub fn open() -> Self {
        Msg::toggled(true)
    }

    pub fn close() -> Self {
        Msg::toggled(false)
    }

    pub fn toggle() -> Self {
        Msg::Toggle(toggle::Msg::Toggle)
    }

    pub fn offset(val: i8) -> Self {
        Msg::Offset(val)
    }
}
