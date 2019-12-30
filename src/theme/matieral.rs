// #[derive(Clone, Debug, Copy)]
// pub struct AnaamTheme {
//     is_dark: bool,
// }

// impl AnaamTheme {
//     pub fn new() -> Self {
//         Self { is_dark: false }
//     }

//     fn color_variant(&self, base: Hsl, variant: Variant) -> Hsl {
//         use Variant::*;

//         let lightness = base.lightness;
//         let saturation = base.saturation;
//         match variant {
//             L50 => base.lighten(lightness * 0.9),
//             L100 => base.lighten(lightness * 0.7),
//             L200 => base.lighten(lightness * 0.4),
//             L300 => base.lighten(lightness * 0.3),
//             L400 => base.lighten(lightness * 0.13),
//             L500 => base,
//             D600 => base.darken(lightness * 0.03),
//             D700 => base.darken(lightness * 0.08),
//             D800 => base.darken(lightness * 0.15),
//             D900 => base.darken(lightness * 0.20),
//             A100 => base.lighten(lightness * 0.95).desaturate(saturation * 0.4),
//             A200 => base.lighten(lightness * 0.88).desaturate(saturation * 0.4),
//             A400 => base.lighten(lightness * 0.68).desaturate(saturation * 0.09),
//             A700 => base.lighten(lightness * 0.6).desaturate(saturation * 0.05),
//         }
//         .clamp()
//     }
// }

// // https://github.com/mbitson/mcg/blob/master/scripts/controllers/ColorGeneratorCtrl.js#L237
// impl Theme for AnaamTheme {
//     fn primary(&self, variant: Variant) -> Hsla {
//         let color = Hsl::new(339.22, 1.0, 0.5);
//         self.color_variant(color, variant).into()
//     }
//     fn secondary(&self, variant: Variant) -> Hsla {
//         let color = Hsl::new(198.67, 0.9757, 0.4843);
//         self.color_variant(color, variant).into()
//     }

//     fn on_primary(&self, variant: Variant) -> Hsla {
//         self.on_color(self.primary(variant))
//     }
//     fn on_secondary(&self, variant: Variant) -> Hsla {
//         self.on_color(self.secondary(variant))
//     }

//     fn background(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 1.0, 1.0)
//     }
//     fn surface(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 0.95, 1.0)
//     }

//     fn on_background(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 0.0, 1.0)
//     }
//     fn on_surface(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 0.0, 1.0)
//     }

//     fn error(&self) -> Hsla {
//         Hsla::new(2.6, 0.831, 0.559, 1.0)
//     }
//     fn warning(&self) -> Hsla {
//         Hsla::new(37.9, 0.831, 0.559, 1.0)
//     }
//     fn success(&self) -> Hsla {
//         Hsla::new(77.6, 0.831, 0.559, 1.0)
//     }
//     fn info(&self) -> Hsla {
//         Hsla::new(212.4, 0.831, 0.559, 1.0)
//     }

//     fn on_error(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 1.0, 1.0)
//     }
//     fn on_warning(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 1.0, 1.0)
//     }
//     fn on_success(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 0.0, 1.0)
//     }
//     fn on_info(&self) -> Hsla {
//         Hsla::new(0.0, 0.0, 1.0, 1.0)
//     }

//     fn on_color(&self, color: impl Into<Hsla>) -> Hsla {
//         let color: GammaLuma = color.into().into();
//         let black: GammaLuma = Hsla::new(0.0, 0.0, 1.0, 1.0).into();
//         let white: GammaLuma = Hsla::new(0.0, 0.0, 0.0, 1.0).into();

//         let with_white = contrast_ratio(color.luma, black.luma);
//         let with_black = contrast_ratio(color.luma, white.luma);
//         if with_white > with_black {
//             Hsla::new(0.0, 0.0, 1.0, 1.0)
//         } else {
//             Hsla::new(0.0, 0.0, 0.0, 1.0)
//         }
//     }
// }
