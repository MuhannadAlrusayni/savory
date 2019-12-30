use crate::{
    css,
    el::{
        icon::Icon,
        propertie::{Size, Shape},
    },
    macros::*,
    model::Model,
    theme::Theme,
    view::View,
};
use derive_rich::Rich;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Rich, Debug, Clone, Default)]
pub struct Style {
    #[rich(write(take, style = compose))]
    pub size: css::Size,
    #[rich(write(take, style = compose))]
    pub border: css::Border,
    #[rich(write(take, style = compose))]
    pub background: css::Background,
    #[rich(write(take, style = compose))]
    pub margin: css::Margin,
    #[rich(write(take, style = compose))]
    pub padding: css::Padding,
}

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Normal,
    Suggestion,
    Destructive,
}

#[derive(Debug)]
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
}

#[derive(Debug, Rich)]
pub struct Button {
    // children
    pub inner: Inner,
    // properties
    #[rich(write(take, style = compose))]
    pub style: Style,
    // #[rich(write(take, style = compose))]
    // events: Events<Msg>,
    #[rich(value_fns(take) = {
        small = Size::Small,
        medium = Size::Medium,
        large = Size::Large,
    })]
    size: Size,
    #[rich(value_fns(take) = {
        normal = Kind::Normal,
        suggestion = Kind::Suggestion,
        destructive = Kind::Destructive,
    })]
    pub kind: Kind,
    #[rich(value_fns(take) = {
        circle = Shape::Circle,
        round = Shape::Round,
        rectangle = Shape::Rectangle
    })]
    pub shape: Shape,
    #[rich(
        read(copy, rename = is_disabled),
        value_fns(take) = { disable = true, enable = false }
    )]
    pub disabled: bool,
    #[rich(value_fns(take) = { block = true, inline = false })]
    pub block: bool,
    #[rich(
        read(copy, rename = is_loading),
        value_fns(take) = { loading = true, loading_off = false }
    )]
    pub loading: bool,
    #[rich(
        read(copy, rename = is_ghost),
        value_fns(take) = { ghost = true, ghost_off = false }
    )]
    pub ghost: bool,
    #[rich(write(take))]
    pub href: Option<Cow<'static, str>>,

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
            style: Style::default(),
            disabled: false,
            block: false,
            shape: Shape::Round,
            size: Size::Medium,
            kind: Kind::Normal,
            loading: false,
            ghost: false,
            href: None,
            // events: Self::create_events(),
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
}

impl<GMsg: 'static> Model<Msg, GMsg> for Button {
    fn update(&mut self, msg: Msg, _: &mut impl Orders<Msg, GMsg>) {
        match msg {
            Msg::MouseEnter => {
                self.mouse_over = true;
                log!("btn mouse enter");
            }
            Msg::MouseLeave => self.mouse_over = false,
            Msg::Focus => {
                self.focus = true;
                log!("btn focused");
            }
            Msg::Blur => self.focus = false,
        }
    }
}

impl View<Msg> for Button {
    fn view(&self, theme: &impl Theme) -> Node<Msg> {
        let inner: Vec<Node<Msg>> = match self.inner {
            Inner::Child(ref children) => children.clone(),
            Inner::Common(ref lbl, ref icon) => {
                let icon = icon
                    .as_ref()
                    .map(|icon| icon.view(theme))
                    .unwrap_or(empty![]);
                let lbl = lbl
                    .as_ref()
                    .map(|lbl| plain![lbl.clone()])
                    .unwrap_or(empty![]);
                vec![icon, lbl]
            }
        };
        button![
            simple_ev(Ev::Focus, Msg::Focus),
            simple_ev(Ev::Blur, Msg::Blur),
            simple_ev(Ev::MouseEnter, Msg::MouseEnter),
            simple_ev(Ev::MouseLeave, Msg::MouseLeave),
            theme.button(self),
            inner,
        ]
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
