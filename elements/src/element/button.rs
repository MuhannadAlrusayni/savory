use crate::{icon::IconLens, label::LabelLens, prelude::*};
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::{any::Any, rc::Rc};

#[derive(Rich, Element)]
#[element(style(button, label(label::Style), icon(icon::Style)), events(button))]
pub struct Button<PMsg> {
    // general element properties
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: EventsStore<Events<Msg>>,
    #[rich(read)]
    #[element(props(default))]
    events: EventsStore<Events<PMsg>>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    // button element properties
    #[rich(read)]
    #[element(theme_lens(nested), props)]
    label: Option<Label<Msg>>,
    #[rich(read)]
    #[element(theme_lens(nested), props)]
    icon: Option<Icon<Msg>>,
    #[rich(read(copy))]
    #[element(theme_lens, props)]
    kind: Option<Kind>,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "false"))]
    ghost: bool,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, props(default = "false"))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,
}

pub enum Msg {
    // EventsStore<Events<PMsg>>
    EventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    UpdateEventsStore(Rc<dyn Any>),
    // Option<Styler<PMsg>>
    Styler(Rc<dyn Any>),
    // Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>
    UpdateStyler(Rc<dyn Any>),
    Theme(Theme),
    Label(Option<Label<Msg>>),
    Icon(Option<Icon<Msg>>),
    Kind(Option<Kind>),
    Ghost(bool),
    Disabled(bool),
    Focus(bool),
    MouseOver(bool),
}

impl<PMsg: 'static> Element<PMsg> for Button<PMsg> {
    type Message = Msg;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::theme(theme.0));

        let local_events = || {
            Events::default().and_button(|conf| {
                conf.focus(|_| Msg::focus(true))
                    .blur(|_| Msg::focus(false))
                    .mouse_enter(|_| Msg::mouse_over(true))
                    .mouse_leave(|_| Msg::mouse_over(false))
            })
        };

        Button {
            theme: props.theme,
            styler: props.styler,
            msg_mapper: props.msg_mapper,
            local_events: local_events.into(),
            events: props.events,
            label: props.label,
            icon: props.icon,
            kind: props.kind,
            ghost: props.ghost,
            disabled: props.disabled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg>) {
        match msg {
            Msg::EventsStore(val) => {
                if let Ok(val) = val.downcast::<EventsStore<Events<PMsg>>>() {
                    self.events = val.into();
                }
            }
            Msg::UpdateEventsStore(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>>() {
                    self.events = val(self.events.clone());
                }
            }
            Msg::Styler(val) => {
                if let Ok(val) = val.downcast::<Option<Styler<PMsg>>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::UpdateStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>>() {
                    self.styler = Some(val(self.styler.clone().unwrap_or_else(Styler::default)));
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

impl<PMsg: 'static> View for Button<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler.get(self))
                .unwrap_or_else(|| self.theme.button().get(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Button<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Self::Style) -> Self::Output {
        let Style {
            button,
            label,
            icon,
        } = style;
        html::button()
            .class("button")
            .set(att::disabled(self.disabled))
            .set(&self.local_events.get().button)
            .set(button)
            .try_add(self.icon.as_ref().map(|el| el.styled_view(icon)))
            .try_add(self.label.as_ref().map(|el| el.styled_view(label)))
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.get().button)
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Button<PMsg> {
        Button::init(self, orders)
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

pub fn events<PMsg>() -> Events<PMsg> {
    Events::default()
}

pub fn style() -> Style {
    Style::default()
}

pub type Styler<PMsg> = theme::Styler<Button<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<ButtonLens<'a>, Style>;

impl Msg {
    pub fn events_store<PMsg: 'static>(val: EventsStore<PMsg>) -> Self {
        Msg::EventsStore(Rc::new(val))
    }

    pub fn update_events_store<PMsg: 'static>(
        val: impl Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>> + 'static,
    ) -> Self {
        Msg::UpdateEventsStore(Rc::new(val))
    }

    pub fn styler<PMsg: 'static>(val: Styler<PMsg>) -> Self {
        Msg::try_styler(Some(val))
    }

    pub fn update_styler<PMsg: 'static>(
        val: impl Fn(Styler<PMsg>) -> Styler<PMsg> + 'static,
    ) -> Self {
        Msg::UpdateStyler(Rc::new(val))
    }

    pub fn try_styler<PMsg: 'static>(val: Option<Styler<PMsg>>) -> Self {
        Msg::Styler(Rc::new(val))
    }

    pub fn theme(val: Theme) -> Self {
        Msg::Theme(val)
    }

    pub fn label(val: Label<Msg>) -> Self {
        Self::try_label(Some(val))
    }

    pub fn try_label(val: Option<Label<Msg>>) -> Self {
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
