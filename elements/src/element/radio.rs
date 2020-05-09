use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(radio, button, label))]
pub struct Radio {
    // general element properties
    #[rich(read)]
    #[element(config)]
    id: Id,
    input_el_ref: ElRef<web_sys::HtmlInputElement>,
    label_el_ref: ElRef<web_sys::HtmlLabelElement>,
    #[rich(read)]
    #[element(config)]
    styler: Option<<Radio as Stylable>::Styler>,
    #[rich(read)]
    #[element(config(default))]
    theme: Theme,

    // radio element properties
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
    Styler(Option<<Radio as Stylable>::Styler>),
    UpdateStyler(UpdateStyler<Radio>),
    Theme(Theme),
    Label(Option<Cow<'static, str>>),
    Toggled(bool),
    Toggle,
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl Element for Radio {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, orders: &mut impl Orders<Msg>) -> Self {
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        Self {
            id: config.id.unwrap_or_else(Id::generate),
            input_el_ref: ElRef::default(),
            label_el_ref: ElRef::default(),
            theme: config.theme,
            styler: config.styler,
            label: config.label,
            disabled: config.disabled,
            toggled: config.toggled,
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
                    None => Some(val.update(self.theme.radio())),
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

impl Stylable for Radio {
    type Style = Style;
    type Styler = Styler<Self, Style>;

    fn styler(&self) -> Self::Styler {
        self.styler
            .clone()
            .unwrap_or_else(|| (|s: &Self| s.theme.radio().get(s)).into())
    }

    fn style(&self) -> Self::Style {
        self.styler().get(self)
    }
}

impl View<Node<Msg>> for Radio {
    fn view(&self) -> Node<Msg> {
        self.styled_view(self.style())
    }
}

impl StyledView<Node<Msg>> for Radio {
    fn styled_view(&self, style: Style) -> Node<Msg> {
        let Style {
            radio,
            button,
            label,
        } = style;

        let radio = html::input()
            .class("radio")
            .set(att::disabled(self.disabled))
            .set(att::checked(self.toggled))
            .set(att::Type::Radio)
            .set(radio)
            .el_ref(&self.input_el_ref)
            // add button if the radio is toggled
            .config_if(self.is_toggled(), |conf| {
                conf.add(html::div().class("button").set(button))
            })
            .on_focus(|_| Msg::focus(true))
            .on_blur(|_| Msg::focus(false))
            .on_mouse_enter(|_| Msg::mouse_over(true))
            .on_mouse_leave(|_| Msg::mouse_over(false))
            .on_click(|_| Msg::toggle());

        match self.label.as_ref() {
            None => radio.id(self.id.clone()),
            Some(lbl) => html::label()
                .id(self.id.clone())
                .class("label")
                .set(label)
                .add(radio)
                .add(lbl.clone())
                .el_ref(&self.label_el_ref)
                .on_mouse_enter(|_| Msg::mouse_over(true))
                .on_mouse_leave(|_| Msg::mouse_over(false)),
        }
    }
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Radio {
        Radio::init(self, orders)
    }
}

impl Msg {
    pub fn styler(val: <Radio as Stylable>::Styler) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler(val: impl Into<UpdateStyler<Radio>>) -> Self {
        Msg::UpdateStyler(val.into())
    }

    pub fn try_styler(val: Option<impl Into<<Radio as Stylable>::Styler>>) -> Self {
        Msg::Styler(val.map(|s| s.into()))
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
