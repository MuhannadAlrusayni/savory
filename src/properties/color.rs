use crate::properties::unit::*;
use palette::{
    rgb::{Rgb, Rgba},
    Hsl, Hsla,
};

pub fn display_rgb(rgb: &Rgb) -> String {
    let (red, green, blue) = rgb.into_components();
    format!(
        "rgb({}%, {}%, {}%)",
        red * 100.0,
        green * 100.0,
        blue * 100.0
    )
}

pub fn display_rgba(rgb: &Rgba) -> String {
    let (red, green, blue, alpha) = rgb.into_components();
    format!(
        "rgba({}%, {}%, {}%, {})",
        red * 100.0,
        green * 100.0,
        blue * 100.0,
        alpha
    )
}

pub fn display_hsl(hsl: &Hsl) -> String {
    let (hue, saturate, lightness) = hsl.into_components();
    format!(
        "hsl({}, {}%, {}%)",
        hue.to_positive_degrees(),
        saturate * 100.0,
        lightness * 100.0,
    )
}

pub fn display_hsla(hsla: &Hsla) -> String {
    let (hue, saturate, lightness, alpha) = hsla.into_components();
    format!(
        "hsla({}, {}%, {}%, {})",
        hue.to_positive_degrees(),
        saturate * 100.0,
        lightness * 100.0,
        alpha
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Color {
    #[from]
    #[display(fmt = "{}", "display_rgb(_0)")]
    Rgb(Rgb),
    #[from]
    #[display(fmt = "{}", "display_rgba(_0)")]
    Rgba(Rgba),
    #[from]
    #[display(fmt = "{}", "display_hsl(_0)")]
    Hsl(Hsl),
    #[from]
    #[display(fmt = "{}", "display_hsla(_0)")]
    Hsla(Hsla),
    // https://www.w3.org/TR/css-color-3/#transparent
    #[display(fmt = "transparent")]
    Transparent,
    // Extended color keywords
    // https://www.w3.org/TR/css-color-3/#svg-color
    #[display(fmt = "aliceblue")]
    AliceBlue,
    #[display(fmt = "antiquewhite")]
    AntiqueWhite,
    #[display(fmt = "aqua")]
    Aqua,
    #[display(fmt = "aquamarine")]
    AquaMarine,
    #[display(fmt = "azure")]
    Azure,
    #[display(fmt = "beige")]
    Beige,
    #[display(fmt = "bisque")]
    Bisque,
    #[display(fmt = "black")]
    Black,
    #[display(fmt = "blanchedalmond")]
    BlanchedAlmond,
    #[display(fmt = "blue")]
    Blue,
    #[display(fmt = "blueviolet")]
    BlueViolet,
    #[display(fmt = "brown")]
    Brown,
    #[display(fmt = "burlywood")]
    BurlyWood,
    #[display(fmt = "cadetblue")]
    CadetBlue,
    #[display(fmt = "chartreuse")]
    Chartreuse,
    #[display(fmt = "chocolate")]
    Chocolate,
    #[display(fmt = "coral")]
    Coral,
    #[display(fmt = "cornflowerblue")]
    CornFlowerBlue,
    #[display(fmt = "cornsilk")]
    CornSilk,
    #[display(fmt = "crimson")]
    Crimson,
    #[display(fmt = "cyan")]
    Cyan,
    #[display(fmt = "darkblue")]
    DarkBlue,
    #[display(fmt = "darkcyan")]
    DarkCyan,
    #[display(fmt = "darkgoldenrod")]
    DarkGoldenrod,
    #[display(fmt = "darkgray")]
    DarkGray,
    #[display(fmt = "darkgreen")]
    DarkGreen,
    #[display(fmt = "darkgrey")]
    DarkGrey,
    #[display(fmt = "darkkhaki")]
    DarkKhaki,
    #[display(fmt = "darkmagenta")]
    DarkMagenta,
    #[display(fmt = "darkolivegreen")]
    DarkOliveGreen,
    #[display(fmt = "darkorange")]
    DarkOrange,
    #[display(fmt = "darkorchid")]
    DarkOrchid,
    #[display(fmt = "darkred")]
    DarkRed,
    #[display(fmt = "darksalmon")]
    DarkSalmon,
    #[display(fmt = "darkseagreen")]
    DarkSeaGreen,
    #[display(fmt = "darkslateblue")]
    DarkSlateBlue,
    #[display(fmt = "darkslategray")]
    DarkSlateGray,
    #[display(fmt = "darkslategrey")]
    DarkSlateGrey,
    #[display(fmt = "darkturquoise")]
    DarkTurquoise,
    #[display(fmt = "darkviolet")]
    DarkViolet,
    #[display(fmt = "deeppink")]
    DeepPink,
    #[display(fmt = "deepskyblue")]
    DeepSkyBlue,
    #[display(fmt = "dimgray")]
    DimGray,
    #[display(fmt = "dimgrey")]
    DimGrey,
    #[display(fmt = "dodgerblue")]
    DodgerBlue,
    #[display(fmt = "firebrick")]
    FireBrick,
    #[display(fmt = "floralwhite")]
    FloralWhite,
    #[display(fmt = "forestgreen")]
    ForestGreen,
    #[display(fmt = "fuchsia")]
    Fuchsia,
    #[display(fmt = "gainsboro")]
    Gainsboro,
    #[display(fmt = "ghostwhite")]
    GhostWhite,
    #[display(fmt = "gold")]
    Gold,
    #[display(fmt = "goldenrod")]
    GoldEnrod,
    #[display(fmt = "gray")]
    Gray,
    #[display(fmt = "green")]
    Green,
    #[display(fmt = "greenyellow")]
    GreenYellow,
    #[display(fmt = "grey")]
    Grey,
    #[display(fmt = "honeydew")]
    Honeydew,
    #[display(fmt = "hotpink")]
    HotPink,
    #[display(fmt = "indianred")]
    Indianred,
    #[display(fmt = "indigo")]
    Indigo,
    #[display(fmt = "ivory")]
    Ivory,
    #[display(fmt = "khaki")]
    Khaki,
    #[display(fmt = "lavender")]
    Lavender,
    #[display(fmt = "lavenderblush")]
    LavenderBlush,
    #[display(fmt = "lawngreen")]
    LawnGreen,
    #[display(fmt = "lemonchiffon")]
    Lemonchiffon,
    #[display(fmt = "lightblue")]
    LightBlue,
    #[display(fmt = "lightcoral")]
    LightCoral,
    #[display(fmt = "lightcyan")]
    LightCyan,
    #[display(fmt = "lightgoldenrodyellow")]
    LightGoldenrodyellow,
    #[display(fmt = "lightgray")]
    LightGray,
    #[display(fmt = "lightgreen")]
    LightGreen,
    #[display(fmt = "lightgrey")]
    LightGrey,
    #[display(fmt = "lightpink")]
    LightPink,
    #[display(fmt = "lightsalmon")]
    LightSalmon,
    #[display(fmt = "lightseagreen")]
    LightSeaGreen,
    #[display(fmt = "lightskyblue")]
    LightSkyBlue,
    #[display(fmt = "lightslategray")]
    LightSlateGray,
    #[display(fmt = "lightslategrey")]
    LightSlateGrey,
    #[display(fmt = "lightsteelblue")]
    LightSteelBlue,
    #[display(fmt = "lightyellow")]
    LightYellow,
    #[display(fmt = "lime")]
    Lime,
    #[display(fmt = "limegreen")]
    LimeGreen,
    #[display(fmt = "linen")]
    Linen,
    #[display(fmt = "magenta")]
    Magenta,
    #[display(fmt = "maroon")]
    Maroon,
    #[display(fmt = "mediumaquamarine")]
    MediumAquamarine,
    #[display(fmt = "mediumblue")]
    MediumBlue,
    #[display(fmt = "mediumorchid")]
    MediumOrchid,
    #[display(fmt = "mediumpurple")]
    MediumPurple,
    #[display(fmt = "mediumseagreen")]
    MediumSeaGreen,
    #[display(fmt = "mediumslateblue")]
    MediumSlateBlue,
    #[display(fmt = "mediumspringgreen")]
    MediumSpringGreen,
    #[display(fmt = "mediumturquoise")]
    MediumTurquoise,
    #[display(fmt = "mediumvioletred")]
    MediumVioletRed,
    #[display(fmt = "midnightblue")]
    MidnightBlue,
    #[display(fmt = "mintcream")]
    MintCream,
    #[display(fmt = "mistyrose")]
    MistyRose,
    #[display(fmt = "moccasin")]
    Moccasin,
    #[display(fmt = "navajowhite")]
    NavajoWhite,
    #[display(fmt = "navy")]
    Navy,
    #[display(fmt = "oldlace")]
    OldLace,
    #[display(fmt = "olive")]
    Olive,
    #[display(fmt = "olivedrab")]
    OliveDrab,
    #[display(fmt = "orange")]
    Orange,
    #[display(fmt = "orangered")]
    OrangeRed,
    #[display(fmt = "orchid")]
    Orchid,
    #[display(fmt = "palegoldenrod")]
    PaleGoldenrod,
    #[display(fmt = "palegreen")]
    PaleGreen,
    #[display(fmt = "paleturquoise")]
    PaleTurquoise,
    #[display(fmt = "palevioletred")]
    PaleVioletred,
    #[display(fmt = "papayawhip")]
    PapayaWhip,
    #[display(fmt = "peachpuff")]
    PeachPuff,
    #[display(fmt = "peru")]
    Peru,
    #[display(fmt = "pink")]
    Pink,
    #[display(fmt = "plum")]
    Plum,
    #[display(fmt = "powderblue")]
    PowderBlue,
    #[display(fmt = "purple")]
    Purple,
    #[display(fmt = "red")]
    Red,
    #[display(fmt = "rosybrown")]
    RosyBrown,
    #[display(fmt = "royalblue")]
    RoyalBlue,
    #[display(fmt = "saddlebrown")]
    SaddleBrown,
    #[display(fmt = "salmon")]
    Salmon,
    #[display(fmt = "sandybrown")]
    SandyBrown,
    #[display(fmt = "seagreen")]
    SeaGreen,
    #[display(fmt = "seashell")]
    SeaShell,
    #[display(fmt = "sienna")]
    Sienna,
    #[display(fmt = "silver")]
    Silver,
    #[display(fmt = "skyblue")]
    SkyBlue,
    #[display(fmt = "slateblue")]
    SlateBlue,
    #[display(fmt = "slategray")]
    SlateGray,
    #[display(fmt = "slategrey")]
    SlateGrey,
    #[display(fmt = "snow")]
    Snow,
    #[display(fmt = "springgreen")]
    SpringGreen,
    #[display(fmt = "steelblue")]
    SteelBlue,
    #[display(fmt = "tan")]
    Tan,
    #[display(fmt = "teal")]
    Teal,
    #[display(fmt = "thistle")]
    Thistle,
    #[display(fmt = "tomato")]
    Tomato,
    #[display(fmt = "turquoise")]
    Turquoise,
    #[display(fmt = "violet")]
    Violet,
    #[display(fmt = "wheat")]
    Wheat,
    #[display(fmt = "white")]
    White,
    #[display(fmt = "whitesmoke")]
    Whitesmoke,
    #[display(fmt = "yellow")]
    Yellow,
    #[display(fmt = "yellowgreen")]
    YellowGreen,
}
