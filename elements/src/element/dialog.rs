use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

#[derive(Rich, Element)]
#[element(style(dialog, dialog_background), events(dialog, dialog_background))]
pub struct Dialog {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    #[rich(read)]
    events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Dialog as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // dialog element properties
    #[rich(read)]
    #[element(config(nested, default = "HeaderBar::config()"))]
    header_bar: HeaderBar,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_mouse_on_widget))]
    mouse_on_dialog: bool,
    #[rich(read)]
    #[element(config(nested, default = "Toggle::config().close_after(400)"))]
    toggle: Toggle,
}

pub enum Msg {
    Styler(Option<<Dialog as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Dialog>),
    Theme(Theme),
    MouseOnDialog(bool),
    ClickedOutSide,
    Toggle(toggle::Msg),
    HeaderBar(header_bar::Msg),
}

impl Element for Dialog {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let events = || {
            events()
                .and_dialog_background(|conf| conf.click(|_| Msg::clicked_out_side()))
                .and_dialog(|conf| {
                    conf.mouse_enter(|_| Msg::mouse_on_dialog(true))
                        .mouse_leave(|_| Msg::mouse_on_dialog(false))
                })
        };

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            events: events.into(),
            styler: config.styler,
            theme: config.theme,
            header_bar: config.header_bar.init(&mut orders.proxy(Msg::HeaderBar)),
            disabled: config.disabled,
            mouse_on_dialog: false,
            toggle: config.toggle.init(&mut orders.proxy(Msg::Toggle)),
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.dialog())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::MouseOnDialog(val) => self.mouse_on_dialog = val,
            Msg::ClickedOutSide => {
                if !self.mouse_on_dialog {
                    self.toggle.toggled(false, &mut orders.proxy(Msg::Toggle));
                }
            }
            Msg::Toggle(msg) => self.toggle.update(msg, &mut orders.proxy(Msg::Toggle)),
            Msg::HeaderBar(msg) => self
                .header_bar
                .update(msg, &mut orders.proxy(Msg::HeaderBar)),
        }
    }
}

impl Stylable for Dialog {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.dialog().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Dialog {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Dialog {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let events = self.events.get();

        let dialog = html::div()
            .class("dialog")
            .set(style.dialog)
            .add(
                self.header_bar
                    .view()
                    .map_msg(Msg::HeaderBar)
                    .for_class("button", |node| {
                        node.and_events(|conf| conf.click(|_| Msg::close()))
                    }),
            )
            // placeholder node
            .add(html::div().class("dialog-content"))
            .add(&events.dialog);

        html::div()
            .id(self.id.clone())
            .class("dialog-background")
            .set(style.dialog_background)
            .set(&events.dialog_background)
            .add(dialog)
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Dialog {
        Dialog::init(self, orders)
    }

    pub fn title(mut self, val: impl Into<Label>) -> Self {
        self.header_bar.title = Some(val.into());
        self
    }

    pub fn subtitle(mut self, val: impl Into<Label>) -> Self {
        self.header_bar.subtitle = Some(val.into());
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closing,
    Closed,
    Opening,
    Opened,
}

impl Msg {
    pub fn styler(val: <Dialog as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Dialog>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Dialog as Stylable>::Styler>>) -> Self {
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

    pub fn try_title(val: Option<impl Into<Label>>) -> Self {
        Msg::HeaderBar(header_bar::Msg::try_title(val))
    }

    pub fn title(val: impl Into<Label>) -> Self {
        Msg::try_title(Some(val))
    }

    pub fn try_subtitle(val: Option<impl Into<Label>>) -> Self {
        Msg::HeaderBar(header_bar::Msg::try_subtitle(val))
    }

    pub fn subtitle(val: impl Into<Label>) -> Self {
        Msg::try_subtitle(Some(val))
    }

    pub fn mouse_on_dialog(val: bool) -> Self {
        Msg::MouseOnDialog(val)
    }

    pub fn clicked_out_side() -> Self {
        Msg::ClickedOutSide
    }
}
