use crate::{
    css::{self, unit::px, values as val, Style},
    el::{Flexbox, Icon},
    model::Model,
    propertie::{Shape, Size},
    render::Render,
    theme::{Theme, Themeable},
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
    Link,
    Dashed,
}

#[derive(Debug, Clone)]
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

#[derive(Clone, Debug, Rich)]
pub struct Button {
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

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    pub fn new() -> Self {
        Button {
            inner: Inner::Common(None, None),
            size: None,
            kind: None,
            shape: None,
            block: false,
            loading: false,
            ghost: false,
            style: Style::default(),
            route: None,
            // events: Self::create_events(),
            disabled: false,
            focus: false,
            mouse_over: false,
        }
    }

    pub fn with_label(label: impl Into<String>) -> Self {
        Button::default().label(label)
    }

    pub fn with_children(children: Vec<Node<Msg>>) -> Self {
        Button::default().children(children)
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

impl<GMsg: 'static> Model<Msg, GMsg> for Button {
    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg, GMsg>) {
        match msg {
            Msg::MouseEnter => self.mouse_over = true,
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => self.focus = true,
            Msg::Blur => self.focus = false,
            Msg::Route => self.handle_route_msg(),
        }
    }
}

impl Render<Msg> for Button {
    type View = Node<Msg>;

    fn render(&self, theme: &impl Theme) -> Self::View {
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

        button![
            attrs![
                At::Disabled => self.disabled.as_at_value()
            ],
            simple_ev(Ev::Focus, Msg::Focus),
            simple_ev(Ev::Blur, Msg::Blur),
            simple_ev(Ev::MouseEnter, Msg::MouseEnter),
            simple_ev(Ev::MouseLeave, Msg::MouseLeave),
            simple_ev(Ev::Click, Msg::Route),
            theme.button(self),
            inner,
        ]
    }
}

impl Themeable for Button {
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
