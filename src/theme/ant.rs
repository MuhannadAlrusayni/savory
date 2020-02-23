use crate::{
    css::{
        self,
        unit::{em, ms, px, sec},
        values as val, St, Style,
    },
    el::prelude::*,
    theme::Theme,
};

use palette::{Hsl, Hsla};
// use seed::prelude::*;

pub fn contrast_ratio(background_light: f32, foreground_light: f32) -> f32 {
    if background_light > foreground_light {
        (background_light + 0.05) / (foreground_light + 0.05)
    } else {
        (foreground_light + 0.05) / (background_light + 0.05)
    }
}

pub struct BaseColor {
    dust_red: [Hsl; 10],
    volcano: [Hsl; 10],
    sunset_orange: [Hsl; 10],
    calendula_gold: [Hsl; 10],
    sunrise_yellow: [Hsl; 10],
    lime: [Hsl; 10],
    polar_green: [Hsl; 10],
    cyan: [Hsl; 10],
    daybreak_blue: [Hsl; 10],
    geek_blue: [Hsl; 10],
    golden_purple: [Hsl; 10],
    magenta: [Hsl; 10],
}

impl Default for BaseColor {
    fn default() -> Self {
        Self {
            dust_red: variant(Hsl::new(356.87, 0.9134, 0.5471)),
            volcano: variant(Hsl::new(15.14, 0.9569, 0.5451)),
            sunset_orange: variant(Hsl::new(31.05, 0.958, 0.5333)),
            calendula_gold: variant(Hsl::new(39.91, 0.9583, 0.5294)),
            sunrise_yellow: variant(Hsl::new(51.91, 0.9583, 0.5294)),
            lime: variant(Hsl::new(77.1, 0.8547, 0.4588)),
            polar_green: variant(Hsl::new(100.24, 0.7658, 0.4353)),
            cyan: variant(Hsl::new(180.0, 0.9021, 0.7608)),
            daybreak_blue: variant(Hsl::new(208.8, 1.0, 0.547)),
            geek_blue: variant(Hsl::new(228.2, 0.825, 0.553)),
            golden_purple: variant(Hsl::new(265.0, 0.639, 0.5)),
            magenta: variant(Hsl::new(327.1, 0.825, 0.553)),
        }
    }
}

pub struct SystemLevel {
    base_colors: BaseColor,
    neutral_color: [Hsl; 10],
}

impl Default for SystemLevel {
    fn default() -> Self {
        let gray = |lvl| Hsl::new(0., 0., lvl);
        Self {
            base_colors: BaseColor::default(),
            neutral_color: [
                gray(1.),
                gray(0.98),
                gray(0.961),
                gray(0.91),
                gray(0.851),
                gray(0.749),
                gray(0.549),
                gray(0.349),
                gray(0.149),
                gray(0.0),
            ],
        }
    }
}

pub struct FunctionalColor {
    blue: Hsl,
    green: Hsl,
    gold: Hsl,
    red: Hsl,
}

impl Default for FunctionalColor {
    fn default() -> Self {
        Self {
            blue: Hsl::new(208.8, 1.0, 0.547),
            green: Hsl::new(100.2, 0.766, 0.435),
            gold: Hsl::new(39.9, 0.0958, 0.529),
            red: Hsl::new(356.9, 0.913, 0.547),
        }
    }
}

pub struct ProdectNeutralColor {
    title: (Hsla, Hsla),
    primary_text: (Hsla, Hsla),
    secondary_text: (Hsla, Hsla),
    disable: (Hsla, Hsla),
    border: (Hsla, Hsla),
    divider: (Hsla, Hsla),
    background: (Hsla, Hsla),
    table_header: (Hsla, Hsla),
}

impl Default for ProdectNeutralColor {
    fn default() -> Self {
        let white = |alpha| Hsla::new(0.0, 0.0, 1.0, alpha);
        let black = |alpha| Hsla::new(0.0, 0.0, 0.0, alpha);
        Self {
            title: (black(0.85), white(1.0)),
            primary_text: (black(0.65), white(0.85)),
            secondary_text: (black(0.45), white(0.65)),
            disable: (black(0.25), white(0.45)),
            border: (black(0.15), white(0.25)),
            divider: (black(0.9), white(0.15)),
            background: (black(0.4), white(0.9)),
            table_header: (black(0.2), white(0.4)),
        }
    }
}

pub struct ProdectLevel {
    brand_color: [Hsl; 10],
    functional_color: FunctionalColor,
    neutral_color: ProdectNeutralColor,
}

impl Default for ProdectLevel {
    fn default() -> Self {
        Self {
            brand_color: variant(Hsl::new(208.8, 1.0, 0.547)),
            functional_color: FunctionalColor::default(),
            neutral_color: ProdectNeutralColor::default(),
        }
    }
}

pub fn variant(base_color: impl Into<Hsl>) -> [Hsl; 10] {
    use palette::Hsv;

    let hue_step = 2.;
    let saturation_step = 16.;
    let saturation_step2 = 5.;
    let brightness_step1 = 5.;
    let brightness_step2 = 15.;
    let light_color_count = 5.;
    let dark_color_count = 4.;

    let base_color = base_color.into();

    let get_hue = |hsv: &Hsv, i, light| {
        let hue = hsv.hue.to_positive_degrees();
        let hue = if hue > 60. && hue <= 240. {
            if light {
                hue - hue_step * i
            } else {
                hue + hue_step * i
            }
        } else {
            if light {
                hue + hue_step * i
            } else {
                hue - hue_step * i
            }
        };
        hue % 360.
    };

    let get_saturation = |hsv: &Hsv, i, light| {
        // grey color don't change saturation
        if hsv.hue == 0. && hsv.saturation == 0. {
            return hsv.saturation;
        }
        let mut saturation = if light {
            (hsv.saturation * 100.) - saturation_step * i
        } else if i == dark_color_count {
            (hsv.saturation * 100.) + saturation_step
        } else {
            (hsv.saturation * 100.) + saturation_step2 * i
        };

        if saturation > 100. {
            saturation = 100.;
        }

        if light && i == light_color_count && saturation > 10. {
            saturation = 10.;
        }

        if saturation < 6. {
            saturation = 6.;
        }

        saturation * 0.01
    };

    let get_value = |hsv: &Hsv, i, light| {
        let value = if light {
            (hsv.value * 100.) + brightness_step1 * i
        } else {
            (hsv.value * 100.) - brightness_step2 * i
        };

        value * 0.01
    };

    let mut variants: [Hsl; 10] = [base_color; 10];
    let hsv: Hsv = base_color.into();
    // light variants
    for i in (0..light_color_count as usize).rev() {
        let hsl: Hsl = Hsv::new(
            get_hue(&hsv, i as f32, true),
            get_saturation(&hsv, i as f32, true),
            get_value(&hsv, i as f32, true),
        )
        .into();
        variants[(i as f32 - light_color_count).abs() as usize] = hsl;
    }
    // dark variants
    variants[5] = base_color;
    for i in 1..=dark_color_count as usize {
        let hsl: Hsl = Hsv::new(
            get_hue(&hsv, i as f32, false),
            get_saturation(&hsv, i as f32, false),
            get_value(&hsv, i as f32, false),
        )
        .into();
        variants[i + 5] = hsl;
    }

    variants.into()
}

pub enum Variant {
    L50 = 0,
    L100 = 1,
    L200 = 2,
    L300 = 3,
    L400 = 4,
    L500 = 5,
    D600 = 6,
    D700 = 7,
    D800 = 8,
    D900 = 9,
}

#[derive(Default)]
pub struct Ant {
    prodect: ProdectLevel,
    system: SystemLevel,
}

impl Ant {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn brand(&self, variant: Variant) -> Hsla {
        self.prodect.brand_color[variant as usize].into()
    }

    pub fn blue(&self) -> Hsla {
        self.prodect.functional_color.blue.into()
    }

    pub fn green(&self) -> Hsla {
        self.prodect.functional_color.green.into()
    }

    pub fn gold(&self) -> Hsla {
        self.prodect.functional_color.gold.into()
    }

    pub fn red(&self) -> Hsla {
        self.prodect.functional_color.red.into()
    }

    pub fn primary(&self) -> Hsla {
        self.brand(Variant::D600)
    }

    pub fn suggestion(&self) -> Hsla {
        self.brand(Variant::D600)
    }

    pub fn destructive(&self) -> Hsla {
        self.red()
    }

    pub fn dust_red(&self, variant: Variant) -> Hsla {
        self.system.base_colors.dust_red[variant as usize].into()
    }

    pub fn volcano(&self, variant: Variant) -> Hsla {
        self.system.base_colors.volcano[variant as usize].into()
    }

    pub fn sunset_orange(&self, variant: Variant) -> Hsla {
        self.system.base_colors.sunset_orange[variant as usize].into()
    }

    pub fn calendula_gold(&self, variant: Variant) -> Hsla {
        self.system.base_colors.calendula_gold[variant as usize].into()
    }

    pub fn sunrise_yellow(&self, variant: Variant) -> Hsla {
        self.system.base_colors.sunrise_yellow[variant as usize].into()
    }

    pub fn lime(&self, variant: Variant) -> Hsla {
        self.system.base_colors.lime[variant as usize].into()
    }

    pub fn polar_green(&self, variant: Variant) -> Hsla {
        self.system.base_colors.polar_green[variant as usize].into()
    }

    pub fn cyan(&self, variant: Variant) -> Hsla {
        self.system.base_colors.cyan[variant as usize].into()
    }

    pub fn daybreak_blue(&self, variant: Variant) -> Hsla {
        self.system.base_colors.daybreak_blue[variant as usize].into()
    }

    pub fn geek_blue(&self, variant: Variant) -> Hsla {
        self.system.base_colors.geek_blue[variant as usize].into()
    }

    pub fn golden_purple(&self, variant: Variant) -> Hsla {
        self.system.base_colors.golden_purple[variant as usize].into()
    }

    pub fn magenta(&self, variant: Variant) -> Hsla {
        self.system.base_colors.magenta[variant as usize].into()
    }

    pub fn gray(&self, variant: Variant) -> Hsla {
        self.system.neutral_color[variant as usize].into()
    }

    pub fn white(&self) -> Hsla {
        Hsla::new(0., 0., 1., 1.)
    }

    pub fn black(&self) -> Hsla {
        Hsla::new(0., 0., 0., 1.)
    }

    pub fn title(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.title.1
        } else {
            self.prodect.neutral_color.title.0
        }
    }

    pub fn primary_text(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.primary_text.1
        } else {
            self.prodect.neutral_color.primary_text.0
        }
    }

    pub fn secondary_text(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.secondary_text.1
        } else {
            self.prodect.neutral_color.secondary_text.0
        }
    }

    pub fn disable(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.disable.1
        } else {
            self.prodect.neutral_color.disable.0
        }
    }

    pub fn border(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.border.1
        } else {
            self.prodect.neutral_color.border.0
        }
    }

    pub fn divider(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.divider.1
        } else {
            self.prodect.neutral_color.divider.0
        }
    }

    pub fn background(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.background.1
        } else {
            self.prodect.neutral_color.background.0
        }
    }

    pub fn table_header(&self, light: bool) -> Hsla {
        if light {
            self.prodect.neutral_color.table_header.1
        } else {
            self.prodect.neutral_color.table_header.0
        }
    }

    // fn color(_base_color: impl Into<Hsla>, _variant: Variant) -> Hsla {
    //     unimplemented!()
    // }

    // fn on_color(_base_color: impl Into<Hsla>, _variant: Variant) -> Hsla {
    //     unimplemented!()
    // }

    fn button_normal<PMsg>(&self, btn: &Button<PMsg>) -> Style {
        // colors
        let (bg, fg, border_color) =
            match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
                // btn is disabled
                (true, _, _) => (
                    self.gray(Variant::L200),
                    self.disable(false),
                    self.gray(Variant::L400),
                ),
                // btn is not focused or hovered
                (false, false, false) => {
                    (self.white(), self.primary_text(false), self.border(false))
                }
                // btn is hovered or focused
                _ => (
                    self.white(),
                    self.brand(Variant::L500),
                    self.brand(Variant::L500),
                ),
            };

        let mut style = Style::default();
        style
            .and_border(|conf| {
                conf.width(px(1.))
                    .solid()
                    .radius(px(4.))
                    .color(border_color)
            })
            .and_background(|conf| conf.color(bg))
            .and_text(|conf| conf.color(fg))
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)");
        style
    }

    fn button_suggestion<PMsg>(&self, btn: &Button<PMsg>) -> Style {
        // colors
        let (bg, fg, border_color) =
            match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
                // btn is disabled
                (true, _, _) => (
                    self.gray(Variant::L200),
                    self.disable(false),
                    self.gray(Variant::L400),
                ),
                // btn is not focused or hovered
                (false, false, false) => (
                    self.brand(Variant::L500),
                    self.white(),
                    self.brand(Variant::L500),
                ),
                // btn is hovered or focused
                _ => (
                    self.brand(Variant::L400),
                    self.white(),
                    self.brand(Variant::L400),
                ),
            };

        let mut style = Style::default();
        style
            .and_border(|conf| {
                conf.width(px(1.))
                    .solid()
                    .radius(px(4.))
                    .color(border_color)
            })
            .and_background(|conf| conf.color(bg))
            .and_text(|conf| conf.color(fg))
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)");
        style
    }

    fn button_destructive<PMsg>(&self, btn: &Button<PMsg>) -> Style {
        // colors
        let (bg, fg, border_color) =
            match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
                // btn is disabled
                (true, _, _) => (
                    self.gray(Variant::L200),
                    self.disable(false),
                    self.gray(Variant::L400),
                ),
                // btn is not focused or hovered
                (false, false, false) => (
                    self.dust_red(Variant::L500),
                    self.white(),
                    self.dust_red(Variant::L500),
                ),
                // btn is hovered or focused
                _ => (
                    self.dust_red(Variant::L400),
                    self.white(),
                    self.dust_red(Variant::L400),
                ),
            };

        let mut style = Style::default();
        style
            .and_border(|conf| {
                conf.width(px(1.))
                    .solid()
                    .radius(px(4.))
                    .color(border_color)
            })
            .and_background(|conf| conf.color(bg))
            .and_text(|conf| conf.color(fg))
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)");
        style
    }

    fn button_link<PMsg>(&self, btn: &Button<PMsg>) -> Style {
        // colors
        let (bg, fg) = match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
            (true, _, _) => (self.white(), self.disable(false)),
            // btn is not focused or hovered
            (false, false, false) => (self.white(), self.brand(Variant::L500)),
            // btn is hovered or focused
            _ => (self.white(), self.brand(Variant::L400)),
        };

        let mut style = Style::default();
        style
            .and_text(|conf| conf.color(fg))
            .and_border(|conf| conf.width(px(0.)).solid().radius(px(4.)).color(bg))
            .and_background(|conf| conf.color(bg));
        style
    }

    fn button_dashed<PMsg>(&self, btn: &Button<PMsg>) -> Style {
        // colors
        let (bg, fg, border_color) =
            match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
                (true, _, _) => (
                    self.gray(Variant::L200),
                    self.disable(false),
                    self.gray(Variant::L400),
                ),
                // btn is not focused or hovered
                (false, false, false) => (
                    self.white(),
                    self.primary_text(false),
                    self.gray(Variant::D600),
                ),
                // btn is hovered or focused
                _ => (
                    self.white(),
                    self.brand(Variant::L500),
                    self.brand(Variant::L500),
                ),
            };

        let mut style = Style::default();
        style
            .and_border(|conf| {
                conf.width(px(1.))
                    .dashed()
                    .radius(px(4.))
                    .color(border_color)
            })
            .and_background(|conf| conf.color(bg))
            .and_text(|conf| conf.color(fg))
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)");
        style
    }
}

impl Theme for Ant {
    fn flexbox<PMsg: 'static>(&self, flex: &Flexbox<PMsg>) -> flexbox::Style {
        let mut style = Style::default();
        style
            .display(val::Flex)
            .try_flex_direction(flex.direction)
            .try_flex_wrap(flex.wrap)
            .try_justify_content(flex.justify_content)
            .try_align_items(flex.align_items)
            .try_align_content(flex.align_content)
            .try_gap(flex.gap)
            .size(flex.size)
            .border(flex.border)
            .background(flex.background.clone())
            .margin(flex.margin)
            .padding(flex.padding)
            .merge(&flex.style);
        style
    }

    fn flexbox_item<PMsg: 'static>(&self, item: &flexbox::Item<PMsg>) -> flexbox::ItemStyle {
        let mut style = Style::default();
        style
            .try_add(St::Order, item.order)
            .try_add(St::FlexGrow, item.grow)
            .try_add(St::FlexShrink, item.shrink)
            .try_merge(item.basis.as_ref())
            .try_merge(item.align_self.as_ref())
            .and_size(|conf| {
                *conf = item.size.clone();
                conf
            })
            .merge(&item.size)
            .merge(&item.border)
            .merge(&item.background)
            .merge(&item.margin)
            .merge(&item.padding);
        style
    }

    // fn grid(&self) -> Style;

    fn popover<'a, PMsg, C, T>(&self, popover: &Popover<'a, PMsg, C, T>) -> popover::Style {
        let mut container = Style::default();
        container.and_position(|conf| conf.relative());

        let mut panel = Style::default();
        panel
            .and_transition(|conf| {
                conf.add("opacity", |conf| conf.duration(ms(150.)).ease())
                    .add("transform", |conf| conf.duration(ms(150.)).ease())
                    .add("visibility", |conf| conf.duration(ms(150.)).ease())
            })
            .and_position(|conf| conf.absolute())
            .and_background(|conf| conf.color(self.white()))
            .and_border(|conf| {
                conf.color(self.border(false))
                    .none()
                    .width(px(0.))
                    .radius(px(4.))
            })
            .add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)")
            .and_padding(|conf| conf.x(px(4.)).y(px(2)))
            .and_margin(|conf| conf.top(px(popover.offset)))
            .config_block(|style| {
                if popover.is_visible() {
                    style.opacity(1.).visibility(val::Visible)
                } else {
                    style.visibility(val::Hidden).opacity(0.)
                }
            });

        popover::Style { container, panel }
    }

    fn svg_icon<PMsg: 'static>(&self, icon: &SvgIcon<PMsg>) -> icon::SvgStyle {
        let mut style = Style::default();
        style.try_merge(icon.color.as_ref()).merge(&icon.size);
        style
    }

    fn html_icon<PMsg>(&self, icon: &HtmlIcon<PMsg>) -> icon::HtmlStyle {
        let mut style = Style::default();
        style.try_merge(icon.color.as_ref()).merge(&icon.size);
        style
    }

    fn url_icon<PMsg>(&self, icon: &UrlIcon<PMsg>) -> icon::UrlStyle {
        let mut style = Style::default();
        style.merge(&icon.size);
        style
    }

    // TODO: handle is_loading()
    // TODO: handle btn.is_block()
    // TODO: handle btn.is_ghost()
    fn button<PMsg>(&self, btn: &Button<PMsg>) -> button::Style {
        let cursor = btn
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Pointer.into());

        let mut style = match btn.kind {
            Some(button::Kind::Normal) | None => self.button_normal(btn),
            Some(button::Kind::Suggestion) => self.button_suggestion(btn),
            Some(button::Kind::Destructive) => self.button_destructive(btn),
            Some(button::Kind::Link) => self.button_link(btn),
            Some(button::Kind::Dashed) => self.button_dashed(btn),
        };
        style
            .and_padding(|conf| conf.x(px(15.)).y(px(0.)))
            .and_size(|conf| conf.height(px(32.)))
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .and_text(|conf| {
                conf.decoration(|d| d.line(val::None))
                    .line_height(1.499)
                    .white_space(val::Nowrap)
            })
            .add(St::Outline, val::None)
            .cursor(cursor)
            .add(St::UserSelect, val::None)
            .add(St::BoxSizing, val::BorderBox)
            .add(St::FontSize, px(14.))
            .add(St::FontWeight, "400")
            .merge(&btn.style);
        style
    }

    fn switch<PMsg>(&self, switch: &Switch<PMsg>) -> switch::Style {
        let width = 44.;
        let height = 22.;
        let btn_size = height - 3.;
        let top = 3. / 2.;
        let left = 3. / 2.;

        let cursor = switch
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Pointer.into());

        let bg_color = if switch.is_toggled() {
            self.brand(Variant::L500)
        } else {
            self.gray(Variant::L500)
        };

        let mut background = Style::default();
        background
            .config_block(|conf| {
                if switch.is_disabled() {
                    conf.opacity(0.4);
                }
                conf
            })
            .cursor(cursor)
            .and_position(|conf| conf.relative())
            .and_background(|conf| conf.color(bg_color))
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .and_border(|conf| {
                conf.transparent()
                    .width(px(0.))
                    .radius(px(height / 2.))
                    .none()
            })
            .display(val::InlineBlock)
            .and_text(|conf| conf.decoration(|d| d.line(val::None)))
            // .add(St::Outline, val::None)
            .add(St::UserSelect, val::None)
            .add(St::BoxSizing, val::BorderBox)
            .and_size(|conf| conf.height(px(height)).min_width(px(width)));

        let mut button = Style::default();
        button
            .config_block(|conf| {
                if switch.is_toggled() {
                    conf.add(St::Transform, format!("translateX({})", px(width / 2.)));
                }
                conf
            })
            .and_position(|conf| conf.absolute().top(px(top)).left(px(left)))
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .and_background(|conf| conf.color(self.white()))
            .and_border(|conf| conf.width(px(0.)).transparent().none().radius(0.5))
            .add(St::BoxShadow, "0 2px 4px 0 rgba(0, 35, 11, 0.2)")
            .and_size(|conf| conf.resize(px(btn_size), px(btn_size)));

        switch::Style { background, button }
    }

    fn checkbox<PMsg>(&self, checkbox: &Checkbox<PMsg>) -> checkbox::Style {
        let (bg, fg, border) = match (
            checkbox.is_disabled(),
            checkbox.is_focused(),
            checkbox.is_mouse_over(),
        ) {
            (true, _, _) => (
                self.gray(Variant::L200),
                self.disable(false),
                self.gray(Variant::L400),
            ),
            (false, false, false) => {
                if checkbox.is_toggled() {
                    (
                        self.brand(Variant::L500),
                        self.white(),
                        self.brand(Variant::L500),
                    )
                } else {
                    (self.white(), self.white(), self.border(false))
                }
            }
            _ => {
                if checkbox.is_toggled() {
                    (
                        self.brand(Variant::L500),
                        self.white(),
                        self.brand(Variant::L500),
                    )
                } else {
                    (self.white(), self.white(), self.brand(Variant::L500))
                }
            }
        };

        let cursor = checkbox
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Pointer.into());

        let mut input = Style::default();
        input
            .and_transition(|conf| {
                conf.all(|conf| {
                    conf.duration(sec(0.3))
                        .cubic_bezier(0.645, 0.045, 0.355, 1.)
                })
            })
            .cursor(cursor)
            .display(val::Flex)
            .justify_content(val::Center)
            .align_items(val::Center)
            .add(St::WebkitAppearance, val::None)
            .add(St::Appearance, val::None)
            .and_size(|conf| conf.resize(px(16.), px(16.)))
            .and_border(|conf| conf.solid().width(px(1.)).color(border).radius(px(2.)))
            .and_background(|conf| conf.color(bg))
            .and_text(|conf| conf.color(fg));

        let mut button = Style::default();
        button.config_block(|conf| {
            if checkbox.is_toggled() {
                conf.cursor(cursor)
                    .and_transition(|conf| {
                        conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
                    })
                    .and_size(|conf| conf.resize(0.2, 0.55))
                    .and_border(|conf| {
                        conf.bottom(|conf| conf.solid().width(px(2.)).color(fg))
                            .right(|conf| conf.solid().width(px(2.)).color(fg))
                    })
                    .and_margin(|conf| conf.bottom(px(2.)))
                    .add(St::Transform, "rotate(45deg)");
            }
            conf
        });

        let mut label = Style::default();
        label
            .config_block(|conf| {
                if checkbox.is_disabled() {
                    conf.and_text(|conf| conf.color(self.disable(false)))
                } else {
                    conf
                }
            })
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .cursor(cursor)
            .display(val::Flex)
            .gap(px(4.));

        checkbox::Style {
            input,
            button,
            label,
        }
    }

    fn radio<PMsg>(&self, radio: &Radio<PMsg>) -> radio::Style {
        let (bg, fg, border) = match (
            radio.is_disabled(),
            radio.is_focused(),
            radio.is_mouse_over(),
        ) {
            (true, _, _) => (
                self.gray(Variant::L200),
                self.disable(false),
                self.gray(Variant::L400),
            ),
            (false, false, false) => {
                if radio.is_toggled() {
                    (
                        self.white(),
                        self.brand(Variant::L500),
                        self.brand(Variant::L500),
                    )
                } else {
                    (self.white(), self.white(), self.border(false))
                }
            }
            _ => {
                if radio.is_toggled() {
                    (
                        self.white(),
                        self.brand(Variant::L500),
                        self.brand(Variant::L500),
                    )
                } else {
                    (self.white(), self.white(), self.brand(Variant::L500))
                }
            }
        };

        let cursor = radio
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Pointer.into());

        let mut input = Style::default();
        input
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .cursor(cursor)
            .display(val::Flex)
            .justify_content(val::Center)
            .align_items(val::Center)
            .add(St::WebkitAppearance, val::None)
            .and_text(|conf| conf.color(fg))
            .and_size(|conf| conf.resize(px(16.), px(16.)))
            .and_border(|conf| conf.solid().width(px(1.)).color(border).radius(0.5))
            .and_background(|conf| conf.color(bg));

        let mut button = Style::default();
        button
            .config_block(|conf| {
                if radio.is_toggled() {
                    conf.cursor(cursor)
                        .and_size(|conf| conf.resize(0.6, 0.6))
                        .and_border(|conf| conf.none().radius(0.5))
                        .and_background(|conf| conf.color(fg));
                }
                conf
            })
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            });

        let mut label = Style::default();
        label
            .config_block(|conf| {
                if radio.is_disabled() {
                    conf.and_text(|conf| conf.color(self.disable(false)));
                }
                conf
            })
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .cursor(cursor)
            .display(val::Flex)
            .gap(px(4.));

        radio::Style {
            input,
            button,
            label,
        }
    }

    fn entry<PMsg>(&self, entry: &Entry<PMsg>) -> entry::Style {
        let (bg, fg, border) = match (
            entry.is_disabled(),
            entry.is_focused(),
            entry.is_mouse_over(),
        ) {
            (true, _, _) => (
                self.gray(Variant::L200),
                self.disable(false),
                self.gray(Variant::L400),
            ),
            (false, false, false) => (self.white(), self.primary_text(false), self.border(false)),
            _ => (
                self.white(),
                self.primary_text(false),
                self.brand(Variant::L500),
            ),
        };

        let cursor = entry
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Initial.into());

        let mut container = Style::default();
        container
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .display(val::Flex)
            .align_items(val::Center)
            .justify_content(val::Center)
            .and_padding(|conf| conf.y(px(4.)).x(px(11.)))
            .gap(px(4.))
            .and_background(|conf| conf.color(bg))
            .and_border(|conf| conf.solid().width(px(1.)).color(border).radius(px(4.)))
            .and_size(|conf| conf.width(1.))
            .cursor(cursor);

        let mut input = Style::default();
        input
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .and_size(|conf| conf.width(1.).height(px(32.)))
            .and_text(|conf| conf.color(fg))
            .and_border(|conf| conf.none())
            .and_background(|conf| conf.transparent())
            .add(St::WebkitAppearance, val::None)
            .cursor(cursor);

        entry::Style { container, input }
    }

    fn spin_entry<PMsg>(&self, spin_entry: &SpinEntry<PMsg>) -> spin_entry::Style {
        let (bg, fg, border) = match (
            spin_entry.is_disabled(),
            spin_entry.is_focused(),
            spin_entry.is_mouse_over(),
        ) {
            (true, _, _) => (
                self.gray(Variant::L200),
                self.disable(false),
                self.gray(Variant::L400),
            ),
            (false, false, false) => (self.white(), self.primary_text(false), self.border(false)),
            _ => (
                self.white(),
                self.primary_text(false),
                self.brand(Variant::L500),
            ),
        };

        let cursor = spin_entry
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Initial.into());

        // em unit used here
        let width = 4.;
        let height = 1.5;
        let btns_container_width = 1.;
        let btns_container_height = height;
        let input_width = width - 0.2;
        let input_height = 1.;
        // px used here
        let radius = 4.;
        let border_width = 1.;
        // percent units used here
        let btn_height = 0.5;
        let btn_width = 1.;
        let btn_mouse_over_height = btn_height + 0.10;
        let btn_mouse_over_height_2 = btn_height - 0.10;

        let mut container = Style::default();
        container
            .and_position(|conf| conf.relative())
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .display(val::Flex)
            .align_items(val::Center)
            .justify_content(val::Center)
            .and_background(|conf| conf.color(bg))
            .and_border(|conf| {
                conf.solid()
                    .width(px(border_width))
                    .color(border)
                    .radius(px(radius))
            })
            .and_size(|conf| conf.width(em(width)).height(em(height)))
            .cursor(cursor);

        let mut input = Style::default();
        input
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .and_size(|conf| conf.width(em(input_width)).height(em(input_height)))
            .and_text(|conf| conf.color(fg))
            .and_border(|conf| conf.none())
            .and_background(|conf| conf.transparent())
            .add(St::WebkitAppearance, val::None)
            .cursor(cursor);

        let mut buttons_container = Style::default();
        buttons_container
            .and_position(|conf| conf.absolute().right(px(0.)))
            .and_transition(|conf| {
                conf.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .display(val::Flex)
            .flex_direction(val::Column)
            .and_size(|conf| {
                conf.width(em(btns_container_width))
                    .height(em(btns_container_height))
            })
            .and_border(|conf| {
                conf.left(|conf| {
                    conf.solid()
                        .width(px(border_width))
                        .color(self.gray(Variant::L300))
                })
            })
            .config_block(
                |conf| match (spin_entry.is_mouse_over(), spin_entry.is_disabled()) {
                    (true, false) => conf.opacity(1.).visibility(val::Visible),
                    _ => conf.opacity(0.).visibility(val::Hidden),
                },
            );

        let increment_item = Style::default();
        let decrement_item = Style::default();

        let (inc_btn_height, dec_btn_height) = match (
            spin_entry.increment_button.is_mouse_over(),
            spin_entry.decrement_button.is_mouse_over(),
        ) {
            (true, _) => (btn_mouse_over_height, btn_mouse_over_height_2),
            (_, true) => (btn_mouse_over_height_2, btn_mouse_over_height),
            (false, false) => (btn_height, btn_height),
        };

        let mut increment_button = Style::default();
        increment_button
            .and_background(|conf| conf.color(self.white()))
            .and_border(|conf| conf.none().top_right(px(radius)))
            .and_size(|conf| conf.width(btn_width).height(inc_btn_height));
        let mut decrement_button = Style::default();
        decrement_button
            .and_background(|conf| conf.color(self.white()))
            .and_border(|conf| {
                conf.none().bottom_right(px(radius)).top(|conf| {
                    conf.solid()
                        .width(px(border_width))
                        .color(self.gray(Variant::L300))
                })
            })
            .and_size(|conf| conf.width(btn_width).height(dec_btn_height));

        let increment_icon: Icon<spin_entry::Msg> = Icon::html(
            r#"""<path d="M890.5 755.3L537.9 269.2c-12.8-17.6-39-17.6-51.7 0L133.5 755.3A8 8 0 0 0 140 768h75c5.1 0 9.9-2.5 12.9-6.6L512 369.8l284.1 391.6c3 4.1 7.8 6.6 12.9 6.6h75c6.5 0 10.3-7.4 6.5-12.7z"></path>"""#,
        ).into();
        let decrement_icon: Icon<spin_entry::Msg> = Icon::html(
            r#"""<path d="M884 256h-75c-5.1 0-9.9 2.5-12.9 6.6L512 654.2 227.9 262.6c-3-4.1-7.8-6.6-12.9-6.6h-75c-6.5 0-10.3 7.4-6.5 12.7l352.6 486.1c12.8 17.6 39 17.6 51.7 0l352.6-486.1c3.9-5.3.1-12.7-6.4-12.7z"></path>"""#,
        ).into();

        spin_entry::Style {
            container,
            input,
            buttons_container,
            increment_item,
            decrement_item,
            increment_button,
            decrement_button,
            increment_icon,
            decrement_icon,
        }
    }
}
