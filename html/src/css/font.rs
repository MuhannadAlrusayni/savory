use crate::css::{unit::*, values as val, St, StyleMap, ToStyleMap};
use derive_rich::Rich;
use std::borrow::Cow;

/// ```
/// use savory::css::{values as val, Style, Color, unit::em};
/// use palette::rgb::Rgb;
///
/// let mut style = Style::default();
/// style
///     .and_font(|conf| {
///         // set the font size to xx-large
///         conf.xx_large()
///             // we can set the font size with unit functions too
///             .set_size(em(1.5))
///             // set font variant to smal-caps
///             .small_caps()
///             // set font to be bold
///             .bold()
///             // we can pick specific weight (e.g. 200)
///             .weight_200()
///     });
/// ```
#[derive(Rich, Clone, Debug, PartialEq, Default)]
pub struct Font {
    #[rich(write)]
    pub family: Option<Family>,
    #[rich(write, value_fns = {
        medium = val::Medium,
        xx_small = val::XXSmall,
        x_small = val::XSmall,
        small = val::Small,
        large = val::Large,
        x_large = val::XLarge,
        xx_large = val::XXLarge,
        smaller = val::Smaller,
        larger = val::Larger,
    })]
    pub size: Option<Size>,
    #[rich(write, value_fns = {
        normal_style = val::Normal,
        italic = val::Italic,
        oblique = val::Oblique,
    })]
    pub style: Option<Style>,
    #[rich(write, value_fns = {
        normal_variant = val::Normal,
        small_caps = val::SmallCaps,
    })]
    pub variant: Option<Variant>,
    #[rich(write, value_fns = {
        normal_weight = val::Normal,
        bold = val::Bold,
        bolder = val::Bolder,
        lighter = val::Lighter,
        weight_100 = Weight::L100,
        weight_200 = Weight::L200,
        weight_300 = Weight::L300,
        weight_400 = Weight::L400,
        weight_500 = Weight::L500,
        weight_600 = Weight::L600,
        weight_700 = Weight::L700,
        weight_800 = Weight::L800,
        weight_900 = Weight::L900,
    })]
    pub weight: Option<Weight>,
}

impl_add_and_add_assign!(Font { weight variant style size family { clone } });

impl ToStyleMap for Font {
    fn style_map(&self) -> StyleMap {
        StyleMap::default()
            .try_add(St::FontFamily, self.family.clone())
            .try_add(St::FontSize, self.size)
            .try_add(St::FontStyle, self.style)
            .try_add(St::FontVariant, self.variant)
            .try_add(St::FontWeight, self.weight)
    }
}

#[derive(Clone, Debug, PartialEq, Display, From)]
pub enum Family {
    #[from]
    #[display(fmt = "{}", "_0.join(\" \")")]
    Family(Vec<Cow<'static, str>>),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

impl From<Cow<'static, str>> for Family {
    fn from(source: Cow<'static, str>) -> Self {
        Family::Family(vec![source])
    }
}

impl From<String> for Family {
    fn from(source: String) -> Self {
        Family::Family(vec![source.into()])
    }
}

impl From<&'static str> for Family {
    fn from(source: &'static str) -> Self {
        Family::Family(vec![source.into()])
    }
}

impl From<Vec<String>> for Family {
    fn from(source: Vec<String>) -> Self {
        Family::Family(source.into_iter().map(Into::into).collect())
    }
}

impl From<Vec<&'static str>> for Family {
    fn from(source: Vec<&'static str>) -> Self {
        Family::Family(source.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Size {
    #[from]
    Medium(val::Medium),
    #[from]
    XXSmall(val::XXSmall),
    #[from]
    XSmall(val::XSmall),
    #[from]
    Small(val::Small),
    #[from]
    Large(val::Large),
    #[from]
    XLarge(val::XLarge),
    #[from]
    XXLarge(val::XXLarge),
    #[from]
    Smaller(val::Smaller),
    #[from]
    Larger(val::Larger),
    #[from]
    Length(Length),
    #[from]
    Percent(Percent),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Style {
    #[from]
    Normal(val::Normal),
    #[from]
    Italic(val::Italic),
    #[from]
    Oblique(val::Oblique),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Variant {
    #[from]
    Normal(val::Normal),
    #[from]
    SmallCaps(val::SmallCaps),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Weight {
    #[from]
    Normal(val::Normal),
    #[from]
    Bold(val::Bold),
    #[from]
    Bolder(val::Bolder),
    #[from]
    Lighter(val::Lighter),
    #[display(fmt = "100")]
    L100,
    #[display(fmt = "200")]
    L200,
    #[display(fmt = "300")]
    L300,
    #[display(fmt = "400")]
    L400,
    #[display(fmt = "500")]
    L500,
    #[display(fmt = "600")]
    L600,
    #[display(fmt = "700")]
    L700,
    #[display(fmt = "800")]
    L800,
    #[display(fmt = "900")]
    L900,
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}
