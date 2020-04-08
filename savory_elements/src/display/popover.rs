use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

// TODO: add placement property
#[derive(Clone, Rich, Element)]
#[element(style("hm", "ddd"))]
pub struct Popover<PMsg, C, T> {
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    // #[rich(read)]
    // local_events: Events<Msg>,
    #[rich(read)]
    #[element(props(default = "Events::default()"))]
    events: Events<PMsg>,
    #[rich(read)]
    #[element(props)]
    style: Option<Style>,
    #[rich(read)]
    #[element(props)]
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

crate::style_type! {
    panel,
    popover,
}

crate::events_type! {
    panel,
    popover,
}

pub enum Msg {
    SetTheme(Theme),
    SetStyleFn(Box<dyn FnOnce(Style) -> Style>),
    SetStyle(Style),
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
{
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Props<PMsg>, orders: &mut impl Orders<PMsg, GMsg>) -> Self {
        let mut orders = orders.proxy_with(&props.msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        Self {
            msg_mapper: props.msg_mapper,
            events: props.events,
            style: props.style,
            theme: props.theme.unwrap_or_else(|| Theme::default()),
            child: props.child,
            target: props.target,
            toggled: props.toggled,
            offset: props.offset,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetTheme(val) => self.set_theme(val, &mut orders),
            Msg::SetStyleFn(val) => self.set_style_fn(val, &mut orders),
            Msg::SetStyle(val) => self.set_style(val, &mut orders),
            Msg::SetToggled(val) => self.set_toggled(!self.toggled, &mut orders),
            Msg::Toggle => self.set_toggled(!self.toggled, &mut orders),
            Msg::Open => self.set_toggled(true, &mut orders),
            Msg::Close => self.set_toggled(false, &mut orders),
            Msg::SetOffset(val) => self.set_offset(val, &mut orders),
        }
    }
}

impl<PMsg, C, T> View for Popover<PMsg, C, T> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        todo!()
        // let panel = div!()
        //     .set(att::class("panel"))
        //     .set(style["panel"])
        //     .try_set(self.events.get("panel"))
        //     .add(self.child.view(theme));

        // div!()
        //     .set(att::class("popover"))
        //     .set(style["popover"])
        //     .try_set(self.events.get("popover"))
        //     .add(vec![self.target.view(theme), panel])
    }
}

impl<PMsg, C, T> Popover<PMsg, C, T> {
    fn set_theme<GMsg: 'static>(&mut self, val: Theme, orders: &mut impl Orders<Msg, GMsg>) {
        self.theme = val;
    }

    fn set_style_fn<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Style) -> Style,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        // FIXME: finder better way, that doesn't need to clone the style
        self.style = match self.style {
            Some(ref style) => Some(get_val(style.clone())),
            None => Some(get_val(self.theme.popover(self.theme_lens()))),
        };
    }

    fn set_style<GMsg: 'static>(&mut self, val: Style, _orders: &mut impl Orders<Msg, GMsg>) {
        self.style = Some(val);
    }

    fn set_toggled<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.toggled != val {
            self.toggled = val;
        } else {
            orders.skip();
        }
    }

    fn set_offset<GMsg: 'static>(&mut self, val: i8, orders: &mut impl Orders<Msg, GMsg>) {
        if self.offset != val {
            self.offset = val;
        } else {
            orders.skip();
        }
    }
}
