use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(checkbox, button, label), events(checkbox, button, label))]
pub struct Checkbox {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
    #[rich(read)]
    events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Checkbox as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // checkbox element properties
    #[rich(read(copy, rename = is_toggled))]
    #[element(config(default))]
    toggled: bool,
    #[rich(read)]
    #[element(config)]
    label: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(config(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
}

pub enum Msg {
    Styler(Option<<Checkbox as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Checkbox>),
    Theme(Theme),
    Label(Option<Cow<'static, str>>),
    Toggled(bool),
    Toggle,
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl Element for Checkbox {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let events = || {
            Events::default()
                .and_checkbox(|conf| {
                    conf.focus(|_| Msg::focus(true))
                        .blur(|_| Msg::focus(false))
                        .mouse_enter(|_| Msg::mouse_over(true))
                        .mouse_leave(|_| Msg::mouse_over(false))
                        .click(|_| Msg::toggle())
                })
                .and_label(|conf| {
                    conf.mouse_enter(|_| Msg::mouse_over(true))
                        .mouse_leave(|_| Msg::mouse_over(false))
                })
        };

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            input_el_ref: ElRef::default(),
            label_el_ref: ElRef::default(),
            theme: config.theme,
            events: events.into(),
            label: config.label,
            styler: config.styler,
            disabled: config.disabled,
            toggled: config.toggled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Styler(val) => self.styler = val,
            Msg::UpdateStyler(val) => {
                self.styler = match self.styler.clone() {
                    Some(styler) => Some(val.update(styler)),
                    None => Some(val.update(self.theme.checkbox())),
                }
            }
            Msg::Theme(val) => self.theme = val,
            Msg::Label(val) => self.label = val,
            Msg::Toggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
            Msg::Disabled(val) => self.disabled = val,
            Msg::Focus(val) => self.focused = val,
            Msg::MouseOver(val) => self.mouse_over = val,
        }
    }
}

impl Stylable for Checkbox {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.checkbox().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Checkbox {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Checkbox {
    // TODO: use container block and assign the element id for it
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let Style {
            checkbox,
            button,
            label,
        } = style;
        let events = self.events.get();

        let checkbox = html::input()
            .class("checbox")
            .set(att::disabled(self.disabled))
            .set(att::checked(self.toggled))
            .set(att::Type::Checkbox)
            .set(checkbox)
            .set(&events.checkbox)
            .el_ref(&self.input_el_ref)
            // add button if the checkbox is toggled
            .config_if(self.is_toggled(), |conf| {
                let button = html::div().class("button").set(button);
                conf.add(button)
            });

        match self.label.as_ref() {
            None => checkbox.id(self.id.clone()),
            Some(lbl) => html::label()
                .id(self.id.clone())
                .class("label")
                .set(label)
                .set(&events.label)
                .add(checkbox)
                .add(lbl.clone())
                .el_ref(&self.label_el_ref),
        }
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Checkbox {
        Checkbox::init(self, orders)
    }
}

impl Msg {
    pub fn styler(val: <Checkbox as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Checkbox>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Checkbox as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|v| v.into()))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn label(val: Cow<'static, str>) -> Self {
        Msg::try_label(Some(val))
    }

    pub fn try_label(val: Option<Cow<'static, str>>) -> Self {
        Msg::Label(val)
    }

    pub fn toggled(val: bool) -> Self {
        Msg::Toggled(val)
    }

    pub fn toggle_ond() -> Self {
        Msg::toggled(true)
    }

    pub fn toggle_off() -> Self {
        Msg::toggled(false)
    }

    pub fn toggle() -> Self {
        Msg::Toggle
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
