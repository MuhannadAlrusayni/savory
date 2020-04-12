use crate::{prelude::*, theme::ThemeImpl};

use palette::{Hsl, Hsla};
use savory_html::css::{
    self,
    unit::{em, ms, px, sec},
    values as val, Cursor, St,
};

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

    variants
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
}

impl ThemeImpl for Ant {
    fn flexbox<'a>(&self) -> flexbox::ThemeStyler<'a> {
        let styler = |lens: &flexbox::FlexboxLens<'a>| {
            flexbox::Style::default().and_flexbox(|conf| {
                conf.set_display(val::Flex)
                    .try_set_flex_direction(lens.direction.copied())
                    .try_set_flex_wrap(lens.wrap.copied())
                    .try_set_justify_content(lens.justify_content.copied())
                    .try_set_align_items(lens.align_items.copied())
                    .try_set_align_content(lens.align_content.copied())
                    .try_set_gap(lens.gap.copied())
            })
        };
        styler.into()
    }

    fn flexbox_item<'a>(&self) -> flexbox::item::ThemeStyler<'a> {
        let styler = |lens: &flexbox::item::ItemLens<'a>| {
            flexbox::item::Style::default().and_item(|conf| {
                conf.try_add(St::Order, lens.order.copied())
                    .try_add(St::FlexGrow, lens.grow.copied())
                    .try_add(St::FlexShrink, lens.shrink.copied())
                    .try_merge(lens.basis)
                    .try_merge(lens.align_self)
            })
        };
        styler.into()
    }

    // fn grid(&self) -> Style;

    fn popover<'a>(&self) -> popover::ThemeStyler<'a> {
        let border = self.border(false);
        let white = self.white();
        let styler = move |lens: &popover::PopoverLens<'a>| {
            popover::Style::default()
                .and_popover(|conf| conf.and_position(|conf| conf.relative()))
                .and_panel(|conf| {
                    conf.and_transition(|conf| {
                        conf.add("opacity", |conf| conf.set_duration(ms(150.)).ease())
                            .add("transform", |conf| conf.set_duration(ms(150.)).ease())
                            .add("visibility", |conf| conf.set_duration(ms(150.)).ease())
                    })
                    .set_display(val::Flex)
                    .and_position(|conf| conf.absolute())
                    .and_background(|conf| conf.set_color(white))
                    .and_border(|conf| {
                        conf.set_color(border)
                            .none()
                            .set_width(px(0.))
                            .set_radius(px(4.))
                    })
                    .add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)")
                    .and_padding(|conf| conf.set_x(px(4.)).set_y(px(2)))
                    .and_margin(|conf| conf.set_top(px(*lens.offset)))
                    .config_if_else(
                        *lens.toggled,
                        |conf| conf.set_opacity(1.).set_visibility(val::Visible),
                        |conf| conf.set_visibility(val::Hidden).set_opacity(0.),
                    )
                })
        };
        styler.into()
    }

    fn svg_icon<'a>(&self) -> icon::svg::ThemeStyler<'a> {
        let styler = |_: &icon::svg::SvgLens<'a>| icon::svg::Style::default();
        styler.into()
    }

    fn html_icon<'a>(&self) -> icon::html::ThemeStyler<'a> {
        let styler = |_: &icon::html::HtmlLens<'a>| icon::html::Style::default();
        styler.into()
    }

    fn url_icon<'a>(&self) -> icon::url::ThemeStyler<'a> {
        let styler = |_: &icon::url::UrlLens<'a>| icon::url::Style::default();
        styler.into()
    }

    // TODO: handle is_loading()
    // TODO: handle btn.is_block()
    // TODO: handle btn.is_ghost()
    fn button<'a>(&self) -> button::ThemeStyler<'a> {
        let brand_400 = self.brand(Variant::L400);
        let brand_500 = self.brand(Variant::L500);
        let gray_200 = self.gray(Variant::L200);
        let gray_400 = self.gray(Variant::L400);
        let gray_600 = self.gray(Variant::D600);
        let disable = self.disable(false);
        let white = self.white();
        let primary_text = self.primary_text(false);
        let border = self.border(false);
        let dust_red_500 = self.dust_red(Variant::L500);
        let dust_red_400 = self.dust_red(Variant::L400);

        let common_button = move |fg, bg, border| {
            css::Style::default()
                .and_border(|conf| {
                    conf.set_width(px(1.))
                        .solid()
                        .set_radius(px(4.))
                        .set_color(border)
                })
                .and_background(|conf| conf.set_color(bg))
                .and_text(|conf| conf.set_color(fg))
                .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
        };
        let button_normal = move |lens: &button::ButtonLens<'a>| {
            // colors
            let (fg, bg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                // btn is disabled
                (true, _, _) => (gray_200, disable, gray_400),
                // btn is not focused or hovered
                (false, false, false) => (primary_text, white, border),
                // btn is hovered or focused
                _ => (white, brand_500, brand_500),
            };
            common_button(fg, bg, border)
        };
        let button_suggestion = move |lens: &button::ButtonLens<'a>| {
            // colors
            let (fg, bg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                // btn is disabled
                (true, _, _) => (gray_200, disable, gray_400),
                // btn is not focused or hovered
                (false, false, false) => (brand_500, white, brand_500),
                // btn is hovered or focused
                _ => (brand_400, white, brand_400),
            };
            common_button(fg, bg, border)
        };
        let button_destructive = move |lens: &button::ButtonLens<'a>| {
            // colors
            let (fg, bg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                // btn is disabled
                (true, _, _) => (gray_200, disable, gray_400),
                // btn is not focused or hovered
                (false, false, false) => (dust_red_500, white, dust_red_500),
                // btn is hovered or focused
                _ => (dust_red_400, white, dust_red_400),
            };
            common_button(fg, bg, border)
        };
        let button_link = move |lens: &button::ButtonLens<'a>| {
            // colors
            let (bg, fg) = match (lens.disabled, lens.focused, lens.mouse_over) {
                (true, _, _) => (white, disable),
                // btn is not focused or hovered
                (false, false, false) => (white, brand_500),
                // btn is hovered or focused
                _ => (white, brand_400),
            };

            css::Style::default()
                .and_text(|conf| conf.set_color(fg))
                .and_border(|conf| {
                    conf.set_width(px(0.))
                        .solid()
                        .set_radius(px(4.))
                        .set_color(bg)
                })
                .and_background(|conf| conf.set_color(bg))
        };
        let button_dashed = move |lens: &button::ButtonLens<'a>| {
            // colors
            let (bg, fg, border_color) = match (lens.disabled, lens.focused, lens.mouse_over) {
                (true, _, _) => (gray_200, disable, gray_400),
                // btn is not focused or hovered
                (false, false, false) => (white, primary_text, gray_600),
                // btn is hovered or focused
                _ => (white, brand_500, brand_500),
            };

            common_button(fg, bg, border_color).and_border(|conf| conf.dashed())
            // css::Style::default()
            //     .and_border(|conf| {
            //         conf.set_width(px(1.))
            //             .dashed()
            //             .set_radius(px(4.))
            //             .set_color(border_color)
            //     })
            //     .and_background(|conf| conf.set_color(bg))
            //     .and_text(|conf| conf.set_color(fg))
            //     .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
        };

        let styler = move |lens: &button::ButtonLens<'a>| {
            let cursor: Cursor = if *lens.disabled {
                val::NotAllowed.into()
            } else {
                val::Initial.into()
            };

            button::Style::default().and_button(|_| {
                match lens.kind {
                    Some(button::Kind::Normal) | None => button_normal(lens),
                    Some(button::Kind::Suggestion) => button_suggestion(lens),
                    Some(button::Kind::Destructive) => button_destructive(lens),
                    Some(button::Kind::Link) => button_link(lens),
                    Some(button::Kind::Dashed) => button_dashed(lens),
                }
                .and_padding(|conf| conf.set_x(px(15.)).set_y(px(0.)))
                .and_size(|conf| conf.set_all_heights(px(32.)))
                .and_transition(|conf| {
                    conf.all(|val| {
                        val.set_duration(sec(0.3))
                            .cubic_bezier(0.645, 0.045, 0.355, 1.)
                    })
                })
                .and_text(|conf| {
                    conf.and_decoration(|d| d.set_line(val::None))
                        .set_line_height(1.499)
                        .set_white_space(val::Nowrap)
                })
                .and_font(|conf| conf.set_size(px(14.)).weight_400())
                .set_cursor(cursor)
                .add(St::Outline, val::None)
                .add(St::UserSelect, val::None)
                .add(St::BoxSizing, val::BorderBox)
            })
        };
        styler.into()
    }

    fn switch<'a>(&self) -> switch::ThemeStyler<'a> {
        let brand_500 = self.brand(Variant::L500);
        let gray_500 = self.gray(Variant::L500);
        let white = self.white();
        let width = 44.;
        let height = 22.;
        let btn_size = height - 3.;
        let top = 3. / 2.;
        let left = 3. / 2.;

        let styler = move |lens: &switch::SwitchLens<'a>| {
            let cursor: Cursor = if *lens.disabled {
                val::NotAllowed.into()
            } else {
                val::Initial.into()
            };

            switch::Style::default()
                .and_switch(|conf| {
                    let bg_color = if *lens.toggled { brand_500 } else { gray_500 };
                    conf.config_if(*lens.disabled, |conf| conf.set_opacity(0.4))
                        .set_cursor(cursor)
                        .and_position(|conf| conf.relative())
                        .and_background(|conf| conf.set_color(bg_color))
                        .and_transition(|conf| {
                            conf.all(|val| {
                                val.set_duration(sec(0.3))
                                    .cubic_bezier(0.645, 0.045, 0.355, 1.)
                            })
                        })
                        .and_border(|conf| {
                            conf.transparent()
                                .set_width(px(0.))
                                .set_radius(px(height / 2.))
                                .none()
                        })
                        .set_display(val::InlineBlock)
                        .and_text(|conf| conf.and_decoration(|d| d.set_line(val::None)))
                        // .add(St::Outline, val::None)
                        .add(St::UserSelect, val::None)
                        .add(St::BoxSizing, val::BorderBox)
                        .and_size(|conf| conf.set_all_heights(px(height)).set_all_widths(px(width)))
                })
                .and_button(|conf| {
                    conf.config_if(*lens.toggled, |conf| {
                        conf.add(St::Transform, format!("translateX({})", px(width / 2.)))
                    })
                    .and_position(|conf| conf.absolute().set_top(px(top)).set_left(px(left)))
                    .and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .and_background(|conf| conf.set_color(white))
                    .and_border(|conf| conf.set_width(px(0.)).transparent().none().set_radius(0.5))
                    .add(St::BoxShadow, "0 2px 4px 0 rgba(0, 35, 11, 0.2)")
                    .and_size(|conf| conf.resize(px(btn_size), px(btn_size)))
                })
        };
        styler.into()
    }

    fn checkbox<'a>(&self) -> checkbox::ThemeStyler<'a> {
        let gray_200 = self.gray(Variant::L200);
        let gray_400 = self.gray(Variant::L400);
        let brand_500 = self.brand(Variant::L500);
        let white = self.white();
        let disable = self.disable(false);
        let border = self.border(false);

        let styler = move |lens: &checkbox::CheckboxLens<'a>| {
            let (bg, fg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                (true, _, _) => (gray_200, disable, gray_400),
                (false, false, false) if *lens.toggled => (brand_500, white, brand_500),
                (false, false, false) if !*lens.toggled => (white, white, border),
                _ => {
                    if *lens.toggled {
                        (brand_500, white, brand_500)
                    } else {
                        (white, white, brand_500)
                    }
                }
            };

            let cursor: Cursor = if *lens.disabled {
                val::NotAllowed.into()
            } else {
                val::Initial.into()
            };

            checkbox::Style::default()
                .and_checkbox(|conf| {
                    conf.and_transition(|conf| {
                        conf.all(|conf| {
                            conf.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .set_cursor(cursor)
                    .set_display(val::Flex)
                    .set_justify_content(val::Center)
                    .set_align_items(val::Center)
                    .add(St::WebkitAppearance, val::None)
                    .add(St::Appearance, val::None)
                    .and_size(|conf| conf.set_all(px(16.)))
                    .and_border(|conf| {
                        conf.solid()
                            .set_width(px(1.))
                            .set_color(border)
                            .set_radius(px(2.))
                    })
                    .and_background(|conf| conf.set_color(bg))
                    .and_text(|conf| conf.set_color(fg))
                })
                .and_button(|conf| {
                    conf.config_if(*lens.toggled, |conf| {
                        conf.set_cursor(cursor)
                            .and_transition(|conf| {
                                conf.all(|val| {
                                    val.set_duration(sec(0.3))
                                        .cubic_bezier(0.645, 0.045, 0.355, 1.)
                                })
                            })
                            .and_size(|conf| conf.resize(0.2, 0.55))
                            .and_border(|conf| {
                                conf.and_bottom(|conf| conf.solid().set_width(px(2.)).set_color(fg))
                                    .and_right(|conf| conf.solid().set_width(px(2.)).set_color(fg))
                            })
                            .and_margin(|conf| conf.set_bottom(px(2.)))
                            .add(St::Transform, "rotate(45deg)")
                    })
                })
                .and_label(|conf| {
                    conf.config_if(*lens.disabled, |conf| {
                        conf.and_text(|conf| conf.set_color(disable))
                    })
                    .and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .set_cursor(cursor)
                    .set_display(val::Flex)
                    .set_gap(px(4))
                })
        };
        styler.into()
    }

    fn radio<'a>(&self) -> radio::ThemeStyler<'a> {
        let disable = self.disable(false);
        let gray_200 = self.gray(Variant::L200);
        let gray_400 = self.gray(Variant::L400);
        let white = self.white();
        let brand_500 = self.brand(Variant::L500);
        let border = self.border(false);

        let styler = move |lens: &radio::RadioLens<'a>| {
            let (bg, fg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                (true, _, _) => (gray_200, disable, gray_400),
                (false, false, false) if *lens.toggled => (white, brand_500, brand_500),
                (false, false, false) if !*lens.toggled => (white, white, border),
                _ => {
                    if *lens.toggled {
                        (white, brand_500, brand_500)
                    } else {
                        (white, white, brand_500)
                    }
                }
            };

            let cursor: Cursor = if *lens.disabled {
                val::NotAllowed.into()
            } else {
                val::Initial.into()
            };

            radio::Style::default()
                .and_radio(|conf| {
                    conf.and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .set_cursor(cursor)
                    .set_display(val::Flex)
                    .and_margin(|conf| conf.zero())
                    .set_justify_content(val::Center)
                    .set_align_items(val::Center)
                    .add(St::WebkitAppearance, val::None)
                    .and_text(|conf| conf.set_color(fg))
                    .and_size(|conf| conf.set_all(px(16)))
                    .and_border(|conf| {
                        conf.solid()
                            .set_width(px(1.))
                            .set_color(border)
                            .set_radius(0.5)
                    })
                    .and_background(|conf| conf.set_color(bg))
                })
                .and_button(|conf| {
                    conf.config_if(*lens.toggled, |conf| {
                        conf.set_cursor(cursor)
                            .and_size(|conf| conf.resize(0.6, 0.6))
                            .and_border(|conf| conf.none().set_radius(0.5))
                            .and_background(|conf| conf.set_color(fg))
                    })
                    .and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                })
                .and_label(|conf| {
                    conf.config_if(*lens.disabled, |conf| {
                        conf.and_text(|conf| conf.set_color(disable))
                    })
                    .and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .set_cursor(cursor)
                    .set_display(val::Flex)
                    .set_align_items(val::Center)
                    .set_gap(px(4.))
                })
        };
        styler.into()
    }

    fn entry<'a>(&self) -> entry::ThemeStyler<'a> {
        let gray_200 = self.gray(Variant::L200);
        let gray_400 = self.gray(Variant::L400);
        let disable = self.disable(false);
        let white = self.white();
        let primary_text = self.primary_text(false);
        let border = self.border(false);
        let brand_500 = self.brand(Variant::L500);

        // em unit
        let container_width = 14.;
        let container_height = 1.5;
        let input_width = container_width - 0.6;
        let input_height = 1.;
        let font_size = 1.;
        // px unit
        let container_radius = 4;
        let container_border_width = 1;

        let styler = move |lens: &entry::EntryLens<'a>| {
            let (bg, fg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                (true, _, _) => (gray_200, disable, gray_400),
                (false, false, false) => (white, primary_text, border),
                _ => (white, primary_text, brand_500),
            };

            let cursor: Cursor = if *lens.disabled {
                val::NotAllowed.into()
            } else {
                val::Initial.into()
            };

            entry::Style::default()
                .and_container(|conf| {
                    conf.and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .set_display(val::Flex)
                    .set_align_items(val::Center)
                    .set_justify_content(val::Center)
                    // .and_padding(|conf| conf.set_y(px(4.)).set_x(px(11.)))
                    // .set_gap(px(4.))
                    .and_background(|conf| conf.set_color(bg))
                    .and_border(|conf| {
                        conf.solid()
                            .set_width(px(container_border_width))
                            .set_color(border)
                            .set_radius(px(container_radius))
                    })
                    .and_size(|conf| {
                        conf.set_all_widths(em(container_width))
                            .set_all_heights(em(container_height))
                    })
                    .set_cursor(cursor)
                })
                .and_input(|conf| {
                    conf.and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .and_size(|conf| conf.set_width(em(input_width)).set_height(em(input_height)))
                    .and_text(|conf| conf.set_color(fg))
                    .and_font(|conf| conf.set_size(em(font_size)))
                    .and_border(|conf| conf.none())
                    .and_background(|conf| conf.transparent())
                    .add(St::WebkitAppearance, val::None)
                    .set_cursor(cursor)
                })
        };
        styler.into()
    }

    fn spin_entry<'a>(&self) -> spin_entry::ThemeStyler<'a> {
        // colors
        let gray_200 = self.gray(Variant::L200);
        let gray_300 = self.gray(Variant::L300);
        let gray_400 = self.gray(Variant::L400);
        let brand_500 = self.brand(Variant::L500);
        let disable = self.disable(false);
        let white = self.white();
        let primary_text = self.primary_text(false);
        let secondary_text = self.secondary_text(false);
        let border = self.border(false);

        // em unit used here
        let width = 4.;
        let height = 1.5;
        let btn_width = 1.;
        let btn_height = height / 2.;
        let btn_lbl_font_size = 0.6;
        let input_width = width - 0.6;
        let input_height = 1.;
        let font_size = 1.;
        let btn_mouse_over_height = btn_height + 0.10;
        let btn_mouse_over_height_2 = btn_height - 0.10;
        // px used here
        let radius = 4.;
        let border_width = 1.;

        // side border conf for btns
        let border_conf = move |conf: css::border::Side| {
            conf.solid().set_width(px(border_width)).set_color(gray_300)
        };
        // centered flex conf
        let centered_flex = move |conf: css::Style| {
            conf.set_display(val::Flex)
                .set_align_items(val::Center)
                .set_justify_content(val::Center)
        };

        let styler = move |lens: &spin_entry::SpinEntryLens<'a>| {
            let (bg, fg, border) = match (lens.disabled, lens.focused, lens.mouse_over) {
                (true, _, _) => (gray_200, disable, gray_400),
                (false, false, false) => (white, primary_text, border),
                _ => (white, primary_text, brand_500),
            };

            let cursor: Cursor = if *lens.disabled {
                val::NotAllowed.into()
            } else {
                val::Initial.into()
            };

            let (inc_btn_height, dec_btn_height) = match (
                lens.increment_button.mouse_over,
                lens.decrement_button.mouse_over,
            ) {
                (true, _) => (btn_mouse_over_height, btn_mouse_over_height_2),
                (_, true) => (btn_mouse_over_height_2, btn_mouse_over_height),
                (false, false) => (btn_height, btn_height),
            };

            let btns_opacity = if *lens.focused
                || *lens.mouse_over
                || *lens.increment_button.focused
                || *lens.decrement_button.focused
            {
                1.0
            } else {
                0.0
            };

            spin_entry::Style::default()
                .and_spin_entry(|conf| {
                    centered_flex(conf)
                        .and_position(|conf| conf.relative())
                        .and_transition(|conf| {
                            conf.all(|val| {
                                val.set_duration(sec(0.3))
                                    .cubic_bezier(0.645, 0.045, 0.355, 1.)
                            })
                        })
                        .and_background(|conf| conf.set_color(bg))
                        .and_border(|conf| {
                            conf.solid()
                                .set_width(px(border_width))
                                .set_color(border)
                                .set_radius(px(radius))
                        })
                        .and_size(|conf| conf.set_all_widths(em(width)).set_all_heights(em(height)))
                        .set_cursor(cursor)
                })
                .and_input(|conf| {
                    conf.and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .and_size(|conf| conf.set_width(em(input_width)).set_height(em(input_height)))
                    .and_text(|conf| conf.set_color(fg))
                    .and_font(|conf| conf.set_size(em(font_size)))
                    .and_border(|conf| conf.none())
                    .and_background(|conf| conf.transparent())
                    .add(St::WebkitAppearance, val::None)
                    .set_cursor(cursor)
                })
                .and_increment_button(|conf| {
                    conf.and_label(|conf| {
                        conf.and_label(|conf| {
                            conf.and_font(|conf| conf.set_size(em(btn_lbl_font_size)))
                        })
                    })
                    .and_button(|conf| {
                        centered_flex(conf)
                            .set_opacity(btns_opacity)
                            .and_transition(|conf| {
                                conf.all(|val| {
                                    val.set_duration(sec(0.3))
                                        .cubic_bezier(0.645, 0.045, 0.355, 1.)
                                })
                            })
                            .and_text(|conf| conf.set_color(secondary_text))
                            .and_background(|conf| conf.set_color(white))
                            .and_border(|conf| {
                                conf.none().set_top_right(px(radius)).and_left(border_conf)
                            })
                            .and_size(|conf| {
                                conf.set_width(em(btn_width)).set_height(em(inc_btn_height))
                            })
                            .and_position(|conf| conf.absolute().set_top(px(0)).set_right(px(0)))
                    })
                })
                .and_decrement_button(|conf| {
                    conf.and_label(|conf| {
                        conf.and_label(|conf| {
                            conf.and_font(|conf| conf.set_size(em(btn_lbl_font_size)))
                        })
                    })
                    .and_button(|conf| {
                        centered_flex(conf)
                            .set_opacity(btns_opacity)
                            .and_transition(|conf| {
                                conf.all(|val| {
                                    val.set_duration(sec(0.3))
                                        .cubic_bezier(0.645, 0.045, 0.355, 1.)
                                })
                            })
                            .and_text(|conf| conf.set_color(secondary_text))
                            .and_background(|conf| conf.set_color(white))
                            .and_border(|conf| {
                                conf.none()
                                    .set_bottom_right(px(radius))
                                    .and_top(border_conf)
                                    .and_left(border_conf)
                            })
                            .and_size(|conf| {
                                conf.set_width(em(btn_width)).set_height(em(dec_btn_height))
                            })
                            .and_position(|conf| conf.absolute().set_bottom(px(0)).set_right(px(0)))
                    })
                })
        };
        styler.into()
    }

    fn dialog<'a>(&self) -> dialog::ThemeStyler<'a> {
        let primary_text = self.primary_text(false);
        let white = self.white();

        let styler = move |lens: &dialog::DialogLens<'a>| {
            dialog::Style::default()
                .and_dialog_background(|conf| {
                    conf.and_position(|conf| {
                        conf.fixed()
                            .set_right(px(0))
                            .set_top(px(0))
                            .set_bottom(px(0))
                            .set_left(px(0))
                            .set_z_index(500)
                    })
                    .and_background(|conf| conf.set_color(Hsla::new(0.0, 0., 0., 0.5)))
                    .and_size(|conf| conf.full())
                    .and_transition(|conf| {
                        conf.all(|val| {
                            val.set_duration(sec(0.3))
                                .cubic_bezier(0.645, 0.045, 0.355, 1.)
                        })
                    })
                    .config(|conf| {
                        use dialog::State::*;
                        match lens.state {
                            Closed => conf.set_display(val::None),
                            Opening => conf
                                .set_display(val::Flex)
                                .set_align_content(val::Center)
                                .set_align_items(val::Center)
                                .set_justify_content(val::Center)
                                .set_opacity(0.0),
                            Opened => conf
                                .set_display(val::Flex)
                                .set_align_content(val::Center)
                                .set_align_items(val::Center)
                                .set_justify_content(val::Center)
                                .set_opacity(1.0),
                            Closing => conf
                                .set_display(val::Flex)
                                .set_align_content(val::Center)
                                .set_align_items(val::Center)
                                .set_justify_content(val::Center)
                                .set_opacity(0.0),
                        }
                    })
                })
                .and_dialog(|conf| {
                    conf.and_background(|conf| conf.set_color(white))
                        .and_text(|conf| conf.set_color(primary_text))
                        .and_border(|conf| conf.set_radius(px(2)))
                        .set_display(val::Flex)
                        .set_flex_direction(val::Column)
                        .set_align_content(val::Center)
                        .set_align_items(val::Center)
                        .set_justify_content(val::Center)
                        .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
                })
        };
        styler.into()
    }

    fn header_bar<'a>(&self) -> header_bar::ThemeStyler<'a> {
        // colors
        let secondary_text = self.secondary_text(false);
        let primary_text = self.primary_text(false);
        let title = self.title(false);
        let gray_600 = self.gray(Variant::D600);

        // em units
        let height = 2.5;
        let button_size = height;
        let subtitle_font_size = 0.6;
        let title_container_padding = 1.;

        let styler = move |lens: &header_bar::HeaderBarLens<'a>| {
            header_bar::Style::default()
                .and_title(|conf| {
                    conf.and_label(|conf| conf.and_text(|conf| conf.set_color(title)))
                })
                .and_subtitle(|conf| {
                    conf.and_label(|conf| {
                        conf.and_text(|conf| conf.set_color(secondary_text))
                            .and_font(|conf| conf.set_size(em(subtitle_font_size)))
                    })
                })
                .and_titles_container(|conf| {
                    conf.set_display(val::Flex)
                        .set_flex_direction(val::Column)
                        .set_justify_content(val::Center)
                        .set_align_items(val::Center)
                        .set_align_content(val::Center)
                        .set_flex_wrap(val::Nowrap)
                        .and_padding(|conf| conf.set_x(em(title_container_padding)))
                })
                .and_close_button(|conf| {
                    conf.and_button(|conf| {
                        conf.and_size(|conf| conf.set_all(em(button_size)))
                            .config(|conf| {
                                if let Some(ref btn) = lens.close_button {
                                    if *btn.mouse_over {
                                        conf.and_text(|conf| conf.set_color(primary_text))
                                    } else {
                                        conf.and_text(|conf| conf.set_color(secondary_text))
                                    }
                                } else {
                                    conf
                                }
                            })
                            .set_cursor(val::Pointer)
                            .and_background(|conf| conf.transparent())
                            .and_border(|conf| conf.none())
                            .add(St::Outline, val::None)
                            .add(St::UserSelect, val::None)
                            .add(St::BoxSizing, val::BorderBox)
                            .and_font(|conf| conf.set_size(em(1.)))
                            .and_margin(|conf| conf.zero())
                            .and_padding(|conf| conf.zero())
                    })
                })
                .and_header_bar(|conf| {
                    conf.set_display(val::Flex)
                        .set_justify_content(val::SpaceBetween)
                        .set_align_items(val::Stretch)
                        .set_align_content(val::Stretch)
                        .set_flex_wrap(val::Nowrap)
                        .and_size(|conf| conf.set_all_heights(em(height)))
                        .and_border(|conf| {
                            conf.and_bottom(|conf| {
                                conf.solid().set_width(px(1)).set_color(gray_600)
                            })
                        })
                })
        };
        styler.into()
    }

    fn label<'a>(&self) -> label::ThemeStyler<'a> {
        let styler = move |_: &label::LabelLens<'a>| {
            label::Style::default().and_label(|conf| conf.and_font(|conf| conf.set_size(em(1.))))
        };
        styler.into()
    }

    fn progress_bar<'a>(&self) -> progress_bar::ThemeStyler<'a> {
        // colors
        let brand_500 = self.brand(Variant::L500);
        let gray_300 = self.gray(Variant::L300);
        let suggestion = self.suggestion();
        let destructive = self.destructive();

        // em units
        let height = 0.7;
        let radius = height / 2.;

        // percent units
        let width = 1.;

        let styler = move |lens: &progress_bar::ProgressBarLens<'a>| {
            progress_bar::Style::default()
                .and_progress_bar(|conf| {
                    conf.and_border(|conf| conf.set_radius(em(radius)).none())
                        .set_background(gray_300)
                        .and_size(|conf| conf.set_all_heights(em(height)).set_width(width))
                        .and_transition(|conf| conf.all(|val| val.set_duration(sec(0.3)).ease()))
                        .and_position(|conf| conf.relative())
                        .add(St::Overflow, val::Hidden)
                })
                .and_indicator(|conf| {
                    conf.and_border(|conf| conf.set_radius(em(radius)))
                        .config(|conf| {
                            let color = lens.color.copied().unwrap_or_else(|| {
                                match lens.state {
                                    progress_bar::State::Normal => brand_500,
                                    progress_bar::State::Success => suggestion,
                                    progress_bar::State::Failure => destructive,
                                }
                                .into()
                            });
                            conf.set_background(color)
                        })
                        .and_transition(|conf| {
                            conf.all(|val| {
                                val.set_duration(sec(0.3))
                                    .cubic_bezier(0.645, 0.045, 0.355, 1.)
                            })
                        })
                        .and_position(|conf| conf.absolute())
                        .and_size(|conf| {
                            let width = (lens.value - lens.min).abs() / (lens.max - lens.min);
                            conf.set_width(width as f32).set_all_heights(em(height))
                        })
                })
        };
        styler.into()
    }
}
