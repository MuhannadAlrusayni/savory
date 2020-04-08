use crate::{button::ButtonLens, prelude::*};
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Element)]
pub struct MenuButton<PMsg, C> {
    // general element properties
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read)]
    style: Option<Style>,
    #[rich(read)]
    theme: Theme,

    // menu button element properties
    #[rich(read)]
    #[element(theme_lens(nested))]
    button: Button<Msg>,
    #[rich(read)]
    child: Option<C>,
    #[rich(read(copy, rename = is_toggled))]
    #[element(theme_lens)]
    toggled: bool,
}

crate::style_type! {
    panel,
    popover,

    {
        popover() -> popover::Style {
            panel: panel,
            popover: popover,
        }
    }
}

pub enum Msg {
    SetTheme(Theme),
    SetStyleFn(Box<dyn FnOnce(Style) -> Style>),
    SetStyle(Style),
    Button(button::Msg),
    TogglePopover,
    Open,
    Close,
}

impl<PMsg, GMsg, C> Element<PMsg, GMsg> for MenuButton<PMsg, C>
where
    PMsg: 'static,
    GMsg: 'static,
    C: View<Output = Node<PMsg>>,
{
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        let msg_mapper = msg_mapper.into();
        let mut orders = orders.proxy_with(&msg_mapper);
        orders.subscribe(|theme: ThemeChanged| Msg::SetTheme(theme.0));

        let mut button = Button::init(Msg::Button, &mut orders);
        button.and_events(
            |events| events.and_button(|conf| conf.click(|_| Msg::TogglePopover)),
            &mut orders,
        );

        Self {
            theme: Theme::default(),
            style: None,
            msg_mapper: msg_mapper,
            button,
            child: None,
            toggled: false,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        let mut orders = orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::SetStyleFn(val) => self.set_style_fn(val, &mut orders),
            Msg::SetStyle(val) => self.set_style(val, &mut orders),
            Msg::SetTheme(val) => self.set_theme(val, &mut orders),
            Msg::Button(msg) => self.button.update(msg, &mut orders),
            Msg::TogglePopover => self.set_toggled(!self.toggled, &mut orders),
            Msg::Open => self.set_toggled(true, &mut orders),
            Msg::Close => self.set_toggled(false, &mut orders),
        }
    }
}

impl<PMsg, C> View for MenuButton<PMsg, C>
where
    C: View<Output = Node<PMsg>>,
{
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        let view = |style: &Style| {
            // if let Some(child)
            // let popover = Popover::new(&self.button, &self.child);
            // .set_visible(self.popup)
            // .set_offset(4)
            // .view(theme)
            todo!()
        };

        match self.style {
            Some(ref style) => view(&style),
            None => view(&self.theme.menu_button(self.theme_lens())),
        }
    }
}

impl<PMsg: 'static, C> MenuButton<PMsg, C> {
    pub fn set_child<GMsg>(&mut self, child: C, _orders: &mut impl Orders<PMsg, GMsg>)
    where
        GMsg: 'static,
        C: View<Output = Node<PMsg>>,
    {
        self.child = Some(child);
    }

    pub fn update_child<GMsg>(
        &mut self,
        child_msg: C::Message,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) where
        GMsg: 'static,
        C: Element<PMsg, GMsg>,
    {
        if let Some(ref mut child) = self.child {
            child.update(child_msg, orders);
        }
    }
}

impl<PMsg, C> MenuButton<PMsg, C> {
    fn set_style_fn<GMsg: 'static>(
        &mut self,
        get_val: impl FnOnce(Style) -> Style,
        _orders: &mut impl Orders<Msg, GMsg>,
    ) {
        // FIXME: finder better way, that doesn't need to clone the style
        self.style = match self.style {
            Some(ref style) => Some(get_val(style.clone())),
            None => Some(get_val(self.theme.menu_button(self.theme_lens()))),
        };
    }

    fn set_style<GMsg: 'static>(&mut self, val: Style, _orders: &mut impl Orders<Msg, GMsg>) {
        self.style = Some(val);
    }

    fn set_theme<GMsg: 'static>(&mut self, val: Theme, orders: &mut impl Orders<Msg, GMsg>) {
        self.theme = val;
    }

    fn set_toggled<GMsg: 'static>(&mut self, val: bool, orders: &mut impl Orders<Msg, GMsg>) {
        if self.toggled != val {
            self.toggled = val;
        } else {
            orders.skip();
        }
    }
}
