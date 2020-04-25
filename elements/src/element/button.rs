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
    SetEventsStore(Rc<dyn Any>),
    // Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>
    AndEventsStore(Rc<dyn Any>),
    // Styler<PMsg>
    SetStyler(Rc<dyn Any>),
    // Option<Styler>
    TrySetStyler(Rc<dyn Any>),
    // Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>
    AndStyler(Rc<dyn Any>),
    SetTheme(Theme),
    SetLabel(Label<Msg>),
    TrySetLabel(Option<Label<Msg>>),
    SetIcon(Icon<Msg>),
    TrySetIcon(Option<Icon<Msg>>),
    SetKind(Kind),
    TrySetKind(Option<Kind>),
    SetGhost(bool),
    SetGhostOn,
    SetGhostOff,
    SetDisabled(bool),
    Disable,
    Enable,
    SetFocus(bool),
    SetMouseOver(bool),
}

impl<PMsg: 'static> Element<PMsg> for Button<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = EventsStore::new(|| {
            Events::default().and_button(|conf| {
                conf.focus(|_| Msg::SetFocus(true))
                    .blur(|_| Msg::SetFocus(false))
                    .mouse_enter(|_| Msg::SetMouseOver(true))
                    .mouse_leave(|_| Msg::SetMouseOver(false))
            })
        });

        Button {
            theme: props.theme,
            styler: props.styler,
            msg_mapper: props.msg_mapper,
            local_events,
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
            Msg::SetEventsStore(val) => {
                if let Ok(val) = val.downcast::<EventsStore<Events<PMsg>>>() {
                    self.events = val.into();
                }
            }
            Msg::AndEventsStore(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(EventsStore<Events<PMsg>>) -> EventsStore<Events<PMsg>>>>() {
                    self.events = val(self.events.clone());
                }
            }
            Msg::SetStyler(val) => {
                if let Ok(val) = val.downcast::<Styler<PMsg>>() {
                    self.styler = Some(val.into());
                }
            }
            Msg::TrySetStyler(val) => {
                if let Ok(val) = val.downcast::<Option<Styler<PMsg>>>() {
                    self.styler = val.as_ref().clone();
                }
            }
            Msg::AndStyler(val) => {
                if let Ok(val) = val.downcast::<Box<dyn Fn(Styler<PMsg>) -> Styler<PMsg>>>() {
                    self.styler = Some(val(self
                        .styler
                        .clone()
                        .unwrap_or_else(|| Styler::default())));
                }
            }
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetLabel(val) => self.label = Some(val),
            Msg::TrySetLabel(val) => self.label = val,
            Msg::SetIcon(val) => self.icon = Some(val),
            Msg::TrySetIcon(val) => self.icon = val,
            Msg::SetKind(val) => self.kind = Some(val),
            Msg::TrySetKind(val) => self.kind = val,
            Msg::SetGhost(val) => self.ghost = val,
            Msg::SetGhostOn => self.ghost = true,
            Msg::SetGhostOff => self.ghost = false,
            Msg::SetDisabled(val) => self.disabled = val,
            Msg::Disable => self.disabled = true,
            Msg::Enable => self.disabled = false,
            Msg::SetFocus(val) => self.focused = val,
            Msg::SetMouseOver(val) => self.mouse_over = val,
        }
    }
}

impl<PMsg: 'static> View for Button<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.button()(&self.theme_lens())),
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

pub type Styler<PMsg> = theme::Styler<Button<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<ButtonLens<'a>, Style>;
