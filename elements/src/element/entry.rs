use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
#[element(style(input, container), events(input, container))]
pub struct Entry<PMsg> {
    // general element properties
    el_ref: ElRef<web_sys::HtmlInputElement>,
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    local_events: Events<Msg>,
    #[rich(read)]
    #[element(props(default))]
    events: Events<PMsg>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    // entry element properties
    #[rich(read)]
    #[element(props)]
    text: Option<Cow<'static, str>>,
    #[rich(read(copy))]
    #[element(theme_lens, props)]
    max_length: Option<att::MaxLength>,
    #[rich(read)]
    #[element(theme_lens, props)]
    placeholder: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled))]
    #[element(theme_lens, props(default))]
    disabled: bool,
    #[rich(read(copy, rename = is_focused))]
    #[element(theme_lens)]
    focused: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    #[element(theme_lens)]
    mouse_over: bool,
}

pub enum Msg {
    SetTheme(Theme),
    SetText(Cow<'static, str>),
    TrySetText(Option<Cow<'static, str>>),
    SetMaxLength(att::MaxLength),
    TrySetMaxLength(Option<att::MaxLength>),
    SetPlaceholder(Cow<'static, str>),
    TrySetPlaceholder(Option<Cow<'static, str>>),
    SetDisabled(bool),
    Disable,
    Enable,
    SetFocus(bool),
    SetMouseOver(bool),
    UpdateTextFromView,
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for Entry<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let local_events = Events::default().and_input(|conf| {
            conf.focus(|_| Msg::SetFocus(true))
                .blur(|_| Msg::SetFocus(false))
                .mouse_enter(|_| Msg::SetMouseOver(true))
                .mouse_leave(|_| Msg::SetMouseOver(false))
                .input(|_| Msg::UpdateTextFromView)
        });

        Self {
            el_ref: ElRef::default(),
            msg_mapper: props.msg_mapper,
            local_events,
            events: props.events,
            styler: props.styler,
            theme: props.theme,
            text: props.text,
            max_length: props.max_length,
            placeholder: props.placeholder,
            disabled: props.disabled,
            focused: false,
            mouse_over: false,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetText(val) => self.text = Some(val),
            Msg::TrySetText(val) => self.text = val,
            Msg::SetMaxLength(val) => self.max_length = Some(val),
            Msg::TrySetMaxLength(val) => self.max_length = val,
            Msg::SetPlaceholder(val) => self.placeholder = Some(val),
            Msg::TrySetPlaceholder(val) => self.placeholder = val,
            Msg::SetDisabled(val) => self.disabled = val,
            Msg::Disable => self.disabled = true,
            Msg::Enable => self.disabled = false,
            Msg::SetFocus(val) => self.focused = val,
            Msg::SetMouseOver(val) => self.mouse_over = val,
            Msg::UpdateTextFromView => {
                if let Some(input) = self.el_ref.get() {
                    self.text = Some(input.value().into());
                }
            }
        }
    }
}

impl<PMsg: 'static> View for Entry<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.entry()(&self.theme_lens())),
        )
    }
}

impl<PMsg: 'static> StyledView for Entry<PMsg> {
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let input = html::input()
            .set(&self.local_events.input)
            .set(&style.input)
            .and_attributes(|conf| {
                conf.set_class("input")
                    .set_disabled(self.disabled)
                    .try_set_value(self.text.clone())
                    .try_set_max_length(self.max_length)
                    .try_set_placeholder(self.placeholder.clone())
            })
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.input);

        html::div()
            .set(&style.container)
            .set(&self.local_events.container)
            .map_msg_with(&self.msg_mapper)
            .add(&self.events.container)
            .add(input)
    }
}

impl<PMsg: 'static> Props<PMsg> {
    pub fn init<GMsg: 'static>(self, orders: &mut impl Orders<PMsg, GMsg>) -> Entry<PMsg> {
        Entry::init(self, orders)
    }
}

impl<PMsg: 'static> Entry<PMsg> {
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn try_set_styler<GMsg: 'static>(
        &mut self,
        val: Option<impl Into<Styler<PMsg>>>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler<GMsg: 'static>(
        &mut self,
        val: impl Into<Styler<PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.try_set_styler(Some(val), orders)
    }
}

pub type Styler<PMsg> = theme::Styler<Entry<PMsg>, Style>;
pub type ThemeStyler<'a> = theme::Styler<EntryLens<'a>, Style>;
