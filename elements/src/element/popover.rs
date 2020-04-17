use crate::prelude::*;
use derive_rich::Rich;
use savory_core::prelude::*;
use savory_html::prelude::*;

// TODO: add placement property
#[derive(Clone, Rich, Element)]
#[element(style(panel, popover), events(panel, popover))]
pub struct Popover<PMsg, C, T> {
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    #[element(props(default))]
    events: Events<PMsg>,
    #[rich(read)]
    #[element(props)]
    styler: Option<Styler<PMsg, C, T>>,
    #[rich(read)]
    #[element(theme_lens, props(default))]
    theme: Theme,

    #[rich(read)]
    #[element(props(required))]
    child: C,
    #[rich(read)]
    #[element(props(required))]
    target: T,
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens, props(default = "false"))]
    toggled: bool,
    #[rich(read(copy))]
    #[element(theme_lens, props(default = "0"))]
    offset: i8,
}

pub enum Msg {
    SetTheme(Theme),
    SetToggled(bool),
    Toggle,
    Open,
    Close,
    SetOffset(i8),
}

impl<PMsg, GMsg, C, T> Element<PMsg, GMsg> for Popover<PMsg, C, T>
where
    PMsg: 'static,
    GMsg: 'static,
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    type Message = Msg;
    type Props = Props<PMsg, C, T>;

    fn init(props: Self::Props, orders: &mut impl Orders<PMsg, GMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        Self {
            msg_mapper: props.msg_mapper,
            events: props.events,
            styler: props.styler,
            theme: props.theme,
            child: props.child,
            target: props.target,
            toggled: props.toggled,
            offset: props.offset,
        }
    }

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::SetTheme(val) => self.theme = val,
            Msg::SetToggled(val) => self.toggled = val,
            Msg::Toggle => self.toggled = !self.toggled,
            Msg::Open => self.toggled = true,
            Msg::Close => self.toggled = false,
            Msg::SetOffset(val) => self.offset = val,
        }
    }
}

impl<PMsg, C, T> View for Popover<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        self.styled_view(
            self.styler
                .as_ref()
                .map(|styler| styler(self))
                .unwrap_or_else(|| self.theme.popover()(&self.theme_lens())),
        )
    }
}

impl<PMsg, C, T> StyledView for Popover<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    type Style = Style;

    fn styled_view(&self, style: Style) -> Self::Output {
        let panel = html::div()
            .class("panel")
            .set(&style.panel)
            .set(&self.events.panel)
            .add(self.child.view());

        html::div()
            .class("popover")
            .set(&style.popover)
            .set(&self.events.popover)
            .add(self.target.view())
            .add(panel)
    }
}

impl<PMsg: 'static, C, T> Props<PMsg, C, T>
where
    C: View<Output = Node<PMsg>>,
    T: View<Output = Node<PMsg>>,
{
    pub fn init<GMsg: 'static>(self, orders: &mut impl Orders<PMsg, GMsg>) -> Popover<PMsg, C, T> {
        Popover::init(self, orders)
    }
}

impl<PMsg: 'static, C, T> Popover<PMsg, C, T> {
    pub fn and_events<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Events<PMsg>) -> Events<PMsg>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.events = get_val(self.events.clone());
    }

    pub fn set_child<GMsg>(&mut self, child: C, _: &mut impl Orders<PMsg, GMsg>) -> C
    where
        GMsg: 'static,
        PMsg: 'static,
        C: View<Output = Node<PMsg>>,
    {
        std::mem::replace(&mut self.child, child)
    }

    pub fn set_target<GMsg>(&mut self, target: T, _: &mut impl Orders<PMsg, GMsg>) -> T
    where
        GMsg: 'static,
        PMsg: 'static,
        T: View<Output = Node<PMsg>>,
    {
        std::mem::replace(&mut self.target, target)
    }

    pub fn try_set_styler<GMsg: 'static>(
        &mut self,
        val: Option<impl Into<Styler<PMsg, C, T>>>,
        _: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.styler = val.map(|s| s.into());
    }

    pub fn set_styler<GMsg: 'static>(
        &mut self,
        val: impl Into<Styler<PMsg, C, T>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) {
        self.try_set_styler(Some(val), orders);
    }

    pub fn update_child<GMsg: 'static>(
        &mut self,
        child_msg: C::Message,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) where
        C: Element<PMsg, GMsg>,
    {
        self.child.update(child_msg, orders)
    }

    pub fn update_target<GMsg: 'static>(
        &mut self,
        child_msg: T::Message,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) where
        T: Element<PMsg, GMsg>,
    {
        self.target.update(child_msg, orders)
    }
}

pub type Styler<PMsg, C, T> = theme::Styler<Popover<PMsg, C, T>, Style>;
pub type ThemeStyler<'a> = theme::Styler<PopoverLens<'a>, Style>;
