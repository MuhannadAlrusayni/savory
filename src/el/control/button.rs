use crate::{css, prelude::*};
use derive_rich::Rich;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}

#[derive(Clone)]
pub enum Inner<PMsg: 'static> {
    Children(Vec<Node<PMsg>>),
    Common(Option<Cow<'static, str>>, Option<Icon<PMsg>>),
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Route,
}

pub type LocalEvents = Events<Msg>;
pub type ParentEvents<PMsg> = Events<PMsg>;

#[derive(Clone, Rich)]
pub struct Button<PMsg: 'static> {
    // general element properties
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(read, write(style = compose))]
    local_events: LocalEvents,
    #[rich(read, write(style = compose))]
    events: ParentEvents<PMsg>,
    #[rich(read, write(style = compose))]
    user_style: UserStyle,

    // button element properties
    inner: Inner<PMsg>,
    #[rich(read(copy), value_fns = {
        /// Change button kind to normal
        normal = Kind::Normal,
        /// Change button kind to suggestion
        suggestion = Kind::Suggestion,
        /// Change button kind to destructive
        destructive = Kind::Destructive,
        /// Change button kind to link
        link = Kind::Link,
        /// Change button kind to dashed
        dashed = Kind::Dashed,
    })]
    kind: Option<Kind>,
    #[rich(read(copy), write)]
    block: bool,
    #[rich(read(copy), write)]
    ghost: bool,
    #[rich(read, write)]
    route: Option<Cow<'static, str>>,
    #[rich(read(copy, rename = is_disabled), write, value_fns = { disable = true, enable = false })]
    disabled: bool,
    #[rich(read(copy, rename = is_focused), write)]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over), write)]
    mouse_over: bool,
}

impl<PMsg> Button<PMsg> {
    pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
        let local_events = Events::default()
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Route);

        Button {
            el_ref: ElRef::default(),
            msg_mapper: msg_mapper.into(),
            local_events,
            events: Events::default(),
            inner: Inner::Common(None, None),
            kind: None,
            block: false,
            ghost: false,
            user_style: UserStyle::default(),
            route: None,
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    pub fn with_label(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        label: impl Into<Cow<'static, str>>,
    ) -> Self {
        Button::new(msg_mapper).set_label(label)
    }

    pub fn with_children(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        children: Vec<Node<PMsg>>,
    ) -> Self {
        Button::new(msg_mapper).set_children(children)
    }

    pub fn label(&self) -> Option<&str> {
        match self.inner {
            Inner::Common(Some(ref lbl), _) => Some(lbl),
            _ => None,
        }
    }

    pub fn set_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        match self.inner {
            Inner::Common(Some(ref mut lbl), _) => *lbl = label.into(),
            Inner::Common(ref mut lbl, _) => *lbl = Some(label.into()),
            _ => self.inner = Inner::Common(Some(label.into()), None),
        };
        self
    }

    pub fn set_children(mut self, children: Vec<Node<PMsg>>) -> Self {
        self.inner = Inner::Children(children);
        self
    }

    pub fn set_icon(mut self, new_icon: impl Into<Icon<PMsg>>) -> Self {
        match self.inner {
            Inner::Common(_, ref mut icon) => *icon = Some(new_icon.into()),
            _ => self.inner = Inner::Common(None, Some(new_icon.into())),
        };
        self
    }

    fn handle_route_msg(&mut self) {
        if let Some(ref route) = self.route {
            seed::browser::service::routing::push_route(
                route
                    .split('/')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }
}

impl<GMsg, PMsg: 'static> Model<PMsg, GMsg> for Button<PMsg> {
    type Message = Msg;

    fn update(&mut self, msg: Msg, _: &mut impl Orders<PMsg, GMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Route => self.handle_route_msg(),
        }
    }
}

/// This style used by users when they want to change styles of SpinEntry
#[derive(Clone, Default, Rich)]
pub struct UserStyle {
    #[rich(write(style = compose))]
    pub button: css::Style,
    #[rich(write(style = compose))]
    pub common_container: flexbox::Style,
}

/// This style returned by the Theme and consumed by render function, thus the
/// icons must be returned by the theme
#[derive(Clone, Debug, Default, Rich)]
pub struct Style {
    #[rich(write(style = compose))]
    pub button: css::Style,
    #[rich(write(style = compose))]
    pub common_container: flexbox::Style,
}

impl<PMsg: 'static> Render<PMsg> for Button<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.button(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        let inner = match self.inner {
            Inner::Children(ref children) => children.clone(),
            Inner::Common(ref label, ref icon) => {
                let icon = icon.as_ref().map(|icon| icon.render(theme));
                let label = label.as_ref().map(|lbl| plain!(lbl.clone()));
                nodes![Flexbox::new()
                    .try_add(icon)
                    .try_add(label)
                    .render_with_style(theme, style.common_container)]
            }
        };

        button!()
            .set_events(&self.local_events)
            .set_style(style.button)
            .and_attributes(|conf| conf.set_class("button").set_disabled(self.disabled))
            .map_msg_with(&self.msg_mapper)
            .add_events(&self.events)
            .add_children(inner)
    }
}
