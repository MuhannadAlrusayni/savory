use crate::{
    css::{
        self,
        unit::{em, ms, px, sec},
        values as val, Cursor, St,
    },
    prelude::*,
    theme::ThemeImpl,
};
use palette::{Hsl, Hsla};

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

    fn button_normal<'a>(&self, btn: ButtonLens<'a>) -> css::Style {
        todo!()
        // // colors
        // let (bg, fg, border_color) =
        //     match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
        //         // btn is disabled
        //         (true, _, _) => (
        //             self.gray(Variant::L200),
        //             self.disable(false),
        //             self.gray(Variant::L400),
        //         ),
        //         // btn is not focused or hovered
        //         (false, false, false) => {
        //             (self.white(), self.primary_text(false), self.border(false))
        //         }
        //         // btn is hovered or focused
        //         _ => (
        //             self.white(),
        //             self.brand(Variant::L500),
        //             self.brand(Variant::L500),
        //         ),
        //     };

        // css::Style::default()
        //     .and_border(|conf| {
        //         conf.set_width(px(1.))
        //             .solid()
        //             .set_radius(px(4.))
        //             .set_color(border_color)
        //     })
        //     .and_background(|conf| conf.set_color(bg))
        //     .and_text(|conf| conf.set_color(fg))
        //     .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
    }

    fn button_suggestion<'a>(&self, btn: ButtonLens<'a>) -> css::Style {
        todo!()
        // // colors
        // let (bg, fg, border_color) =
        //     match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
        //         // btn is disabled
        //         (true, _, _) => (
        //             self.gray(Variant::L200),
        //             self.disable(false),
        //             self.gray(Variant::L400),
        //         ),
        //         // btn is not focused or hovered
        //         (false, false, false) => (
        //             self.brand(Variant::L500),
        //             self.white(),
        //             self.brand(Variant::L500),
        //         ),
        //         // btn is hovered or focused
        //         _ => (
        //             self.brand(Variant::L400),
        //             self.white(),
        //             self.brand(Variant::L400),
        //         ),
        //     };

        // css::Style::default()
        //     .and_border(|conf| {
        //         conf.set_width(px(1.))
        //             .solid()
        //             .set_radius(px(4.))
        //             .set_color(border_color)
        //     })
        //     .and_background(|conf| conf.set_color(bg))
        //     .and_text(|conf| conf.set_color(fg))
        //     .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
    }

    fn button_destructive<'a>(&self, btn: ButtonLens<'a>) -> css::Style {
        todo!()
        // // colors
        // let (bg, fg, border_color) =
        //     match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
        //         // btn is disabled
        //         (true, _, _) => (
        //             self.gray(Variant::L200),
        //             self.disable(false),
        //             self.gray(Variant::L400),
        //         ),
        //         // btn is not focused or hovered
        //         (false, false, false) => (
        //             self.dust_red(Variant::L500),
        //             self.white(),
        //             self.dust_red(Variant::L500),
        //         ),
        //         // btn is hovered or focused
        //         _ => (
        //             self.dust_red(Variant::L400),
        //             self.white(),
        //             self.dust_red(Variant::L400),
        //         ),
        //     };

        // css::Style::default()
        //     .and_border(|conf| {
        //         conf.set_width(px(1.))
        //             .solid()
        //             .set_radius(px(4.))
        //             .set_color(border_color)
        //     })
        //     .and_background(|conf| conf.set_color(bg))
        //     .and_text(|conf| conf.set_color(fg))
        //     .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
    }

    fn button_link<'a>(&self, btn: ButtonLens<'a>) -> css::Style {
        todo!()
        // // colors
        // let (bg, fg) = match (btn.is_disabled(), btn.is_focused(), btn.is_mouse_over()) {
        //     (true, _, _) => (self.white(), self.disable(false)),
        //     // btn is not focused or hovered
        //     (false, false, false) => (self.white(), self.brand(Variant::L500)),
        //     // btn is hovered or focused
        //     _ => (self.white(), self.brand(Variant::L400)),
        // };

        // css::Style::default()
        //     .and_text(|conf| conf.set_color(fg))
        //     .and_border(|conf| {
        //         conf.set_width(px(0.))
        //             .solid()
        //             .set_radius(px(4.))
        //             .set_color(bg)
        //     })
        //     .and_background(|conf| conf.set_color(bg))
    }

    fn button_dashed<'a>(&self, btn: ButtonLens<'a>) -> css::Style {
        todo!()
        //     // colors
        //     let (bg, fg, border_color) = match (btn.disabled, btn.focused, btn.mouse_over) {
        //         (true, _, _) => (
        //             self.gray(Variant::L200),
        //             self.disable(false),
        //             self.gray(Variant::L400),
        //         ),
        //         // btn is not focused or hovered
        //         (false, false, false) => (
        //             self.white(),
        //             self.primary_text(false),
        //             self.gray(Variant::D600),
        //         ),
        //         // btn is hovered or focused
        //         _ => (
        //             self.white(),
        //             self.brand(Variant::L500),
        //             self.brand(Variant::L500),
        //         ),
        //     };

        //     css::Style::default()
        //         .and_border(|conf| {
        //             conf.set_width(px(1.))
        //                 .dashed()
        //                 .set_radius(px(4.))
        //                 .set_color(border_color)
        //         })
        //         .and_background(|conf| conf.set_color(bg))
        //         .and_text(|conf| conf.set_color(fg))
        //         .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
    }
}

impl ThemeImpl for Ant {
    fn flexbox<'a>(&self, flex: FlexboxLens<'a>) -> Style {
        todo!()
        // Style::default()
        //     .set_display(val::Flex)
        //     .try_set_flex_direction(flex.get_direction())
        //     .try_set_flex_wrap(flex.get_wrap())
        //     .try_set_justify_content(flex.get_justify_content())
        //     .try_set_align_items(flex.get_align_items())
        //     .try_set_align_content(flex.get_align_content())
        //     .try_set_gap(flex.get_gap())
        //     .merge(flex.user_style())
    }

    fn flexbox_item<'a>(&self, item: flexbox::ItemLens<'a>) -> Style {
        todo!()
        // Style::default()
        //     .try_add(St::Order, item.get_order())
        //     .try_add(St::FlexGrow, item.get_grow())
        //     .try_add(St::FlexShrink, item.get_shrink())
        //     .try_merge(item.get_basis().as_ref())
        //     .try_merge(item.get_align_self().as_ref())
        //     .merge(item.user_style())
    }

    // fn grid(&self) -> Style;

    fn popover<'a>(&self, popover: PopoverLens<'a>) -> Style {
        todo!()
        // let container = Style::default().and_position(|conf| conf.relative());

        // let panel = Style::default()
        //     .and_transition(|conf| {
        //         conf.add("opacity", |conf| conf.set_duration(ms(150.)).ease())
        //             .add("transform", |conf| conf.set_duration(ms(150.)).ease())
        //             .add("visibility", |conf| conf.set_duration(ms(150.)).ease())
        //     })
        //     .and_position(|conf| conf.absolute())
        //     .and_background(|conf| conf.set_color(self.white()))
        //     .and_border(|conf| {
        //         conf.set_color(self.border(false))
        //             .none()
        //             .set_width(px(0.))
        //             .set_radius(px(4.))
        //     })
        //     .add(St::BoxShadow, "0 2px 8px rgba(0, 35, 11, 0.15)")
        //     .and_padding(|conf| conf.set_x(px(4.)).set_y(px(2)))
        //     .and_margin(|conf| conf.set_top(px(popover.offset())))
        //     .config_if_else(
        //         popover.is_visible(),
        //         |conf| conf.set_opacity(1.).set_visibility(val::Visible),
        //         |conf| conf.set_visibility(val::Hidden).set_opacity(0.),
        //     );

        // popover::Style { container, panel }
    }

    fn svg_icon<'a>(&self, icon: SvgIconLens<'a>) -> Style {
        todo!()
        // Style::default().merge(icon.user_style())
    }

    fn html_icon<'a>(&self, icon: HtmlIconLens<'a>) -> Style {
        todo!()
        // Style::default().merge(icon.user_style())
    }

    fn url_icon<'a>(&self, icon: UrlIconLens<'a>) -> Style {
        todo!()
        // Style::default().merge(icon.user_style())
    }

    // TODO: handle is_loading()
    // TODO: handle btn.is_block()
    // TODO: handle btn.is_ghost()
    fn button<'a>(&self, btn: ButtonLens<'a>) -> Style {
        todo!()
        // let cursor: Cursor = if btn.is_disabled() {
        //     val::NotAllowed.into()
        // } else {
        //     val::Initial.into()
        // };

        // Style::default().insert("button", |_| {
        //     match btn.get_kind() {
        //         Some(button::Kind::Normal) | None => self.button_normal(btn),
        //         Some(button::Kind::Suggestion) => self.button_suggestion(btn),
        //         Some(button::Kind::Destructive) => self.button_destructive(btn),
        //         Some(button::Kind::Link) => self.button_link(btn),
        //         Some(button::Kind::Dashed) => self.button_dashed(btn),
        //     }
        //     .and_padding(|conf| conf.set_x(px(15.)).set_y(px(0.)))
        //     .and_size(|conf| conf.set_all_heights(px(32.)))
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .and_text(|conf| {
        //         conf.and_decoration(|d| d.set_line(val::None))
        //             .set_line_height(1.499)
        //             .set_white_space(val::Nowrap)
        //     })
        //     .and_font(|conf| conf.set_size(px(14.)).weight_400())
        //     .set_cursor(cursor)
        //     .add(St::Outline, val::None)
        //     .add(St::UserSelect, val::None)
        //     .add(St::BoxSizing, val::BorderBox)
        //     .merge(&btn.user_style().button)
        // })
    }

    fn switch<'a>(&self, switch: SwitchLens<'a>) -> Style {
        todo!()
        // let width = 44.;
        // let height = 22.;
        // let btn_size = height - 3.;
        // let top = 3. / 2.;
        // let left = 3. / 2.;

        // let cursor: Cursor = if switch.is_disabled() {
        //     val::NotAllowed.into()
        // } else {
        //     val::Initial.into()
        // };

        // let bg_color = if switch.is_toggled() {
        //     self.brand(Variant::L500)
        // } else {
        //     self.gray(Variant::L500)
        // };

        // let background = Style::default()
        //     .config_if(switch.is_disabled(), |conf| conf.set_opacity(0.4))
        //     .set_cursor(cursor)
        //     .and_position(|conf| conf.relative())
        //     .and_background(|conf| conf.set_color(bg_color))
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .and_border(|conf| {
        //         conf.transparent()
        //             .set_width(px(0.))
        //             .set_radius(px(height / 2.))
        //             .none()
        //     })
        //     .set_display(val::InlineBlock)
        //     .and_text(|conf| conf.and_decoration(|d| d.set_line(val::None)))
        //     // .add(St::Outline, val::None)
        //     .add(St::UserSelect, val::None)
        //     .add(St::BoxSizing, val::BorderBox)
        //     .and_size(|conf| conf.set_all_heights(px(height)).set_all_widths(px(width)))
        //     .merge(&switch.user_style().background);

        // let button = Style::default()
        //     .config_if(switch.is_toggled(), |conf| {
        //         conf.add(St::Transform, format!("translateX({})", px(width / 2.)))
        //     })
        //     .and_position(|conf| conf.absolute().set_top(px(top)).set_left(px(left)))
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .and_background(|conf| conf.set_color(self.white()))
        //     .and_border(|conf| conf.set_width(px(0.)).transparent().none().set_radius(0.5))
        //     .add(St::BoxShadow, "0 2px 4px 0 rgba(0, 35, 11, 0.2)")
        //     .and_size(|conf| conf.resize(px(btn_size), px(btn_size)))
        //     .merge(&switch.user_style().button);

        // switch::Style { background, button }
    }

    fn checkbox<'a>(&self, checkbox: CheckboxLens<'a>) -> Style {
        todo!()
        // let (bg, fg, border) = match (
        //     checkbox.is_disabled(),
        //     checkbox.is_focused(),
        //     checkbox.is_mouse_over(),
        // ) {
        //     (true, _, _) => (
        //         self.gray(Variant::L200),
        //         self.disable(false),
        //         self.gray(Variant::L400),
        //     ),
        //     (false, false, false) => {
        //         if checkbox.is_toggled() {
        //             (
        //                 self.brand(Variant::L500),
        //                 self.white(),
        //                 self.brand(Variant::L500),
        //             )
        //         } else {
        //             (self.white(), self.white(), self.border(false))
        //         }
        //     }
        //     _ => {
        //         if checkbox.is_toggled() {
        //             (
        //                 self.brand(Variant::L500),
        //                 self.white(),
        //                 self.brand(Variant::L500),
        //             )
        //         } else {
        //             (self.white(), self.white(), self.brand(Variant::L500))
        //         }
        //     }
        // };

        // let cursor: Cursor = if checkbox.is_disabled() {
        //     val::NotAllowed.into()
        // } else {
        //     val::Initial.into()
        // };

        // let input = Style::default()
        //     .and_transition(|conf| {
        //         conf.all(|conf| {
        //             conf.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .set_cursor(cursor)
        //     .set_display(val::Flex)
        //     .set_justify_content(val::Center)
        //     .set_align_items(val::Center)
        //     .add(St::WebkitAppearance, val::None)
        //     .add(St::Appearance, val::None)
        //     .and_size(|conf| conf.set_all(px(16.)))
        //     .and_border(|conf| {
        //         conf.solid()
        //             .set_width(px(1.))
        //             .set_color(border)
        //             .set_radius(px(2.))
        //     })
        //     .and_background(|conf| conf.set_color(bg))
        //     .and_text(|conf| conf.set_color(fg))
        //     .merge(&checkbox.user_style().input);

        // let button = Style::default()
        //     .config_if(checkbox.is_toggled(), |conf| {
        //         conf.set_cursor(cursor)
        //             .and_transition(|conf| {
        //                 conf.all(|val| {
        //                     val.set_duration(sec(0.3))
        //                         .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //                 })
        //             })
        //             .and_size(|conf| conf.resize(0.2, 0.55))
        //             .and_border(|conf| {
        //                 conf.and_bottom(|conf| conf.solid().set_width(px(2.)).set_color(fg))
        //                     .and_right(|conf| conf.solid().set_width(px(2.)).set_color(fg))
        //             })
        //             .and_margin(|conf| conf.set_bottom(px(2.)))
        //             .add(St::Transform, "rotate(45deg)")
        //     })
        //     .merge(&checkbox.user_style().button);

        // let label = Style::default()
        //     .config_if(checkbox.is_disabled(), |conf| {
        //         conf.and_text(|conf| conf.set_color(self.disable(false)))
        //     })
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .set_cursor(cursor)
        //     .set_display(val::Flex)
        //     .set_gap(px(4))
        //     .merge(&checkbox.user_style().label);

        // checkbox::Style {
        //     input,
        //     button,
        //     label,
        // }
    }

    fn radio<'a>(&self, radio: RadioLens<'a>) -> Style {
        todo!()
        // let (bg, fg, border) = match (
        //     radio.is_disabled(),
        //     radio.is_focused(),
        //     radio.is_mouse_over(),
        // ) {
        //     (true, _, _) => (
        //         self.gray(Variant::L200),
        //         self.disable(false),
        //         self.gray(Variant::L400),
        //     ),
        //     (false, false, false) => {
        //         if radio.is_toggled() {
        //             (
        //                 self.white(),
        //                 self.brand(Variant::L500),
        //                 self.brand(Variant::L500),
        //             )
        //         } else {
        //             (self.white(), self.white(), self.border(false))
        //         }
        //     }
        //     _ => {
        //         if radio.is_toggled() {
        //             (
        //                 self.white(),
        //                 self.brand(Variant::L500),
        //                 self.brand(Variant::L500),
        //             )
        //         } else {
        //             (self.white(), self.white(), self.brand(Variant::L500))
        //         }
        //     }
        // };

        // let cursor: Cursor = if radio.is_disabled() {
        //     val::NotAllowed.into()
        // } else {
        //     val::Initial.into()
        // };

        // let input = Style::default()
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .set_cursor(cursor)
        //     .set_display(val::Flex)
        //     .and_margin(|conf| conf.zero())
        //     .set_justify_content(val::Center)
        //     .set_align_items(val::Center)
        //     .add(St::WebkitAppearance, val::None)
        //     .and_text(|conf| conf.set_color(fg))
        //     .and_size(|conf| conf.set_all(px(16)))
        //     .and_border(|conf| {
        //         conf.solid()
        //             .set_width(px(1.))
        //             .set_color(border)
        //             .set_radius(0.5)
        //     })
        //     .and_background(|conf| conf.set_color(bg))
        //     .merge(&radio.user_style().input);

        // let button = Style::default()
        //     .config_if(radio.is_toggled(), |conf| {
        //         conf.set_cursor(cursor)
        //             .and_size(|conf| conf.resize(0.6, 0.6))
        //             .and_border(|conf| conf.none().set_radius(0.5))
        //             .and_background(|conf| conf.set_color(fg))
        //     })
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .merge(&radio.user_style().button);

        // let label = Style::default()
        //     .config_if(radio.is_disabled(), |conf| {
        //         conf.and_text(|conf| conf.set_color(self.disable(false)))
        //     })
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .set_cursor(cursor)
        //     .set_display(val::Flex)
        //     .set_align_items(val::Center)
        //     .set_gap(px(4.))
        //     .merge(&radio.user_style().label);

        // radio::Style {
        //     input,
        //     button,
        //     label,
        // }
    }

    fn entry<'a>(&self, entry: EntryLens<'a>) -> Style {
        todo!()
        // let (bg, fg, border) = match (
        //     entry.is_disabled(),
        //     entry.is_focused(),
        //     entry.is_mouse_over(),
        // ) {
        //     (true, _, _) => (
        //         self.gray(Variant::L200),
        //         self.disable(false),
        //         self.gray(Variant::L400),
        //     ),
        //     (false, false, false) => (self.white(), self.primary_text(false), self.border(false)),
        //     _ => (
        //         self.white(),
        //         self.primary_text(false),
        //         self.brand(Variant::L500),
        //     ),
        // };

        // let cursor: Cursor = if entry.is_disabled() {
        //     val::NotAllowed.into()
        // } else {
        //     val::Initial.into()
        // };

        // // em unit
        // let container_width = 14.;
        // let container_height = 1.5;
        // let input_width = container_width - 0.6;
        // let input_height = 1.;
        // let font_size = 1.;
        // // px unit
        // let container_radius = 4;
        // let container_border_width = 1;

        // let container = Style::default()
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .set_display(val::Flex)
        //     .set_align_items(val::Center)
        //     .set_justify_content(val::Center)
        //     // .and_padding(|conf| conf.set_y(px(4.)).set_x(px(11.)))
        //     // .set_gap(px(4.))
        //     .and_background(|conf| conf.set_color(bg))
        //     .and_border(|conf| {
        //         conf.solid()
        //             .set_width(px(container_border_width))
        //             .set_color(border)
        //             .set_radius(px(container_radius))
        //     })
        //     .and_size(|conf| {
        //         conf.set_all_widths(em(container_width))
        //             .set_all_heights(em(container_height))
        //     })
        //     .set_cursor(cursor)
        //     .merge(&entry.user_style().container);

        // let input = Style::default()
        //     .and_transition(|conf| {
        //         conf.all(|val| {
        //             val.set_duration(sec(0.3))
        //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //         })
        //     })
        //     .and_size(|conf| conf.set_width(em(input_width)).set_height(em(input_height)))
        //     .and_text(|conf| conf.set_color(fg))
        //     .and_font(|conf| conf.set_size(em(font_size)))
        //     .and_border(|conf| conf.none())
        //     .and_background(|conf| conf.transparent())
        //     .add(St::WebkitAppearance, val::None)
        //     .set_cursor(cursor)
        //     .merge(&entry.user_style().input);

        // entry::Style { container, input }
    }

    fn spin_entry<'a>(&self, spin_entry: SpinEntryLens<'a>) -> Style {
        todo!()
        // let (bg, fg, border) = match (
        //     spin_entry.is_disabled(),
        //     spin_entry.is_focused(),
        //     spin_entry.is_mouse_over(),
        // ) {
        //     (true, _, _) => (
        //         self.gray(Variant::L200),
        //         self.disable(false),
        //         self.gray(Variant::L400),
        //     ),
        //     (false, false, false) => (self.white(), self.primary_text(false), self.border(false)),
        //     _ => (
        //         self.white(),
        //         self.primary_text(false),
        //         self.brand(Variant::L500),
        //     ),
        // };

        // // em unit used here
        // let width = 4.;
        // let height = 1.5;
        // let btns_container_width = 1.;
        // let btns_container_height = height;
        // let input_width = width - 0.6;
        // let input_height = 1.;
        // let font_size = 1.;
        // // px used here
        // let radius = 4.;
        // let border_width = 1.;
        // // percent units used here
        // let btn_height = 0.5;
        // let btn_width = 1.;
        // let btn_mouse_over_height = btn_height + 0.10;
        // let btn_mouse_over_height_2 = btn_height - 0.10;

        // let cursor: Cursor = if spin_entry.is_disabled() {
        //     val::NotAllowed.into()
        // } else {
        //     val::Initial.into()
        // };

        // let (inc_btn_height, dec_btn_height) = match (
        //     spin_entry.increment_button.is_mouse_over(),
        //     spin_entry.decrement_button.is_mouse_over(),
        // ) {
        //     (true, _) => (btn_mouse_over_height, btn_mouse_over_height_2),
        //     (_, true) => (btn_mouse_over_height_2, btn_mouse_over_height),
        //     (false, false) => (btn_height, btn_height),
        // };

        // Style::default()
        //     .insert("spin-entry", |conf| {
        //         conf.and_position(|conf| conf.relative())
        //             .and_transition(|conf| {
        //                 conf.all(|val| {
        //                     val.set_duration(sec(0.3))
        //                         .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //                 })
        //             })
        //             .set_display(val::Flex)
        //             .set_align_items(val::Center)
        //             .set_justify_content(val::Center)
        //             .and_background(|conf| conf.set_color(bg))
        //             .and_border(|conf| {
        //                 conf.solid()
        //                     .set_width(px(border_width))
        //                     .set_color(border)
        //                     .set_radius(px(radius))
        //             })
        //             .and_size(|conf| conf.set_all_widths(em(width)).set_all_heights(em(height)))
        //             .set_cursor(cursor)
        //             .merge(&spin_entry.user_style().container);
        //     })
        //     .insert("input", |conf| {
        //         conf.and_transition(|conf| {
        //             conf.all(|val| {
        //                 val.set_duration(sec(0.3))
        //                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //             })
        //         })
        //         .and_size(|conf| conf.set_width(em(input_width)).set_height(em(input_height)))
        //         .and_text(|conf| conf.set_color(fg))
        //         .and_font(|conf| conf.set_size(em(font_size)))
        //         .and_border(|conf| conf.none())
        //         .and_background(|conf| conf.transparent())
        //         .add(St::WebkitAppearance, val::None)
        //         .set_cursor(cursor)
        //         .merge(&spin_entry.user_style().input)
        //     })
        //     .insert("increment-button", |conf| {
        //         conf.and_transition(|conf| {
        //             conf.all(|val| {
        //                 val.set_duration(sec(0.3))
        //                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //             })
        //         })
        //         .and_font(|conf| conf.set_size(em(0.6)))
        //         .and_text(|conf| conf.set_color(self.secondary_text(false)))
        //         .and_background(|conf| conf.set_color(self.white()))
        //         .and_border(|conf| conf.none().set_top_right(px(radius)))
        //         .and_size(|conf| conf.set_width(btn_width).set_height(inc_btn_height))
        //         .merge(&spin_entry.user_style().decrement_button.button)
        //     })
        //     // .insert("increment-button-container", |conf| {
        //     //     conf.set_display(val::Flex)
        //     //         .set_align_items(val::Center)
        //     //         .set_align_content(val::Center)
        //     //         .set_justify_content(val::Center)
        //     //         .merge(&spin_entry.user_style().decrement_button.common_container)
        //     // })
        //     .insert("decrement-button", |conf| {
        //         conf.and_transition(|conf| {
        //             conf.all(|val| {
        //                 val.set_duration(sec(0.3))
        //                     .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //             })
        //         })
        //         .and_font(|conf| conf.set_size(em(0.6)))
        //         .and_text(|conf| conf.set_color(self.secondary_text(false)))
        //         .and_background(|conf| conf.set_color(self.white()))
        //         .and_border(|conf| {
        //             conf.none().set_bottom_right(px(radius)).and_top(|conf| {
        //                 conf.solid()
        //                     .set_width(px(border_width))
        //                     .set_color(self.gray(Variant::L300))
        //             })
        //         })
        //         .and_size(|conf| conf.set_width(btn_width).set_height(dec_btn_height))
        //         .merge(&spin_entry.user_style().decrement_button.button)
        //     })
        // // .insert("decrement-button-container", |conf| {
        // //     conf.set_display(val::Flex)
        // //         .set_align_items(val::Center)
        // //         .set_align_content(val::Center)
        // //         .set_justify_content(val::Center)
        // //         .merge(&spin_entry.user_style().decrement_button.common_container)
        // // })

        // // let buttons_container = Style::default()
        // //     .and_position(|conf| conf.absolute().set_right(px(0.)))
        // //     .and_transition(|conf| {
        // //         conf.all(|val| {
        // //             val.set_duration(sec(0.3))
        // //                 .cubic_bezier(0.645, 0.045, 0.355, 1.)
        // //         })
        // //     })
        // //     .set_display(val::Flex)
        // //     .set_flex_direction(val::Column)
        // //     .and_size(|conf| {
        // //         conf.set_width(em(btns_container_width))
        // //             .set_height(em(btns_container_height))
        // //     })
        // //     .and_border(|conf| {
        // //         conf.and_left(|conf| {
        // //             conf.solid()
        // //                 .set_width(px(border_width))
        // //                 .set_color(self.gray(Variant::L300))
        // //         })
        // //     })
        // //     .config(
        // //         |conf| match (spin_entry.is_mouse_over(), spin_entry.is_disabled()) {
        // //             (true, false) => conf.set_opacity(1.).set_visibility(val::Visible),
        // //             _ => conf.set_opacity(0.).set_visibility(val::Hidden),
        // //         },
        // //     )
        // //     .merge(&spin_entry.user_style().buttons_container);
    }

    fn dialog<'a>(&self, dialog: DialogLens<'a>) -> Style {
        todo!()
        // Style::default()
        //     .insert("dialog-background", |conf| {
        //         conf.and_position(|conf| conf.absolute().set_z_index(500))
        //             .and_background(|conf| conf.set_color(Hsla::new(0.0, 0., 0., 0.5)))
        //             .and_size(|conf| conf.full())
        //             .and_transition(|conf| {
        //                 conf.all(|val| {
        //                     val.set_duration(sec(0.3))
        //                         .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //                 })
        //             })
        //             .config(|conf| {
        //                 use dialog::State::*;
        //                 match dialog.state() {
        //                     Closed => conf.set_display(val::None),
        //                     Opening => conf
        //                         .set_display(val::Flex)
        //                         .set_align_content(val::Center)
        //                         .set_align_items(val::Center)
        //                         .set_justify_content(val::Center)
        //                         .set_opacity(0.0),
        //                     Opened => conf
        //                         .set_display(val::Flex)
        //                         .set_align_content(val::Center)
        //                         .set_align_items(val::Center)
        //                         .set_justify_content(val::Center)
        //                         .set_opacity(1.0),
        //                     Closing => conf
        //                         .set_display(val::Flex)
        //                         .set_align_content(val::Center)
        //                         .set_align_items(val::Center)
        //                         .set_justify_content(val::Center)
        //                         .set_opacity(0.0),
        //                 }
        //             })
        //             .merge(&dialog.user_style().background)
        //     })
        //     .insert("dialog", |conf| {
        //         conf.and_background(|conf| conf.set_color(self.white()))
        //             .and_text(|conf| conf.set_color(self.primary_text(true)))
        //             .and_border(|conf| conf.set_radius(px(2)))
        //             .set_display(val::Flex)
        //             .set_flex_direction(val::Column)
        //             .set_align_content(val::Center)
        //             .set_align_items(val::Center)
        //             .set_justify_content(val::Center)
        //             .add(St::BoxShadow, "0 2px 0 rgba(0, 0, 0, 0.015)")
        //             .merge(&dialog.user_style().widget)
        //     })
        //     .insert("content", |conf| {
        //         conf.and_padding(|conf| conf.set_all(em(1.)))
        //             .merge(&dialog.user_style().content)
        //     })
    }

    fn header_bar<'a>(&self, header_bar: HeaderBarLens<'a>) -> Style {
        todo!()
        // // em units
        // let height = 2.5;
        // let button_size = height;
        // let subtitle_font_size = 0.6;
        // let title_container_padding = 1.;

        // let user_style = header_bar.user_style();

        // Style::default()
        //     .insert("title", |conf| {
        //         conf.and_text(|conf| conf.set_color(self.title(false)))
        //             .merge(&user_style.title)
        //     })
        //     .insert("subtitle", |conf| {
        //         conf.and_text(|conf| conf.set_color(self.secondary_text(false)))
        //             .and_font(|conf| conf.set_size(em(subtitle_font_size)))
        //             .merge(&user_style.subtitle)
        //     })
        //     .insert("title-container", |conf| {
        //         conf.set_display(val::Flex)
        //             .set_flex_direction(val::Column)
        //             .set_justify_content(val::Center)
        //             .set_align_items(val::Center)
        //             .set_align_content(val::Center)
        //             .set_flex_wrap(val::Nowrap)
        //             .and_padding(|conf| conf.set_x(em(title_container_padding)))
        //             .merge(&user_style.title_container)
        //     })
        //     .insert("close-button", |conf| {
        //         conf.and_size(|conf| conf.set_all(em(button_size)))
        //             .config(|conf| {
        //                 if let Some(ref btn) = header_bar.close_button {
        //                     if btn.is_mouse_over() {
        //                         conf.and_text(|conf| conf.set_color(self.primary_text(false)))
        //                     } else {
        //                         conf.and_text(|conf| conf.set_color(self.secondary_text(false)))
        //                     }
        //                 } else {
        //                     conf
        //                 }
        //             })
        //             .set_cursor(val::Pointer)
        //             .and_background(|conf| conf.transparent())
        //             .and_border(|conf| conf.none())
        //             .add(St::Outline, val::None)
        //             .add(St::UserSelect, val::None)
        //             .add(St::BoxSizing, val::BorderBox)
        //             .and_font(|conf| conf.set_size(em(1.)))
        //             .and_margin(|conf| conf.zero())
        //             .and_padding(|conf| conf.zero())
        //             .merge(&user_style.close_button.button)
        //     })
        //     .insert("close-button-container", |conf| {
        //         conf.set_display(val::Flex)
        //             .set_justify_content(val::Center)
        //             .set_align_items(val::Center)
        //             .set_align_content(val::Center)
        //             .and_margin(|conf| conf.zero())
        //             .and_padding(|conf| conf.zero())
        //             .merge(&user_style.close_button.common_container)
        //     })
        //     .insert("container", |conf| {
        //         conf.set_display(val::Flex)
        //             .set_justify_content(val::SpaceBetween)
        //             .set_align_items(val::Stretch)
        //             .set_align_content(val::Stretch)
        //             .set_flex_wrap(val::Nowrap)
        //             .and_size(|conf| conf.set_all_heights(em(height)))
        //             .and_border(|conf| {
        //                 conf.and_bottom(|conf| {
        //                     conf.solid()
        //                         .set_width(px(1))
        //                         .set_color(self.gray(Variant::D600))
        //                 })
        //             })
        //             .merge(&user_style.container)
        //     })
    }

    fn label<'a>(&self, _: LabelLens<'a>) -> Style {
        todo!()
        // Style::default()
    }

    fn progress_bar<'a>(&self, progress_bar: ProgressBarLens<'a>) -> Style {
        todo!()
        // // em units
        // let height = 0.7;
        // let radius = height / 2.;

        // // percent units
        // let width = 1.;

        // progress_bar::Style::default()
        //     .and_container(|conf| {
        //         conf.and_border(|conf| conf.set_radius(em(radius)).none())
        //             .set_background(self.gray(Variant::L300))
        //             .and_size(|conf| conf.set_all_heights(em(height)).set_width(width))
        //             .and_transition(|conf| conf.all(|val| val.set_duration(sec(0.3)).ease()))
        //             .and_position(|conf| conf.relative())
        //             .add(St::Overflow, val::Hidden)
        //     })
        //     .and_indicator(|conf| {
        //         conf.and_border(|conf| conf.set_radius(em(radius)))
        //             .config(|conf| {
        //                 let color = progress_bar.get_color().unwrap_or_else(|| {
        //                     match progress_bar.state() {
        //                         progress_bar::State::Normal => self.brand(Variant::L500),
        //                         progress_bar::State::Success => self.suggestion(),
        //                         progress_bar::State::Failure => self.destructive(),
        //                     }
        //                     .into()
        //                 });
        //                 conf.set_background(color)
        //             })
        //             .and_transition(|conf| {
        //                 conf.all(|val| {
        //                     val.set_duration(sec(0.3))
        //                         .cubic_bezier(0.645, 0.045, 0.355, 1.)
        //                 })
        //             })
        //             .and_position(|conf| conf.absolute())
        //             .and_size(|conf| {
        //                 let width = (progress_bar.value() - progress_bar.min()).abs()
        //                     / (progress_bar.max() - progress_bar.min());
        //                 conf.set_width(width as f32).set_all_heights(em(height))
        //             })
        //     })
    }
}
