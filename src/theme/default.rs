use crate::{
    css::{self, Style},
    el::{
        button::{self, Button},
        icon::{SvgIcon, HtmlIcon, UrlIcon},
        layout::flexbox::{self, Flexbox},
    },
    theme::Theme,
};
use seed::prelude::*;
use palette::{Hsl, Hsla};

pub struct Default;

impl Default {
    pub fn normal(&self) -> Hsla {
        Hsla::new(0., 0., 0., 1.0)
    }

    pub fn suggestion(&self) -> Hsla {
        Hsla::new(0., 0., 0., 1.0)
    }

    pub fn destructive(&self) -> Hsla {
        Hsla::new(0., 0., 0., 1.0)
    }
}

impl Theme for Default {
    fn flexbox<PMsg: 'static>(&self, flex: &Flexbox<PMsg>) -> Style {
        // flex container style
        Style::default()
            .add(St::Display, css::Flex)
            .try_merge(flex.direction.as_ref())
            .try_merge(flex.wrap.as_ref())
            .try_merge(flex.justify_content.as_ref())
            .try_merge(flex.align_items.as_ref())
            .try_merge(flex.align_content.as_ref())
            .try_merge(flex.gap.as_ref())
            .merge(&flex.size)
            .merge(&flex.border)
            .merge(&flex.background)
            .merge(&flex.border)
            .merge(&flex.margin)
            .merge(&flex.padding)
    }

    fn flexbox_item<PMsg: 'static>(&self, item: &flexbox::Item<PMsg>) -> Style {
        Style::default()
            .try_add(St::Order, item.order)
            .try_add(St::FlexGrow, item.grow)
            .try_add(St::FlexShrink, item.shrink)
            .try_merge(item.basis.as_ref())
            .try_merge(item.align_self.as_ref())
            .merge(&item.size)
            .merge(&item.border)
            .merge(&item.background)
            .merge(&item.margin)
            .merge(&item.padding)
    }

    // fn grid(&self) -> Style;

    fn svg_icon<Msg: 'static>(&self, icon: &SvgIcon<Msg>) -> Style {
        Style::default()
            .try_merge(icon.color.as_ref())
            .merge(&icon.size)
    }

    fn html_icon(&self, icon: &HtmlIcon) -> Style {
        Style::default()
            .try_merge(icon.color.as_ref())
            .merge(&icon.size)
    }

    fn url_icon(&self, icon: &UrlIcon) -> Style {
        Style::default().merge(&icon.size)
    }

    fn button(&self, btn: &Button) -> Style {
        let main_color = match btn.kind {
            button::Kind::Normal => self.normal(),
            button::Kind::Suggestion => self.suggestion(),
            button::Kind::Destructive => self.destructive(),
        };

        let main_color = match (btn.is_focused(), btn.is_disabled(), btn.is_mouse_over()) {
            (_, true, _) => {},
            (true, false, false) => {},
            (true, false, true) => {},
            (false, false, false) => {},
            (false, false, true) => {},
        };


        let style = &btn.style;
        Style::default()
            .merge(&style.size)
            .merge(&style.border)
            .merge(&style.background)
            .merge(&style.margin)
            .merge(&style.padding)
    }
}
