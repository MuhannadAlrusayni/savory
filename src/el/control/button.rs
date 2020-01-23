use crate::{
    css::{self, unit::px, values as val, Style},
    el::{Flexbox, Icon},
    events::Events,
    model::Model,
    propertie::{Shape, Size},
    render::Render,
    theme::{Theme, Themeable},
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

#[derive(Clone, Rich)]
pub struct Button<PMsg> {
    msg_mapper: Rc<dyn Fn(Msg) -> PMsg>,
    internal_events: Events<Msg>,
    #[rich(write(take, style = compose))]
    events: Events<PMsg>,
    // children
    pub inner: Inner,
    // properties
    #[rich(value_fns(take) = {
        small = Size::Small,
        medium = Size::Medium,
        large = Size::Large,
    })]
    pub size: Option<Size>,
    #[rich(value_fns(take) = {
        normal = Kind::Normal,
        suggestion = Kind::Suggestion,
        destructive = Kind::Destructive,
        link = Kind::Link,
        dashed = Kind::Dashed,
    })]
    pub kind: Option<Kind>,
    #[rich(value_fns(take) = {
        circle = Shape::Circle,
        round = Shape::Round,
        rectangle = Shape::Rectangle
    })]
    pub shape: Option<Shape>,
    #[rich(value_fns(take) = { block = true, inline = false })]
    pub block: bool,
    #[rich(
        read(copy, rename = is_loading),
        value_fns(take) = { loading = true, loading_off = false }
    )]
    pub loading: bool,
    #[rich(value_fns(take) = { ghost = true, ghost_off = false })]
    pub ghost: bool,
    #[rich(write(take, style = compose))]
    pub style: Style,

    // #[rich(write(take, style = compose))]
    // events: Events<Msg>,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns(take) = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(write(take))]
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
        Button {
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            internal_events: Events::default()
                .focus(|_| Msg::Focus)
                .blur(|_| Msg::Blur)
                .mouse_enter(|_| Msg::MouseEnter)
                .mouse_leave(|_| Msg::MouseLeave)
                .click(|_| Msg::Route),
            events: Events::default(),
            inner: Inner::Common(None, None),
            size: None,
            kind: None,
            shape: None,
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
        Button::new(msg_mapper).label(label)
    }

    pub fn with_children(
        msg_mapper: impl FnOnce(Msg) -> PMsg + Clone + 'static,
        children: Vec<Node<Msg>>,
    ) -> Self {
        Button::new(msg_mapper).children(children)
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        match self.inner {
            Inner::Common(Some(ref mut lbl), _) => *lbl = label.into(),
            Inner::Common(ref mut lbl, _) => *lbl = Some(label.into()),
            _ => self.inner = Inner::Common(Some(label.into()), None),
        };
        self
    }

    pub fn children(mut self, children: Vec<Node<Msg>>) -> Self {
        self.inner = Inner::Child(children);
        self
    }

    pub fn icon(mut self, new_icon: impl Into<Icon<Msg>>) -> Self {
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

impl<GMsg, PMsg: 'static> Model<Msg, PMsg, GMsg> for Button<PMsg> {
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

impl<PMsg: 'static> Render<PMsg> for Button<PMsg> {
    type View = Node<PMsg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
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
                vec![Flexbox::new()
                    .center()
                    .full_size()
                    .gap(px(4.))
                    .add(|item| item.content(vec![icon]))
                    .add(|item| item.content(vec![lbl]).wrapped())
                    .render(theme)]
            }
        };

        let mut btn = button![
            self.internal_events.events.clone(),
            attrs![
                At::Disabled => self.disabled.as_at_value()
            ],
            theme.button(self),
            inner,
        ]
        .map_msg(move |msg| (Rc::clone(&msg_mapper))(msg));
        for event in self.events.events.clone().into_iter() {
            btn.add_listener(event);
        }
        btn
    }
}

impl<PMsg> Themeable for Button<PMsg> {
    type StyleMap = Style;
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
