use crate::{
    css::{self, unit::px},
    el::{Flexbox, Icon},
    events::Events,
    model::Model,
    render::Render,
    theme::Theme,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::{borrow::Cow, rc::Rc};

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}

#[derive(Clone)]
pub enum Inner {
    Child(Vec<Node<Msg>>),
    Common(Option<String>, Option<Icon<Msg>>),
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
pub struct Button<PMsg> {
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    #[rich(write(style = compose))]
    local_events: LocalEvents,
    #[rich(write(style = compose))]
    events: ParentEvents<PMsg>,
    // children
    pub inner: Inner,
    // properties
    #[rich(value_fns = {
        /// Change button kind to normal
        normal = Kind::Normal,
        suggestion = Kind::Suggestion,
        destructive = Kind::Destructive,
        link = Kind::Link,
        dashed = Kind::Dashed,
    })]
    pub kind: Option<Kind>,
    #[rich(value_fns = { block = true, inline = false })]
    pub block: bool,
    #[rich(
        read(copy, rename = is_loading),
        value_fns = { loading = true, loading_off = false }
    )]
    pub loading: bool,
    #[rich(value_fns = { ghost = true, ghost_off = false })]
    pub ghost: bool,
    #[rich(write(style = compose))]
    pub style: Style,

    // #[rich(write(style = compose))]
    // events: Events<Msg>,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(write)]
    pub route: Option<Cow<'static, str>>,

    // read only properties, these shouldn't be editable from out side of this
    // module, this may changed later.
    #[rich(read(copy, rename = is_focused))]
    focus: bool,
    #[rich(read(copy, rename = is_mouse_over))]
    mouse_over: bool,
    // active: bool,
}

impl<PMsg> Button<PMsg> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static) -> Self {
        let mut local_events = Events::default();
        local_events
            .focus(|_| Msg::Focus)
            .blur(|_| Msg::Blur)
            .mouse_enter(|_| Msg::MouseEnter)
            .mouse_leave(|_| Msg::MouseLeave)
            .click(|_| Msg::Route);

        Button {
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            local_events,
            events: Events::default(),
            inner: Inner::Common(None, None),
            kind: None,
            block: false,
            loading: false,
            ghost: false,
            style: Style::default(),
            route: None,
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    pub fn with_label(
        msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static,
        label: impl Into<String>,
    ) -> Self {
        let mut btn = Button::new(msg_mapper);
        btn.label(label);
        btn
    }

    pub fn with_children(
        msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static,
        children: Vec<Node<Msg>>,
    ) -> Self {
        let mut btn = Button::new(msg_mapper);
        btn.children(children);
        btn
    }

    pub fn label(&mut self, label: impl Into<String>) -> &mut Self {
        match self.inner {
            Inner::Common(Some(ref mut lbl), _) => *lbl = label.into(),
            Inner::Common(ref mut lbl, _) => *lbl = Some(label.into()),
            _ => self.inner = Inner::Common(Some(label.into()), None),
        };
        self
    }

    pub fn children(&mut self, children: Vec<Node<Msg>>) -> &mut Self {
        self.inner = Inner::Child(children);
        self
    }

    pub fn icon(&mut self, new_icon: impl Into<Icon<Msg>>) -> &mut Self {
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

pub type Style = css::Style;

impl<PMsg: 'static> Render<PMsg> for Button<PMsg> {
    type View = Node<PMsg>;
    type Style = Style;

    fn style(&self, theme: &impl Theme) -> Self::Style {
        theme.button(self)
    }

    fn render_with_style(&self, theme: &impl Theme, style: Self::Style) -> Self::View {
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());

        let inner: Vec<Node<Msg>> = match self.inner {
            Inner::Child(ref children) => children.clone(),
            Inner::Common(ref lbl, ref icon) => {
                let icon = icon
                    .as_ref()
                    .map(|icon| icon.render(theme))
                    .unwrap_or(empty![]);
                let lbl = lbl
                    .as_ref()
                    .map(|lbl| plain![lbl.clone()])
                    .unwrap_or(empty![]);
                let mut lbl = Flexbox::item_with(nodes![lbl]);
                lbl.wrapped();
                // TODO: use el::flexbox::Style insted of hard coding the style
                vec![Flexbox::new()
                    .center()
                    .full_size()
                    .gap(px(4.))
                    .add(nodes![icon])
                    .add(lbl)
                    .render(theme)]
            }
        };

        let mut btn = button![
            self.local_events.events.clone(),
            attrs![
                At::Disabled => self.disabled.as_at_value()
            ],
            style,
            inner,
        ]
        .map_msg(move |msg| (Rc::clone(&msg_mapper))(msg));
        for event in self.events.events.clone().into_iter() {
            btn.add_listener(event);
        }
        btn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    enum ChildMsg {
        First,
    }

    #[test]
    fn test_name() {}
}
