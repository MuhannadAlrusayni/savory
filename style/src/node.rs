use crate::Style;
use savory::{
    prelude::{AndEl, El, Node},
    seed::prelude::UpdateEl,
};

pub trait StyleApi {
    fn style(self, style: Style) -> Self;
    fn try_style(self, style: Option<Style>) -> Self;
    fn replace_style(self, style: Style) -> Self;
    fn try_replace_style(self, style: Option<Style>) -> Self;
    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self;
}

impl<Msg> StyleApi for El<Msg> {
    fn style(mut self, style: Style) -> Self {
        style.update_el(&mut self);
        self
    }

    fn try_style(self, style: Option<Style>) -> Self {
        match style {
            Some(style) => self.style(style),
            None => self,
        }
    }

    fn replace_style(mut self, style: Style) -> Self {
        self.style = style.into();
        self
    }

    fn try_replace_style(self, style: Option<Style>) -> Self {
        match style {
            Some(style) => self.replace_style(style),
            None => self,
        }
    }

    fn and_style(mut self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.style = conf(self.style.into()).into();
        self
    }
}

impl<Msg> StyleApi for Node<Msg> {
    fn style(self, style: Style) -> Self {
        self.and_el(|el| el.style(style))
    }

    fn try_style(self, style: Option<Style>) -> Self {
        self.and_el(|el| el.try_style(style))
    }

    fn replace_style(self, style: Style) -> Self {
        self.and_el(|el| el.replace_style(style))
    }

    fn try_replace_style(self, style: Option<Style>) -> Self {
        self.and_el(|el| el.try_replace_style(style))
    }

    fn and_style(self, conf: impl FnOnce(Style) -> Style) -> Self {
        self.and_el(|el| el.and_style(conf))
    }
}
