use crate::{
    css::{self, St, unit::px, color::Color, Style},
    el::{
        button::{self, Button},
        icon::{HtmlIcon, SvgIcon, UrlIcon},
        layout::flexbox::{self, Flexbox},
        propertie::*,
    },
    theme::Theme,
};

use palette::{luma::GammaLuma, rgb::Rgb, FromColor, Hsl, Hsla, Limited, Mix, Saturate, Shade};
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
        ).into();
        variants[(i as f32 - light_color_count).abs() as usize] = hsl;
    }
    // dark variants
    variants[5] = base_color;
    for i in 1..=dark_color_count as usize {
        let hsl: Hsl = Hsv::new(
            get_hue(&hsv, i as f32, false),
            get_saturation(&hsv, i as f32, false),
            get_value(&hsv, i as f32, false),
        ).into();
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
}

impl Theme for Ant {
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
        let gray_l2 = self.gray(Variant::L200);
        let gray_l4 = self.gray(Variant::L400);
        let white = self.white();
        let brand_l5 = self.brand(Variant::L500);
        let brand_l4 = self.brand(Variant::L400);
        let dust_red_l4 = self.dust_red(Variant::L400);
        let dust_red_l5 = self.dust_red(Variant::L500);

        let (bg_color, border_color, text_color) = if btn.is_disabled() {
            (gray_l2, gray_l4, gray_l4)
        } else {
            match (btn.is_focused(), btn.is_mouse_over()) {
                (true, _) | (_, true) => match btn.kind {
                    button::Kind::Normal => (white, brand_l5, brand_l5),
                    button::Kind::Suggestion => (brand_l4, brand_l4, self.title(true)),
                    button::Kind::Destructive => (dust_red_l4, dust_red_l4, self.title(true)),
                },
                (false, false) => match btn.kind {
                    button::Kind::Normal => (white, self.border(false), self.title(false)),
                    button::Kind::Suggestion => (brand_l5, brand_l5, self.title(true)),
                    button::Kind::Destructive => (dust_red_l5, dust_red_l5, self.title(true)),
                }
            }
        };

        let border = css::Border::default()
            .width(px(1.))
            .solid()
            .radius(px(4.))
            .color(border_color);

        let background = css::Background::default()
            .color(bg_color);

        let padding = css::Padding::default()
            .x(px(15.))
            .y(px(0.));

        let size = css::Size::default()
            .height(px(32.));

        let cursor = if btn.is_disabled() {
            "not-allowed"
        } else {
            "pointer"
        };

        let style = &btn.style;
        Style::default()
            .merge(&border)
            .merge(&background)
            .merge(&padding)
            .merge(&size)
            .add(St::Color, Color::from(text_color))
            .add(St::Transition, "all .3s cubic-bezier(.645, .045, .355, 1)")
            .add(St::TextDecoration, css::None)
            .add(St::Outline, css::None)
            .add(St::Cursor, cursor)
            .add(St::UserSelect, css::None)
            .add(St::FontSize, px(14.))
            .add(St::BoxSizing, "border-box")
            .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
            .add(St::LineHeight, "1.499")
            .add(St::FontWeight, "400")
            .add(St::WhiteSpace, css::NoWrap)
            .merge(&style.size)
            .merge(&style.border)
            .merge(&style.background)
            .merge(&style.margin)
            .merge(&style.padding)
    }
}
