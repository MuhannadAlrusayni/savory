use crate::{
    css::{self, unit::px, St, Style},
    el::{
        button::{self, Button},
        icon::{SvgIcon, HtmlIcon, UrlIcon},
        layout::flexbox::{self, Flexbox},
    },
    theme::Theme,
};
// use seed::prelude::*;
use palette::{Hsl, Hsla, Saturate, Shade};

pub struct Simple {
    dark_mode: bool,
}

impl Default for Simple {
    fn default() -> Self {
        Self {
            dark_mode: false,
        }
    }
}

impl Simple {
    pub fn primary(&self) -> Hsla {
        Hsla::new(208.8, 1.0, 0.53, 1.)
    }

    pub fn suggestion(&self) -> Hsla {
        self.primary()
    }

    pub fn destructive(&self) -> Hsla {
        Hsla::new(0., 0.88, 0.541, 1.0)
    }

    pub fn controls(&self) -> Hsla {
        if self.dark_mode {
            Hsla::new(0., 0., 0.098, 1.)
        } else {
            Hsla::new(0., 0., 1., 1.)
        }
    }

    pub fn title(&self) -> Hsla {
        if self.dark_mode {
            Hsla::new(0., 0., 1., 0.98)
        } else {
            Hsla::new(0., 0., 0., 0.98)
        }
    }

    pub fn primary_text(&self) -> Hsla {
        if self.dark_mode {
            Hsla::new(0., 0., 1., 0.9)
        } else {
            Hsla::new(0., 0., 0., 0.9)
        }
    }

    pub fn secondary_text(&self) -> Hsla {
        if self.dark_mode {
            Hsla::new(0., 0., 1., 0.82)
        } else {
            Hsla::new(0., 0., 0., 0.82)
        }
    }

    pub fn disable(&self, value: impl Into<Hsla>) -> Hsla {
        let mut value = value.into().desaturate(1.);
        if self.dark_mode {
            value.alpha -= 0.2;
        } else {
            value.alpha -= 0.1;
        }
        value
    }

    pub fn focus(&self, value: impl Into<Hsla>) -> Hsla {
        let value = value.into();
        if self.dark_mode {
            value.lighten(0.1)
        } else {
            value.lighten(0.15)
        }
    }
}

impl Theme for Simple {
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
        let (bg_color, text_color) = match btn.kind {
            button::Kind::Normal => (self.controls(), self.title()),
            button::Kind::Suggestion => (self.suggestion(), self.title()),
            button::Kind::Destructive => (self.destructive(), self.title()),
        };

        let is_disabled = btn.is_disabled();
        let is_focused = btn.is_focused();
        let is_mouse_over = btn.is_mouse_over();
        let (bg_color, text_color) = match (is_focused, is_disabled, is_mouse_over) {
            (_, true, _) => (self.disable(bg_color), self.disable(text_color)),
            (true, false, _) | (_, false, true) => (self.focus(bg_color), self.focus(text_color)),
            (false, false, false) => (bg_color, text_color),
        };

        let background = css::Background::default()
            .color(bg_color);

        let border = css::Border::default()
            .color(bg_color)
            .width(px(1.))
            .solid()
            .radius(px(4.));

        let padding = css::Padding::default()
            .x(px(12.))
            .y(px(4.));

        let size = css::Size::default()
            .height(px(34.));

        let style = &btn.style;
        Style::default()
            .merge(&background)
            .merge(&border)
            .merge(&padding)
            .merge(&size)
            .color(text_color)
            .merge(&style.size)
            .merge(&style.border)
            .merge(&style.background)
            .merge(&style.margin)
            .merge(&style.padding)
    }
}
