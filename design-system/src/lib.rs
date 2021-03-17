use palette::{Hsla, LinSrgb, LinSrgba};
use savory::prelude::{DeclarativeConfig, Env};
use savory_elements::prelude::*;
use savory_style::{
    calc::calc,
    text::LineHeight,
    unit::{px, sec, Length},
    values as val, Color, St, Style,
};
use std::rc::Rc;

pub struct SavoryDS {
    default_theme: Theme,
    dark_theme: Theme,
    primary: LinSrgb,
    current_theme: ThemeName,
}

pub enum ThemeName {
    Default,
    Dark,
}

#[derive(Clone)]
pub struct Theme {
    // base colors
    pub red: LinSrgb,
    pub volcano: LinSrgb,
    pub orange: LinSrgb,
    pub gold: LinSrgb,
    pub yellow: LinSrgb,
    pub lime: LinSrgb,
    pub green: LinSrgb,
    pub cyan: LinSrgb,
    pub blue: LinSrgb,
    pub geek_blue: LinSrgb,
    pub purple: LinSrgb,
    pub magenta: LinSrgb,

    // colors
    pub info: LinSrgb,
    pub success: LinSrgb,
    pub processing: LinSrgb,
    pub error: LinSrgb,
    pub highlight: LinSrgb,
    pub warning: LinSrgb,
    pub normal: LinSrgb,
    pub white: LinSrgb,
    pub black: LinSrgb,

    // elements colors
    pub bg: LinSrgb,
    pub body_bg: LinSrgb,
    pub element_bg: LinSrgb,
    pub border: LinSrgb,
    pub border_split: LinSrgb,
    pub text: LinSrgba,
    pub text_secondary: LinSrgba,
    pub disabled_bg: LinSrgb,
    pub disabled_text: LinSrgba,

    // base sizes & lengths
    pub font_size: Length,
    pub line_height: LineHeight,
    pub border_radius: Length,
    pub border_width: Length,
    pub height: Length,
    // minmum_padding: Length,
}

impl SavoryDS {
    pub fn push_to_environment(self, env: Env) {
        let ds = Rc::new(self);
        env.overwrite_designer::<Text>(ds.clone())
            .overwrite_designer::<Button>(ds.clone())
            .overwrite_designer::<Switch>(ds.clone())
            .overwrite_designer::<Radio>(ds.clone())
            .overwrite_designer::<TextInput>(ds.clone())
            .overwrite_designer::<ProgressBar>(ds.clone());
    }

    pub fn current_theme(&self) -> &Theme {
        match self.current_theme {
            ThemeName::Default => &self.default_theme,
            ThemeName::Dark => &self.dark_theme,
        }
    }

    pub fn generate(&self, color: impl Into<LinSrgb>) -> Vec<LinSrgb> {
        let color = color.into();
        match self.current_theme {
            ThemeName::Default => colors::generate(color, colors::Opts::default()),
            ThemeName::Dark => {
                let opts = colors::Opts {
                    dark_theme: true,
                    background: Some(self.dark_theme.body_bg),
                };
                colors::generate(color, opts)
            }
        }
    }

    pub fn primary(&self) -> LinSrgb {
        self.primary
    }
}

impl Default for SavoryDS {
    fn default() -> Self {
        // base colors
        let red = LinSrgb::new(0.96078, 0.13333, 0.17647); // #F5222D
        let volcano = LinSrgb::new(0.98039, 0.32941, 0.10980); // #FA541C
        let orange = LinSrgb::new(0.98039, 0.54902, 0.08627); // #FA8C16
        let gold = LinSrgb::new(0.98039, 0.67843, 0.07843); // #FAAD14
        let yellow = LinSrgb::new(0.98039, 0.85882, 0.07843); // #FADB14
        let lime = LinSrgb::new(0.62745, 0.85098, 0.06667); // #A0D911
        let green = LinSrgb::new(0.32157, 0.76863, 0.10196); // #52C41A
        let cyan = LinSrgb::new(0.07451, 0.76078, 0.76078); // #13C2C2
        let blue = LinSrgb::new(0.09412, 0.56471, 1.0); // #1890FF
        let geek_blue = LinSrgb::new(0.18431, 0.32941, 0.92157); // #2F54EB
        let purple = LinSrgb::new(0.44706, 0.18039, 0.81961); // #722ED1
        let magenta = LinSrgb::new(0.92157, 0.18431, 0.58824); // #EB2F96

        // colors
        let primary = LinSrgb::new(0.09412, 0.56471, 1.0); // #1890FF

        let white = LinSrgb::new(1.0, 1.0, 1.0);
        let black = LinSrgb::new(0.0, 0.0, 0.0);
        let bg = LinSrgb::new(0.96078, 0.96078, 0.96078);

        let default_theme = Theme {
            // base colors
            red,
            volcano,
            orange,
            gold,
            yellow,
            lime,
            green,
            cyan,
            blue,
            geek_blue,
            purple,
            magenta,

            // colors
            info: primary,
            success: green,
            processing: blue,
            error: colors::generate(red, colors::Opts::default())[4],
            highlight: colors::generate(red, colors::Opts::default())[4],
            warning: gold,
            normal: LinSrgb::new(0.85098, 0.85098, 0.85098),
            white,
            black,

            // base colors
            bg: bg.clone(),
            body_bg: white,
            element_bg: white,
            // item_selected_bg: colors::generate(primary, colors::Opts::default())[0],
            // item_hover_bg: LinSrgb::new(0.96078, 0.96078, 0.96078),
            border: LinSrgb::new(0.85098, 0.85098, 0.85098),
            border_split: LinSrgb::new(0.94118, 0.94118, 0.94118),
            text: LinSrgba::new(black.red, black.green, black.blue, 0.85),
            text_secondary: LinSrgba::new(black.red, black.green, black.blue, 0.45),
            disabled_bg: bg,
            disabled_text: LinSrgba::new(black.red, black.green, black.blue, 0.25),

            // base sizes & lengths
            font_size: px(14).into(),
            line_height: 1.5715.into(),
            border_radius: px(2).into(),
            border_width: px(1).into(),
            height: px(32).into(),
            // minmum_padding: Length,
        };

        let dark_theme = Theme {
            ..default_theme.clone()
        };

        SavoryDS {
            default_theme,
            dark_theme,
            primary,
            current_theme: ThemeName::Default,
        }
    }
}

impl Design<Text> for SavoryDS {
    fn design(&self, lens: text::TextLens, _: &Env) -> text::StyleMap {
        let theme = self.current_theme();
        let text::TextLens {
            color,
            letter_spacing,
            word_spacing,
            lines_spacing,
            align,
            justify_by,
            indent,
            wrap,
            shadow,
            size,
            style,
            small_caps,
            weight,
            disabled,
        } = lens;
        Style::default()
            .and_text(|t| {
                t.color(color.unwrap_or(theme.text.into()))
                    .try_letter_spacing(letter_spacing)
                    .try_word_spacing(word_spacing)
                    .line_height(lines_spacing.unwrap_or(theme.line_height.clone()))
                    .try_align(align)
                    .try_justify(justify_by)
                    .try_indent(indent)
                    .config_if(wrap, |c| c.word_wrap(val::BreakWord))
                    .try_shadow(shadow)
            })
            .and_font(|f| {
                f.size(size.unwrap_or(theme.font_size.clone().into()))
                    .try_style(style)
                    .config_if(small_caps, |f| f.variant(val::SmallCaps))
                    .try_weight(weight)
            })
            .config_if(disabled, |c| {
                c.cursor(val::NotAllowed)
                    .text(theme.disabled_text)
                    .push(St::UserSelect, val::None)
            })
    }
}

impl Design<Button> for SavoryDS {
    fn design(&self, lens: button::ButtonLens, _: &Env) -> button::StyleMap {
        use button::{ActionType, Kind};
        let theme = self.current_theme();
        let kind = lens.kind;
        let action = lens.action_type;

        Style::default()
            .and_font(|c| c.weight_400().size(theme.font_size.clone()))
            .and_border(|c| {
                c.radius(theme.border_radius.clone())
                    .solid()
                    .width(theme.border_width.clone())
            })
            .and_box_shadow(|c| c.y(px(2.0)).color(LinSrgba::new(0., 0., 0., 0.015)))
            .and_text(|c| {
                c.line_height(theme.line_height.clone())
                    .white_space(val::Nowrap)
                    .align(val::Center)
            })
            .and_padding(|c| c.y(px(4)).x(px(15)))
            .and_size(|c| c.height(theme.height.clone()))
            .push(St::WebkitAppearance, "button")
            .display(val::InlineBlock)
            .cursor(val::Pointer)
            .and_transition(|c| c.duration(sec(0.3)).cubic_bezier(0.645, 0.045, 0.355, 1.0))
            .push(St::UserSelect, val::None)
            .push(St::TouchAction, val::Manipulation)
            .push(St::Outline, 0)
            .config_if_else(
                // Disabled class ========================================
                lens.disabled,
                |c| {
                    match kind {
                        Kind::Default | Kind::Dashed => c
                            .border(theme.border)
                            .background(theme.disabled_bg)
                            .text(theme.disabled_text),
                        _ => c
                            .border(Color::Transparent)
                            .background(Color::Transparent)
                            .text(theme.disabled_text),
                    }
                    .cursor(val::NotAllowed)
                },
                // other classes
                |c| match kind {
                    Kind::Default => match action {
                        ActionType::Default => c
                            .background(theme.element_bg)
                            .text(theme.text)
                            .border(theme.border)
                            .config_if(lens.ghost, |c| {
                                c.background(Color::Transparent)
                                    .text(theme.element_bg)
                                    .border(theme.element_bg)
                            })
                            .config_if(lens.focused || lens.mouse_over, |c| {
                                let color = lens.color.unwrap_or(self.primary.into());
                                c.text(color).border(color)
                            }),
                        ActionType::Suggested | ActionType::Destructive => {
                            let color = lens.color.unwrap_or_else(|| match action {
                                ActionType::Default => unreachable!("cannot get executed"),
                                ActionType::Suggested => self.primary.into(),
                                ActionType::Destructive => theme.red.into(),
                            });
                            c.background(color)
                                .text(lens.text_color.unwrap_or(theme.white.into()))
                                .border(color)
                                .config_if(lens.focused || lens.mouse_over, |c| {
                                    let color = self.generate(color)[4];
                                    c.background(color).border(color)
                                })
                        }
                    },
                    Kind::Dashed => c
                        .background(theme.element_bg)
                        .text(theme.text)
                        .and_border(|b| b.color(theme.border).dashed())
                        .config_if(lens.ghost, |c| {
                            c.background(Color::Transparent)
                                .text(theme.element_bg)
                                .border(theme.element_bg)
                        })
                        .config_if(lens.focused || lens.mouse_over, |c| {
                            let color = lens.color.unwrap_or(self.primary.into());
                            c.text(color).border(color)
                        }),
                    Kind::TextButton => c
                        .background(Color::Transparent)
                        .border(Color::Transparent)
                        .text(lens.text_color.unwrap_or(theme.text.into()))
                        .box_shadow(val::None)
                        .config_if(lens.focused || lens.mouse_over, |c| {
                            c.background(Hsla::new(0.0, 0.0, 0.0, 0.018))
                        }),
                    Kind::LinkButton => {
                        let color = lens.text_color.unwrap_or(self.primary.into());
                        c.background(Color::Transparent)
                            .border(Color::Transparent)
                            .box_shadow(val::None)
                            .text(color)
                            .config_if(lens.focused || lens.mouse_over, |c| {
                                c.text(self.generate(color)[4])
                            })
                    }
                },
            )
            // for ghost buttons only
            .config_if(lens.ghost, |c| c.background(Color::Transparent))
    }
}

impl Design<Switch> for SavoryDS {
    fn design(&self, lens: switch::SwitchLens, _: &Env) -> switch::StyleMap {
        let theme = self.current_theme();
        if lens.checkbox_like {
            let size = 16.0;
            let switch = Style::default()
                .push(St::Appearance, val::None)
                .position(val::Relative)
                .display(val::InlineBlock)
                .push(St::BoxSizing, val::BorderBox)
                .push(St::VerticalAlign, val::Middle)
                .push(St::UserSelect, val::None)
                .push(St::TouchAction, val::Manipulation)
                .cursor(val::Pointer)
                .margin(px(0))
                .padding(px(0))
                .size(px(size))
                .and_text(|t| t.line_height(1.0))
                .and_border(|b| {
                    b.radius(theme.border_radius.clone())
                        .solid()
                        .color(theme.border)
                        .width(theme.border_width.clone())
                })
                .background(theme.element_bg)
                .and_transition(|t| t.duration(sec(0.3)))
                .config_if(lens.toggled, |c| {
                    let color = lens.color.unwrap_or(self.primary.into());
                    c.background(color).border(color)
                })
                .config_if(lens.disabled, |c| {
                    c.background(theme.disabled_bg)
                        .border(theme.border)
                        .cursor(val::NotAllowed)
                });
            let size = 10.0;
            let check_sign = Style::default()
                .push(St::BoxSizing, val::BorderBox)
                .and_border(|b| {
                    b.none()
                        .and_left(|t| t.width(px(2)).solid().color(theme.element_bg))
                        .and_bottom(|t| t.width(px(2)).solid().color(theme.element_bg))
                })
                .cursor(val::Pointer)
                .and_size(|s| s.width(px(size)).height(px(size / 2.0)))
                .push(St::Transition, "all .2s cubic-bezier(.12,.4,.29,1.46) .1s")
                .push(St::Transform, "rotate(-45deg) translate(25%, 25%)")
                .config_if(lens.disabled, |c| {
                    c.border(theme.disabled_text).cursor(val::NotAllowed)
                });
            let text = Style::default()
                .display(val::InlineFlex)
                .align_items(val::Center)
                .push(St::UserSelect, val::None)
                .push(St::VerticalAlign, val::Middle)
                .gap(px(8))
                .and_text(|t| t.color(theme.text).line_height(theme.line_height.clone()))
                .and_font(|f| f.size(theme.font_size.clone()))
                .config_if(lens.disabled, |c| {
                    c.cursor(val::NotAllowed).text(theme.disabled_text)
                });

            switch::StyleMap {
                switch,
                check_sign,
                text,
            }
        } else {
            let height = 22.0;
            let switch = Style::default()
                .push(St::Appearance, val::None)
                .position(val::Relative)
                .display(val::InlineBlock)
                .push(St::BoxSizing, val::BorderBox)
                .push(St::VerticalAlign, val::Middle)
                .push(St::UserSelect, val::None)
                .push(St::TouchAction, val::Manipulation)
                .cursor(val::Pointer)
                .margin(px(0))
                .padding(px(0))
                .and_size(|s| s.height(px(height)).min_width(px(44)))
                .and_text(|t| t.line_height(px(height)))
                .and_border(|b| b.radius(px(100)).none())
                .background(theme.disabled_text)
                .and_transition(|t| t.duration(sec(0.2)))
                .config_if(lens.toggled, |c| {
                    c.background(lens.color.unwrap_or(self.primary.into()))
                })
                .config_if(lens.disabled, |c| c.opacity(0.4).cursor(val::NotAllowed));
            let spaceing = 2.0;
            let size = height - (spaceing * 2.0);
            let check_sign = Style::default()
                .and_position(|p| {
                    p.absolute().top(px(spaceing)).config_if_else(
                        lens.toggled,
                        |c| c.left(calc(1.0, |c| c.sub(px(size + spaceing)))),
                        |c| c.left(px(spaceing)),
                    )
                })
                .push(St::BoxSizing, val::BorderBox)
                .push(St::BoxShadow, "0 2px 4px 0 rgba(0,35,11,.2)")
                .and_border(|b| b.none().radius(px(size)))
                .cursor(val::Pointer)
                .size(px(size))
                .background(theme.element_bg)
                .and_transition(|t| t.duration(sec(0.2)).ease_in_out())
                .config_if(lens.disabled, |c| c.cursor(val::NotAllowed));
            let text = Style::default()
                .display(val::InlineFlex)
                .align_items(val::Center)
                .push(St::UserSelect, val::None)
                .push(St::VerticalAlign, val::Middle)
                .gap(px(8))
                .and_text(|t| t.color(theme.text).line_height(theme.line_height.clone()))
                .and_font(|f| f.size(theme.font_size.clone()))
                .config_if(lens.disabled, |c| {
                    c.cursor(val::NotAllowed).text(theme.disabled_text)
                });

            switch::StyleMap {
                switch,
                check_sign,
                text,
            }
        }
    }
}

impl Design<Radio> for SavoryDS {
    fn design(&self, lens: radio::RadioLens, _: &Env) -> radio::StyleMap {
        let theme = self.current_theme();
        let size = 16.0;
        let radio = Style::default()
            .push(St::Appearance, val::None)
            .position(val::Relative)
            .display(val::InlineBlock)
            .push(St::BoxSizing, val::BorderBox)
            .push(St::VerticalAlign, val::Middle)
            .push(St::UserSelect, val::None)
            .push(St::TouchAction, val::Manipulation)
            .cursor(val::Pointer)
            .margin(px(0))
            .padding(px(0))
            .size(px(size))
            .and_text(|t| t.line_height(1.0))
            .and_border(|b| {
                b.radius(px(size))
                    .solid()
                    .color(theme.border)
                    .width(theme.border_width.clone())
            })
            .background(theme.element_bg)
            .and_transition(|t| t.duration(sec(0.3)))
            .config_if(lens.focused, |c| {
                c.border(lens.color.unwrap_or(self.primary.into()))
            })
            .config_if(lens.toggled, |c| {
                c.border(lens.color.unwrap_or(self.primary.into()))
            })
            .config_if(lens.disabled, |c| {
                c.background(theme.disabled_bg)
                    .border(theme.border)
                    .cursor(val::NotAllowed)
            });
        let check_sign = Style::default()
            .push(St::BoxSizing, val::BorderBox)
            .and_border(|b| b.none().radius(px(size)))
            .cursor(val::Pointer)
            .margin(val::Auto)
            .background(Color::Transparent)
            .size(px(size - 8.0))
            .and_transition(|t| t.duration(sec(0.3)))
            .config_if(lens.toggled, |c| {
                c.background(lens.color.unwrap_or(self.primary.into()))
            })
            .config_if(lens.disabled, |c| {
                c.border(theme.disabled_text)
                    .background(theme.disabled_text)
                    .cursor(val::NotAllowed)
            });
        let text = Style::default()
            .display(val::InlineFlex)
            .align_items(val::Center)
            .push(St::UserSelect, val::None)
            .push(St::VerticalAlign, val::Middle)
            .gap(px(8))
            .and_text(|t| t.color(theme.text).line_height(theme.line_height.clone()))
            .and_font(|f| f.size(theme.font_size.clone()))
            .config_if(lens.disabled, |c| {
                c.cursor(val::NotAllowed).text(theme.disabled_text)
            });

        radio::StyleMap {
            radio,
            check_sign,
            text,
        }
    }
}

impl Design<TextInput> for SavoryDS {
    fn design(&self, lens: text_input::TextInputLens, _: &Env) -> text_input::StyleMap {
        let theme = self.current_theme();
        Style::default()
            .push(St::Appearance, val::None)
            .position(val::Relative)
            .display(val::InlineBlock)
            .push(St::BoxSizing, val::BorderBox)
            // .push(St::VerticalAlign, val::Middle)
            .push(St::UserSelect, val::None)
            .push(St::TouchAction, val::Manipulation)
            .cursor(val::Pointer)
            .and_size(|s| s.width(1.0).height(theme.height.clone()))
            .and_padding(|p| p.x(px(11)).y(px(4)))
            .and_text(|t| t.line_height(theme.height.clone()))
            .and_font(|f| f.size(theme.font_size.clone()))
            .and_border(|b| {
                b.none()
                    .solid()
                    .width(theme.border_width.clone())
                    .color(theme.border)
                    .radius(theme.border_radius.clone())
            })
            .background(theme.element_bg)
            .config_if(lens.focused || lens.mouse_over, |c| {
                c.border(lens.color.unwrap_or(self.primary.into()))
            })
            .config_if(lens.disabled, |c| {
                c.background(theme.disabled_bg)
                    .text(theme.disabled_text)
                    .border(theme.border)
                    .cursor(val::NotAllowed)
            })
    }
}

impl Design<ProgressBar> for SavoryDS {
    fn design(&self, lens: progress_bar::ProgressBarLens, _: &Env) -> progress_bar::StyleMap {
        let theme = self.current_theme();
        let height = 8.0;
        let progress_bar = Style::default()
            .position(val::Relative)
            .display(val::InlineBlock)
            .push(St::BoxSizing, val::BorderBox)
            .background(theme.bg)
            .and_border(|b| b.radius(px(100)))
            .and_size(|s| s.width(1.0).min_width(px(50)).height(px(height)));

        let indicator = Style::default()
            .push(St::BoxSizing, val::BorderBox)
            .background(theme.processing)
            .and_border(|b| b.radius(px(100)))
            .and_size(|s| s.width(lens.value / lens.max).height(px(height)))
            .config_if(lens.disabled, |c| c.background(theme.disabled_bg));

        progress_bar::StyleMap {
            progress_bar,
            indicator,
        }
    }
}

pub mod colors {
    use palette::{Hsv, LinSrgb, Mix};

    const HUE_STEP: f32 = 2.;
    const SATURATION_STEP: f32 = 0.16;
    const SATURATION_STEP2: f32 = 0.05;
    const BRIGHTNESS_STEP1: f32 = 0.05;
    const BRIGHTNESS_STEP2: f32 = 0.15;
    const LIGHT_COLOR_COUNT: u8 = 5;
    const DARK_COLOR_COUNT: u8 = 4;
    // (index, opacity)
    const DARK_COLOR_MAP: [(u8, f32); 10] = [
        (7, 0.15),
        (6, 0.25),
        (5, 0.3),
        (5, 0.45),
        (5, 0.65),
        (5, 0.85),
        (4, 0.9),
        (3, 0.95),
        (2, 0.97),
        (1, 0.98),
    ];

    fn get_hue(color: Hsv, i: u8, light: bool) -> f32 {
        let hue: f32 = color.hue.to_positive_degrees();
        let mut hue: f32 = match (light, hue.round()) {
            (true, hue) if hue >= 60.0 && hue <= 240.0 => hue.round() - (HUE_STEP * i as f32),
            (false, hue) if hue >= 60.0 && hue <= 240.0 => hue.round() + (HUE_STEP * i as f32),
            (true, _) => hue.round() + (HUE_STEP * i as f32),
            (false, _) => hue.round() - (HUE_STEP * i as f32),
        };
        if hue < 0.0 {
            hue += 360.0
        } else if hue >= 360.0 {
            hue -= 360.0
        }
        hue
    }

    fn get_saturation(color: Hsv, i: u8, light: bool) -> f32 {
        // grey color don't change saturation
        if color.hue == 0.0 && color.saturation == 0.0 {
            return color.saturation;
        }
        let mut saturation = if light {
            color.saturation - (SATURATION_STEP * i as f32)
        } else if i == DARK_COLOR_COUNT {
            color.saturation + SATURATION_STEP
        } else {
            color.saturation + (SATURATION_STEP2 * i as f32)
        };

        if saturation > 1.0 {
            saturation = 1.0
        }

        if light && i == LIGHT_COLOR_COUNT && saturation > 0.1 {
            saturation = 0.1;
        }

        if saturation < 0.06 {
            saturation = 0.06;
        }

        (saturation * 100.0).round() / 100.0
    }

    fn get_value(color: Hsv, i: u8, light: bool) -> f32 {
        let i = i as f32;
        let value = if light {
            color.value + (BRIGHTNESS_STEP1 * i)
        } else {
            color.value - (BRIGHTNESS_STEP2 * i)
        };
        if value > 1.0 {
            1.0
        } else {
            (value * 100.0).round() / 100.0
        }
    }

    #[derive(Default)]
    pub struct Opts {
        pub dark_theme: bool,
        pub background: Option<LinSrgb>,
    }

    pub fn generate(color: impl Into<LinSrgb>, opts: Opts) -> Vec<LinSrgb> {
        let hsv: Hsv = color.into().into();

        let light_colors = (1..=LIGHT_COLOR_COUNT)
            .into_iter()
            .rev()
            .map(|i| {
                Hsv::new(
                    get_hue(hsv, i, true),
                    get_saturation(hsv, i, true),
                    get_value(hsv, i, true),
                )
            })
            .collect::<Vec<_>>();
        let dark_colors = (1..=DARK_COLOR_COUNT)
            .into_iter()
            .map(|i| {
                Hsv::new(
                    get_hue(hsv, i, false),
                    get_saturation(hsv, i, false),
                    get_value(hsv, i, false),
                )
            })
            .collect::<Vec<_>>();

        let patterns = [light_colors, vec![hsv], dark_colors]
            .concat()
            .into_iter()
            .map(|h| h.into())
            .collect::<Vec<LinSrgb>>();

        if opts.dark_theme {
            DARK_COLOR_MAP
                .iter()
                .map(|(index, opacity)| {
                    let bg = opts
                        .background
                        // #141414
                        .unwrap_or_else(|| LinSrgb::new(0.07843, 0.07843, 0.07843));
                    bg.mix(&patterns[*index as usize], *opacity).into()
                })
                .collect::<Vec<_>>()
        } else {
            patterns
        }
    }
}
