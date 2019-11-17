use crate::{
    el::icon::Icon,
    macros::*,
    model::Model,
    properties::{background::Background, border::Border, margin::Margin, padding::Padding},
    properties::{color::Color, size::Size},
    theme::Theme,
    view::View,
};
use seed::{dom_types::Style, prelude::*};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub enum Msg<CMsg, IMsg>
where
    CMsg: 'static + Clone,
    IMsg: 'static + Clone,
{
    Child(CMsg),
    Icon(IMsg),
    MouseEnter,
    MouseLeave,
    Foucs,
    Blur,
}

#[derive(Clone, Debug)]
enum Inner<CMsg, IMsg>
where
    CMsg: 'static + Clone,
    IMsg: 'static + Clone,
{
    Child(Node<CMsg>),
    Common(Option<String>, Option<Icon<IMsg>>),
}

#[derive(Clone, Debug)]
pub struct Button<CMsg, IMsg>
where
    CMsg: 'static + Clone,
    IMsg: 'static + Clone,
{
    // child
    inner: Inner<CMsg, IMsg>,
    // properties
    disabled: bool,
    link: Option<Cow<'static, str>>,
    size: Size,
    border: Border,
    background: Background,
    margin: Margin,
    padding: Padding,
}

impl<CMsg, IMsg> Button<CMsg, IMsg>
where
    CMsg: 'static + Clone,
    IMsg: 'static + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: Inner::Common(None, None),
            disabled: false,
            link: None,
            size: Size::default(),
            border: Border::default(),
            background: Background::default(),
            margin: Margin::default(),
            padding: Padding::default(),
        }
    }

    value_functions! {
        disabled {
            disable() => true,
            enable() => false,
        }
    }

    builder_functions! {
        link(Cow<'static, str>),
    }

    composition_functions! {
        size: Size,
        border: Border,
        background: Background,
        margin: Margin,
        padding: Padding,
    }
}

impl<GMsg, CMsg, IMsg> Model<Msg<CMsg, IMsg>, GMsg> for Button<CMsg, IMsg>
where
    CMsg: 'static + Clone,
    IMsg: 'static + Clone,
    GMsg: 'static,
{
    fn update(&mut self, _: Msg<CMsg, IMsg>, _: &mut impl Orders<Msg<CMsg, IMsg>, GMsg>) {
        unimplemented!()
    }
}

impl<CMsg, IMsg> View<Msg<CMsg, IMsg>> for Button<CMsg, IMsg>
where
    CMsg: 'static + Clone,
    IMsg: 'static + Clone,
{
    fn view(&self, theme: &impl Theme) -> Node<Msg<CMsg, IMsg>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let btn = Button::<(), ()>::new();
    }
}
