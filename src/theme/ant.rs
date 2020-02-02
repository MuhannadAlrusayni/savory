use crate::{
    css::{
        self,
        color::Color,
        unit::{ms, px, rem, sec},
        values as val, St, Style,
    },
    el::prelude::*,
    propertie::*,
    theme::{Theme, Themeable},
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
        let mut hue = if hue > 60. && hue <= 240. {
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

    fn color(base_color: impl Into<Hsla>, variant: Variant) -> Hsla {
        unimplemented!()
    }

    fn on_color(base_color: impl Into<Hsla>, variant: Variant) -> Hsla {
        unimplemented!()
    }

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
        Style::default()
            .border(|b| b.width(px(1.)).solid().radius(px(4.)).color(border_color))
            .background(|b| b.color(bg))
            .color(fg)
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
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
        Style::default()
            .border(|b| b.width(px(1.)).solid().radius(px(4.)).color(border_color))
            .background(|b| b.color(bg))
            .color(fg)
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
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
        Style::default()
            .border(|b| b.width(px(1.)).solid().radius(px(4.)).color(border_color))
            .background(|b| b.color(bg))
            .color(fg)
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
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
        Style::default()
            .color(fg)
            .border(|b| b.width(px(0.)).solid().radius(px(4.)).color(bg))
            .background(|b| b.color(bg))
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

        Style::default()
            .border(|b| b.width(px(1.)).dashed().radius(px(4.)).color(border_color))
            .background(|b| b.color(bg))
            .color(fg)
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
    }
}

impl Theme for Ant {
    fn flexbox<PMsg: 'static>(&self, flex: &Flexbox<PMsg>) -> Style {
        // flex container style
        Style::default()
            .display(val::Flex)
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
            .merge(&flex.style)
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

    fn popover<'a, PMsg, C, T>(&self, popover: &Popover<'a, PMsg, C, T>) -> popover::Style {
        let container = Style::default().position(|conf| conf.relative());

        let panel = Style::default()
            .transition(|trans| {
                trans
                    .add("opacity", |conf| conf.duration(ms(150.)).ease())
                    .add("transform", |conf| conf.duration(ms(150.)).ease())
                    .add("visibility", |conf| conf.duration(ms(150.)).ease())
            })
            .position(|conf| conf.absolute())
            .background(|conf| conf.color(self.white()))
            .border(|conf| conf.color(self.border(false)).solid().width(px(1.)))
            .padding(|conf| conf.x(px(4.)).y(px(2)))
            .config_block(|style| {
                if popover.is_visible() {
                    style.opacity(1.).visibility(val::Visible)
                } else {
                    style.visibility(val::Hidden).opacity(0.)
                }
            });

        popover::Style { container, panel }
    }

    fn svg_icon<PMsg: 'static>(&self, icon: &SvgIcon<PMsg>) -> Style {
        Style::default()
            .try_merge(icon.color.as_ref())
            .merge(&icon.size)
    }

    fn html_icon<PMsg>(&self, icon: &HtmlIcon<PMsg>) -> Style {
        Style::default()
            .try_merge(icon.color.as_ref())
            .merge(&icon.size)
    }

    fn url_icon<PMsg>(&self, icon: &UrlIcon<PMsg>) -> Style {
        Style::default().merge(&icon.size)
    }

    // TODO: handle btn.size
    // TODO: handle is_loading()
    // TODO: handle btn.shape()
    // TODO: handle btn.is_block()
    // TODO: handle btn.is_ghost()
    fn button<PMsg>(&self, btn: &Button<PMsg>) -> Style {
        let padding = css::Padding::default().x(px(15.)).y(px(0.));

        let size = css::Size::default().height(px(32.));

        let cursor = btn
            .is_disabled()
            .then(|| -> css::Cursor { val::NotAllowed.into() })
            .unwrap_or_else(|| val::Pointer.into());

        match btn.kind {
            Some(button::Kind::Normal) | None => self.button_normal(btn),
            Some(button::Kind::Suggestion) => self.button_suggestion(btn),
            Some(button::Kind::Destructive) => self.button_destructive(btn),
            Some(button::Kind::Link) => self.button_link(btn),
            Some(button::Kind::Dashed) => self.button_dashed(btn),
        }
        .merge(&padding)
        .merge(&size)
        .transition(|trans| {
            trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
        })
        .text(|t| {
            t.decoration(|d| d.line(val::None))
                .line_height(1.499)
                .white_space(val::Nowrap)
        })
        .add(St::Outline, val::None)
        .cursor(cursor)
        .add(St::UserSelect, val::None)
        .add(St::BoxSizing, val::BorderBox)
        .add(St::FontSize, px(14.))
        .add(St::FontWeight, "400")
        .merge(&btn.style)
    }

    fn switch<PMsg>(&self, switch: &Switch<PMsg>) -> <Switch<PMsg> as Themeable>::StyleMap {
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

        let opacity = if switch.is_disabled() {
            Some(0.4)
        } else {
            None
        };

        let bg_style = Style::default()
            .try_add(St::Opacity, opacity)
            .cursor(cursor)
            .position(|pos| pos.relative())
            .background(|b| b.color(bg_color))
            .transition(|trans| {
                trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .border(|b| b.transparent().width(px(0.)).radius(px(height / 2.)).none())
            .display(val::InlineBlock)
            .text(|t| t.decoration(|d| d.line(val::None)))
            // .add(St::Outline, val::None)
            .add(St::UserSelect, val::None)
            .add(St::BoxSizing, val::BorderBox)
            .size(|s| s.height(px(height)).min_width(px(width)));

        let translatex = if switch.is_toggled() {
            Some(format!("translateX({})", px(width / 2.)))
        } else {
            None
        };

        let btn_style = Style::default()
            .position(|pos| pos.absolute().top(px(top)).left(px(left)))
            .try_add(St::Transform, translatex)
            .transition(|trans| {
                trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .background(|b| b.color(self.white()))
            .border(|b| b.width(px(0.)).transparent().none().radius(0.5))
            .add(St::BoxShadow, "0 2px 4px 0 rgba(0, 35, 11, 0.2)")
            .size(|s| s.resize(px(btn_size), px(btn_size)));

        (bg_style, btn_style)
    }

    fn checkbox<PMsg>(&self, checkbox: &Checkbox<PMsg>) -> <Checkbox<PMsg> as Themeable>::StyleMap {
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

        let input_style = Style::default()
            .transition(|trans| {
                trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .cursor(cursor)
            .display(val::Flex)
            .justify_content(val::Center)
            .align_items(val::Center)
            .add(St::WebkitAppearance, val::None)
            // .add(St::Appearance, val::None)
            .size(|s| s.resize(px(16.), px(16.)))
            .border(|b| b.solid().width(px(1.)).color(border).radius(px(2.)))
            .background(|b| b.color(bg))
            .color(fg);

        let btn_style = if checkbox.is_toggled() {
            Style::default()
                .cursor(cursor)
                .transition(|trans| {
                    trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
                })
                .size(|s| s.resize(0.2, 0.55))
                .merge(
                    &css::Border::default()
                        .bottom(|side| side.solid().width(px(2.)).color(fg))
                        .right(|side| side.solid().width(px(2.)).color(fg)),
                )
                .margin(|m| m.bottom(|_| px(2.).into()))
                .add(St::Transform, "rotate(45deg)")
        } else {
            Style::default()
        };

        let lbl_style = if checkbox.is_disabled() {
            Style::default().color(self.disable(false))
        } else {
            Style::default()
        }
        .transition(|trans| {
            trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
        })
        .cursor(cursor)
        .display(val::Flex)
        .gap(px(4.));

        (input_style, btn_style, lbl_style)
    }

    fn radio<PMsg>(&self, radio: &Radio<PMsg>) -> <Radio<PMsg> as Themeable>::StyleMap {
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

        let input_style = Style::default()
            .transition(|trans| {
                trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .cursor(cursor)
            .display(val::Flex)
            .justify_content(val::Center)
            .align_items(val::Center)
            .add(St::WebkitAppearance, val::None)
            .color(fg)
            .size(|s| s.resize(px(16.), px(16.)))
            .border(|b| b.solid().width(px(1.)).color(border).radius(0.5))
            .background(|b| b.color(bg));

        let btn_style = if radio.is_toggled() {
            Style::default()
                .cursor(cursor)
                .size(|s| s.resize(0.6, 0.6))
                .border(|b| b.none().radius(0.5))
                .background(|b| b.color(fg))
        } else {
            Style::default()
        }
        .transition(|trans| {
            trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
        });

        let lbl_style = if radio.is_disabled() {
            Style::default().color(self.disable(false))
        } else {
            Style::default()
        }
        .transition(|trans| {
            trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
        })
        .cursor(cursor)
        .display(val::Flex)
        .gap(px(4.));

        (input_style, btn_style, lbl_style)
    }

    fn entry<PMsg>(&self, entry: &Entry<PMsg>) -> <Entry<PMsg> as Themeable>::StyleMap {
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

        let container = Style::default()
            .transition(|trans| {
                trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .display(val::Flex)
            .align_items(val::Center)
            .justify_content(val::Center)
            .padding(|p| p.y(px(4.)).x(px(11.)))
            .gap(px(4.))
            .background(|b| b.color(bg))
            .border(|b| b.solid().width(px(1.)).color(border).radius(px(4.)))
            .size(|s| s.width(1.))
            .cursor(cursor);

        let input = Style::default()
            .transition(|trans| {
                trans.all(|val| val.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.))
            })
            .size(|s| s.width(1.).height(px(32.)))
            .color(self.primary_text(false))
            .border(|b| b.none())
            .background(|bg| bg.transparent())
            .add(St::WebkitAppearance, val::None)
            .cursor(cursor);

        entry::StyleMap { container, input }
    }
}
