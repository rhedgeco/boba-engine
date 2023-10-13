pub struct Colorf64 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Colorf64 {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn hex(str: &str) -> Option<Self> {
        Some(Color::hex(str)?.into())
    }
}

impl From<Color> for Colorf64 {
    fn from(color: Color) -> Self {
        Self {
            r: color.r as f64 / 255.,
            g: color.g as f64 / 255.,
            b: color.b as f64 / 255.,
            a: color.a as f64 / 255.,
        }
    }
}

impl Into<Color> for Colorf64 {
    fn into(self) -> Color {
        Color {
            r: (self.r * 255.).clamp(0., 255.).round() as u8,
            g: (self.g * 255.).clamp(0., 255.).round() as u8,
            b: (self.b * 255.).clamp(0., 255.).round() as u8,
            a: (self.a * 255.).clamp(0., 255.).round() as u8,
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn hex_panic(str: &str) -> Self {
        match Self::hex(str) {
            Some(color) => color,
            None => panic!("Invalid hex string"),
        }
    }

    pub const fn hex(str: &str) -> Option<Self> {
        const fn hex_to_byte(hex: u8) -> Option<u8> {
            match hex {
                v @ 48..=57 => Some(v - 48),
                v @ 65..=70 => Some(v - 55),
                _ => None,
            }
        }

        let bytes = str.as_bytes();
        let (r, g, b, a) = match bytes.len() {
            3 => {
                let (Some(r), Some(g), Some(b)) = (
                    hex_to_byte(bytes[0]),
                    hex_to_byte(bytes[1]),
                    hex_to_byte(bytes[2]),
                ) else {
                    return None;
                };

                (r, g, b, 255)
            }
            4 => {
                let (Some(r), Some(g), Some(b), Some(a)) = (
                    hex_to_byte(bytes[0]),
                    hex_to_byte(bytes[1]),
                    hex_to_byte(bytes[2]),
                    hex_to_byte(bytes[3]),
                ) else {
                    return None;
                };

                (r, g, b, a)
            }
            6 => {
                let ((Some(r1), Some(r2)), (Some(g1), Some(g2)), (Some(b1), Some(b2))) = (
                    (hex_to_byte(bytes[0]), hex_to_byte(bytes[1])),
                    (hex_to_byte(bytes[2]), hex_to_byte(bytes[3])),
                    (hex_to_byte(bytes[4]), hex_to_byte(bytes[5])),
                ) else {
                    return None;
                };

                (r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2, 255)
            }
            8 => {
                let (
                    (Some(r1), Some(r2)),
                    (Some(g1), Some(g2)),
                    (Some(b1), Some(b2)),
                    (Some(a1), Some(a2)),
                ) = (
                    (hex_to_byte(bytes[0]), hex_to_byte(bytes[1])),
                    (hex_to_byte(bytes[2]), hex_to_byte(bytes[3])),
                    (hex_to_byte(bytes[4]), hex_to_byte(bytes[5])),
                    (hex_to_byte(bytes[6]), hex_to_byte(bytes[7])),
                )
                else {
                    return None;
                };

                (r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2, a1 * 16 + a2)
            }
            _ => return None,
        };

        Some(Self { r, g, b, a })
    }
}

// automatically generated HTML color names
impl Color {
    pub const TRANSPARENT: Self = Self::hex_panic("0000");
    pub const ALICE_BLUE: Self = Self::hex_panic("F0F8FF");
    pub const ANTIQUE_WHITE: Self = Self::hex_panic("FAEBD7");
    pub const AQUA: Self = Self::hex_panic("00FFFF");
    pub const AQUAMARINE: Self = Self::hex_panic("7FFFD4");
    pub const AZURE: Self = Self::hex_panic("F0FFFF");
    pub const BEIGE: Self = Self::hex_panic("F5F5DC");
    pub const BISQUE: Self = Self::hex_panic("FFE4C4");
    pub const BLACK: Self = Self::hex_panic("000000");
    pub const BLANCHED_ALMOND: Self = Self::hex_panic("FFEBCD");
    pub const BLUE: Self = Self::hex_panic("0000FF");
    pub const BLUE_VIOLET: Self = Self::hex_panic("8A2BE2");
    pub const BROWN: Self = Self::hex_panic("A52A2A");
    pub const BURLY_WOOD: Self = Self::hex_panic("DEB887");
    pub const CADET_BLUE: Self = Self::hex_panic("5F9EA0");
    pub const CHARTREUSE: Self = Self::hex_panic("7FFF00");
    pub const CHOCOLATE: Self = Self::hex_panic("D2691E");
    pub const CORAL: Self = Self::hex_panic("FF7F50");
    pub const CORNFLOWER_BLUE: Self = Self::hex_panic("6495ED");
    pub const CORNSILK: Self = Self::hex_panic("FFF8DC");
    pub const CRIMSON: Self = Self::hex_panic("DC143C");
    pub const CYAN: Self = Self::hex_panic("00FFFF");
    pub const DARK_BLUE: Self = Self::hex_panic("00008B");
    pub const DARK_CYAN: Self = Self::hex_panic("008B8B");
    pub const DARK_GOLDEN_ROD: Self = Self::hex_panic("B8860B");
    pub const DARK_GREY: Self = Self::hex_panic("A9A9A9");
    pub const DARK_GREEN: Self = Self::hex_panic("006400");
    pub const DARK_KHAKI: Self = Self::hex_panic("BDB76B");
    pub const DARK_MAGENTA: Self = Self::hex_panic("8B008B");
    pub const DARK_OLIVE_GREEN: Self = Self::hex_panic("556B2F");
    pub const DARKORANGE: Self = Self::hex_panic("FF8C00");
    pub const DARK_ORCHID: Self = Self::hex_panic("9932CC");
    pub const DARK_RED: Self = Self::hex_panic("8B0000");
    pub const DARK_SALMON: Self = Self::hex_panic("E9967A");
    pub const DARK_SEA_GREEN: Self = Self::hex_panic("8FBC8F");
    pub const DARK_SLATE_BLUE: Self = Self::hex_panic("483D8B");
    pub const DARK_SLATE_GREY: Self = Self::hex_panic("2F4F4F");
    pub const DARK_TURQUOISE: Self = Self::hex_panic("00CED1");
    pub const DARK_VIOLET: Self = Self::hex_panic("9400D3");
    pub const DEEP_PINK: Self = Self::hex_panic("FF1493");
    pub const DEEP_SKY_BLUE: Self = Self::hex_panic("00BFFF");
    pub const DIM_GRAY: Self = Self::hex_panic("696969");
    pub const DODGER_BLUE: Self = Self::hex_panic("1E90FF");
    pub const FIRE_BRICK: Self = Self::hex_panic("B22222");
    pub const FLORAL_WHITE: Self = Self::hex_panic("FFFAF0");
    pub const FOREST_GREEN: Self = Self::hex_panic("228B22");
    pub const FUCHSIA: Self = Self::hex_panic("FF00FF");
    pub const GAINSBORO: Self = Self::hex_panic("DCDCDC");
    pub const GHOST_WHITE: Self = Self::hex_panic("F8F8FF");
    pub const GOLD: Self = Self::hex_panic("FFD700");
    pub const GOLDEN_ROD: Self = Self::hex_panic("DAA520");
    pub const GREY: Self = Self::hex_panic("808080");
    pub const GREEN: Self = Self::hex_panic("008000");
    pub const GREEN_YELLOW: Self = Self::hex_panic("ADFF2F");
    pub const HONEY_DEW: Self = Self::hex_panic("F0FFF0");
    pub const HOT_PINK: Self = Self::hex_panic("FF69B4");
    pub const INDIAN_RED: Self = Self::hex_panic("CD5C5C");
    pub const INDIGO: Self = Self::hex_panic("4B0082");
    pub const IVORY: Self = Self::hex_panic("FFFFF0");
    pub const KHAKI: Self = Self::hex_panic("F0E68C");
    pub const LAVENDER: Self = Self::hex_panic("E6E6FA");
    pub const LAVENDER_BLUSH: Self = Self::hex_panic("FFF0F5");
    pub const LAWN_GREEN: Self = Self::hex_panic("7CFC00");
    pub const LEMON_CHIFFON: Self = Self::hex_panic("FFFACD");
    pub const LIGHT_BLUE: Self = Self::hex_panic("ADD8E6");
    pub const LIGHT_CORAL: Self = Self::hex_panic("F08080");
    pub const LIGHT_CYAN: Self = Self::hex_panic("E0FFFF");
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::hex_panic("FAFAD2");
    pub const LIGHT_GREY: Self = Self::hex_panic("D3D3D3");
    pub const LIGHT_GREEN: Self = Self::hex_panic("90EE90");
    pub const LIGHT_PINK: Self = Self::hex_panic("FFB6C1");
    pub const LIGHT_SALMON: Self = Self::hex_panic("FFA07A");
    pub const LIGHT_SEA_GREEN: Self = Self::hex_panic("20B2AA");
    pub const LIGHT_SKY_BLUE: Self = Self::hex_panic("87CEFA");
    pub const LIGHT_SLATE_GREY: Self = Self::hex_panic("778899");
    pub const LIGHT_STEEL_BLUE: Self = Self::hex_panic("B0C4DE");
    pub const LIGHT_YELLOW: Self = Self::hex_panic("FFFFE0");
    pub const LIME: Self = Self::hex_panic("00FF00");
    pub const LIME_GREEN: Self = Self::hex_panic("32CD32");
    pub const LINEN: Self = Self::hex_panic("FAF0E6");
    pub const MAGENTA: Self = Self::hex_panic("FF00FF");
    pub const MAROON: Self = Self::hex_panic("800000");
    pub const MEDIUM_AQUA_MARINE: Self = Self::hex_panic("66CDAA");
    pub const MEDIUM_BLUE: Self = Self::hex_panic("0000CD");
    pub const MEDIUM_ORCHID: Self = Self::hex_panic("BA55D3");
    pub const MEDIUM_PURPLE: Self = Self::hex_panic("9370D8");
    pub const MEDIUM_SEA_GREEN: Self = Self::hex_panic("3CB371");
    pub const MEDIUM_SLATE_BLUE: Self = Self::hex_panic("7B68EE");
    pub const MEDIUM_SPRING_GREEN: Self = Self::hex_panic("00FA9A");
    pub const MEDIUM_TURQUOISE: Self = Self::hex_panic("48D1CC");
    pub const MEDIUM_VIOLET_RED: Self = Self::hex_panic("C71585");
    pub const MIDNIGHT_BLUE: Self = Self::hex_panic("191970");
    pub const MINT_CREAM: Self = Self::hex_panic("F5FFFA");
    pub const MISTY_ROSE: Self = Self::hex_panic("FFE4E1");
    pub const MOCCASIN: Self = Self::hex_panic("FFE4B5");
    pub const NAVAJO_WHITE: Self = Self::hex_panic("FFDEAD");
    pub const NAVY: Self = Self::hex_panic("000080");
    pub const OLD_LACE: Self = Self::hex_panic("FDF5E6");
    pub const OLIVE: Self = Self::hex_panic("808000");
    pub const OLIVE_DRAB: Self = Self::hex_panic("6B8E23");
    pub const ORANGE: Self = Self::hex_panic("FFA500");
    pub const ORANGE_RED: Self = Self::hex_panic("FF4500");
    pub const ORCHID: Self = Self::hex_panic("DA70D6");
    pub const PALE_GOLDEN_ROD: Self = Self::hex_panic("EEE8AA");
    pub const PALE_GREEN: Self = Self::hex_panic("98FB98");
    pub const PALE_TURQUOISE: Self = Self::hex_panic("AFEEEE");
    pub const PALE_VIOLET_RED: Self = Self::hex_panic("D87093");
    pub const PAPAYA_WHIP: Self = Self::hex_panic("FFEFD5");
    pub const PEACH_PUFF: Self = Self::hex_panic("FFDAB9");
    pub const PERU: Self = Self::hex_panic("CD853F");
    pub const PINK: Self = Self::hex_panic("FFC0CB");
    pub const PLUM: Self = Self::hex_panic("DDA0DD");
    pub const POWDER_BLUE: Self = Self::hex_panic("B0E0E6");
    pub const PURPLE: Self = Self::hex_panic("800080");
    pub const RED: Self = Self::hex_panic("FF0000");
    pub const ROSY_BROWN: Self = Self::hex_panic("BC8F8F");
    pub const ROYAL_BLUE: Self = Self::hex_panic("4169E1");
    pub const SADDLE_BROWN: Self = Self::hex_panic("8B4513");
    pub const SALMON: Self = Self::hex_panic("FA8072");
    pub const SANDY_BROWN: Self = Self::hex_panic("F4A460");
    pub const SEA_GREEN: Self = Self::hex_panic("2E8B57");
    pub const SEA_SHELL: Self = Self::hex_panic("FFF5EE");
    pub const SIENNA: Self = Self::hex_panic("A0522D");
    pub const SILVER: Self = Self::hex_panic("C0C0C0");
    pub const SKY_BLUE: Self = Self::hex_panic("87CEEB");
    pub const SLATE_BLUE: Self = Self::hex_panic("6A5ACD");
    pub const SLATE_GREY: Self = Self::hex_panic("708090");
    pub const SNOW: Self = Self::hex_panic("FFFAFA");
    pub const SPRING_GREEN: Self = Self::hex_panic("00FF7F");
    pub const STEEL_BLUE: Self = Self::hex_panic("4682B4");
    pub const TAN: Self = Self::hex_panic("D2B48C");
    pub const TEAL: Self = Self::hex_panic("008080");
    pub const THISTLE: Self = Self::hex_panic("D8BFD8");
    pub const TOMATO: Self = Self::hex_panic("FF6347");
    pub const TURQUOISE: Self = Self::hex_panic("40E0D0");
    pub const VIOLET: Self = Self::hex_panic("EE82EE");
    pub const WHEAT: Self = Self::hex_panic("F5DEB3");
    pub const WHITE: Self = Self::hex_panic("FFFFFF");
    pub const WHITE_SMOKE: Self = Self::hex_panic("F5F5F5");
    pub const YELLOW: Self = Self::hex_panic("FFFF00");
    pub const YELLOW_GREEN: Self = Self::hex_panic("9ACD32");
}

impl Colorf64 {
    pub const TRANSPARENT: Self = Self::new(0., 0., 0., 0.);
    pub const ALICE_BLUE: Self = Self::new(0.9411764705882353, 0.9725490196078431, 1.0, 1.);
    pub const ANTIQUE_WHITE: Self = Self::new(
        0.9803921568627451,
        0.9215686274509803,
        0.8431372549019608,
        1.,
    );
    pub const AQUA: Self = Self::new(0.0, 1.0, 1.0, 1.);
    pub const AQUAMARINE: Self = Self::new(0.4980392156862745, 1.0, 0.8313725490196079, 1.);
    pub const AZURE: Self = Self::new(0.9411764705882353, 1.0, 1.0, 1.);
    pub const BEIGE: Self = Self::new(
        0.9607843137254902,
        0.9607843137254902,
        0.8627450980392157,
        1.,
    );
    pub const BISQUE: Self = Self::new(1.0, 0.8941176470588236, 0.7686274509803922, 1.);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.);
    pub const BLANCHED_ALMOND: Self = Self::new(1.0, 0.9215686274509803, 0.803921568627451, 1.);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.);
    pub const BLUE_VIOLET: Self = Self::new(
        0.5411764705882353,
        0.16862745098039217,
        0.8862745098039215,
        1.,
    );
    pub const BROWN: Self = Self::new(
        0.6470588235294118,
        0.16470588235294117,
        0.16470588235294117,
        1.,
    );
    pub const BURLY_WOOD: Self = Self::new(
        0.8705882352941177,
        0.7215686274509804,
        0.5294117647058824,
        1.,
    );
    pub const CADET_BLUE: Self = Self::new(
        0.37254901960784315,
        0.6196078431372549,
        0.6274509803921569,
        1.,
    );
    pub const CHARTREUSE: Self = Self::new(0.4980392156862745, 1.0, 0.0, 1.);
    pub const CHOCOLATE: Self = Self::new(
        0.8235294117647058,
        0.4117647058823529,
        0.11764705882352941,
        1.,
    );
    pub const CORAL: Self = Self::new(1.0, 0.4980392156862745, 0.3137254901960784, 1.);
    pub const CORNFLOWER_BLUE: Self = Self::new(
        0.39215686274509803,
        0.5843137254901961,
        0.9294117647058824,
        1.,
    );
    pub const CORNSILK: Self = Self::new(1.0, 0.9725490196078431, 0.8627450980392157, 1.);
    pub const CRIMSON: Self = Self::new(
        0.8627450980392157,
        0.0784313725490196,
        0.23529411764705882,
        1.,
    );
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.);
    pub const DARK_BLUE: Self = Self::new(0.0, 0.0, 0.5450980392156862, 1.);
    pub const DARK_CYAN: Self = Self::new(0.0, 0.5450980392156862, 0.5450980392156862, 1.);
    pub const DARK_GOLDEN_ROD: Self = Self::new(
        0.7215686274509804,
        0.5254901960784314,
        0.043137254901960784,
        1.,
    );
    pub const DARK_GREY: Self = Self::new(
        0.6627450980392157,
        0.6627450980392157,
        0.6627450980392157,
        1.,
    );
    pub const DARK_GREEN: Self = Self::new(0.0, 0.39215686274509803, 0.0, 1.);
    pub const DARK_KHAKI: Self = Self::new(
        0.7411764705882353,
        0.7176470588235294,
        0.4196078431372549,
        1.,
    );
    pub const DARK_MAGENTA: Self = Self::new(0.5450980392156862, 0.0, 0.5450980392156862, 1.);
    pub const DARK_OLIVE_GREEN: Self = Self::new(
        0.3333333333333333,
        0.4196078431372549,
        0.1843137254901961,
        1.,
    );
    pub const DARKORANGE: Self = Self::new(1.0, 0.5490196078431373, 0.0, 1.);
    pub const DARK_ORCHID: Self = Self::new(0.6, 0.19607843137254902, 0.8, 1.);
    pub const DARK_RED: Self = Self::new(0.5450980392156862, 0.0, 0.0, 1.);
    pub const DARK_SALMON: Self = Self::new(
        0.9137254901960784,
        0.5882352941176471,
        0.47843137254901963,
        1.,
    );
    pub const DARK_SEA_GREEN: Self = Self::new(
        0.5607843137254902,
        0.7372549019607844,
        0.5607843137254902,
        1.,
    );
    pub const DARK_SLATE_BLUE: Self = Self::new(
        0.2823529411764706,
        0.23921568627450981,
        0.5450980392156862,
        1.,
    );
    pub const DARK_SLATE_GREY: Self = Self::new(
        0.1843137254901961,
        0.30980392156862746,
        0.30980392156862746,
        1.,
    );
    pub const DARK_TURQUOISE: Self = Self::new(0.0, 0.807843137254902, 0.8196078431372549, 1.);
    pub const DARK_VIOLET: Self = Self::new(0.5803921568627451, 0.0, 0.8274509803921568, 1.);
    pub const DEEP_PINK: Self = Self::new(1.0, 0.0784313725490196, 0.5764705882352941, 1.);
    pub const DEEP_SKY_BLUE: Self = Self::new(0.0, 0.7490196078431373, 1.0, 1.);
    pub const DIM_GRAY: Self = Self::new(
        0.4117647058823529,
        0.4117647058823529,
        0.4117647058823529,
        1.,
    );
    pub const DODGER_BLUE: Self = Self::new(0.11764705882352941, 0.5647058823529412, 1.0, 1.);
    pub const FIRE_BRICK: Self = Self::new(
        0.6980392156862745,
        0.13333333333333333,
        0.13333333333333333,
        1.,
    );
    pub const FLORAL_WHITE: Self = Self::new(1.0, 0.9803921568627451, 0.9411764705882353, 1.);
    pub const FOREST_GREEN: Self = Self::new(
        0.13333333333333333,
        0.5450980392156862,
        0.13333333333333333,
        1.,
    );
    pub const FUCHSIA: Self = Self::new(1.0, 0.0, 1.0, 1.);
    pub const GAINSBORO: Self = Self::new(
        0.8627450980392157,
        0.8627450980392157,
        0.8627450980392157,
        1.,
    );
    pub const GHOST_WHITE: Self = Self::new(0.9725490196078431, 0.9725490196078431, 1.0, 1.);
    pub const GOLD: Self = Self::new(1.0, 0.8431372549019608, 0.0, 1.);
    pub const GOLDEN_ROD: Self = Self::new(
        0.8549019607843137,
        0.6470588235294118,
        0.12549019607843137,
        1.,
    );
    pub const GREY: Self = Self::new(
        0.5019607843137255,
        0.5019607843137255,
        0.5019607843137255,
        1.,
    );
    pub const GREEN: Self = Self::new(0.0, 0.5019607843137255, 0.0, 1.);
    pub const GREEN_YELLOW: Self = Self::new(0.6784313725490196, 1.0, 0.1843137254901961, 1.);
    pub const HONEY_DEW: Self = Self::new(0.9411764705882353, 1.0, 0.9411764705882353, 1.);
    pub const HOT_PINK: Self = Self::new(1.0, 0.4117647058823529, 0.7058823529411765, 1.);
    pub const INDIAN_RED: Self = Self::new(
        0.803921568627451,
        0.3607843137254902,
        0.3607843137254902,
        1.,
    );
    pub const INDIGO: Self = Self::new(0.29411764705882354, 0.0, 0.5098039215686274, 1.);
    pub const IVORY: Self = Self::new(1.0, 1.0, 0.9411764705882353, 1.);
    pub const KHAKI: Self = Self::new(
        0.9411764705882353,
        0.9019607843137255,
        0.5490196078431373,
        1.,
    );
    pub const LAVENDER: Self = Self::new(
        0.9019607843137255,
        0.9019607843137255,
        0.9803921568627451,
        1.,
    );
    pub const LAVENDER_BLUSH: Self = Self::new(1.0, 0.9411764705882353, 0.9607843137254902, 1.);
    pub const LAWN_GREEN: Self = Self::new(0.48627450980392156, 0.9882352941176471, 0.0, 1.);
    pub const LEMON_CHIFFON: Self = Self::new(1.0, 0.9803921568627451, 0.803921568627451, 1.);
    pub const LIGHT_BLUE: Self = Self::new(
        0.6784313725490196,
        0.8470588235294118,
        0.9019607843137255,
        1.,
    );
    pub const LIGHT_CORAL: Self = Self::new(
        0.9411764705882353,
        0.5019607843137255,
        0.5019607843137255,
        1.,
    );
    pub const LIGHT_CYAN: Self = Self::new(0.8784313725490196, 1.0, 1.0, 1.);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::new(
        0.9803921568627451,
        0.9803921568627451,
        0.8235294117647058,
        1.,
    );
    pub const LIGHT_GREY: Self = Self::new(
        0.8274509803921568,
        0.8274509803921568,
        0.8274509803921568,
        1.,
    );
    pub const LIGHT_GREEN: Self = Self::new(
        0.5647058823529412,
        0.9333333333333333,
        0.5647058823529412,
        1.,
    );
    pub const LIGHT_PINK: Self = Self::new(1.0, 0.7137254901960784, 0.7568627450980392, 1.);
    pub const LIGHT_SALMON: Self = Self::new(1.0, 0.6274509803921569, 0.47843137254901963, 1.);
    pub const LIGHT_SEA_GREEN: Self = Self::new(
        0.12549019607843137,
        0.6980392156862745,
        0.6666666666666666,
        1.,
    );
    pub const LIGHT_SKY_BLUE: Self = Self::new(
        0.5294117647058824,
        0.807843137254902,
        0.9803921568627451,
        1.,
    );
    pub const LIGHT_SLATE_GREY: Self = Self::new(0.4666666666666667, 0.5333333333333333, 0.6, 1.);
    pub const LIGHT_STEEL_BLUE: Self = Self::new(
        0.6901960784313725,
        0.7686274509803922,
        0.8705882352941177,
        1.,
    );
    pub const LIGHT_YELLOW: Self = Self::new(1.0, 1.0, 0.8784313725490196, 1.);
    pub const LIME: Self = Self::new(0.0, 1.0, 0.0, 1.);
    pub const LIME_GREEN: Self = Self::new(
        0.19607843137254902,
        0.803921568627451,
        0.19607843137254902,
        1.,
    );
    pub const LINEN: Self = Self::new(
        0.9803921568627451,
        0.9411764705882353,
        0.9019607843137255,
        1.,
    );
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.);
    pub const MAROON: Self = Self::new(0.5019607843137255, 0.0, 0.0, 1.);
    pub const MEDIUM_AQUA_MARINE: Self = Self::new(0.4, 0.803921568627451, 0.6666666666666666, 1.);
    pub const MEDIUM_BLUE: Self = Self::new(0.0, 0.0, 0.803921568627451, 1.);
    pub const MEDIUM_ORCHID: Self = Self::new(
        0.7294117647058823,
        0.3333333333333333,
        0.8274509803921568,
        1.,
    );
    pub const MEDIUM_PURPLE: Self = Self::new(
        0.5764705882352941,
        0.4392156862745098,
        0.8470588235294118,
        1.,
    );
    pub const MEDIUM_SEA_GREEN: Self = Self::new(
        0.23529411764705882,
        0.7019607843137254,
        0.44313725490196076,
        1.,
    );
    pub const MEDIUM_SLATE_BLUE: Self = Self::new(
        0.4823529411764706,
        0.40784313725490196,
        0.9333333333333333,
        1.,
    );
    pub const MEDIUM_SPRING_GREEN: Self =
        Self::new(0.0, 0.9803921568627451, 0.6039215686274509, 1.);
    pub const MEDIUM_TURQUOISE: Self = Self::new(0.2823529411764706, 0.8196078431372549, 0.8, 1.);
    pub const MEDIUM_VIOLET_RED: Self = Self::new(
        0.7803921568627451,
        0.08235294117647059,
        0.5215686274509804,
        1.,
    );
    pub const MIDNIGHT_BLUE: Self = Self::new(
        0.09803921568627451,
        0.09803921568627451,
        0.4392156862745098,
        1.,
    );
    pub const MINT_CREAM: Self = Self::new(0.9607843137254902, 1.0, 0.9803921568627451, 1.);
    pub const MISTY_ROSE: Self = Self::new(1.0, 0.8941176470588236, 0.8823529411764706, 1.);
    pub const MOCCASIN: Self = Self::new(1.0, 0.8941176470588236, 0.7098039215686275, 1.);
    pub const NAVAJO_WHITE: Self = Self::new(1.0, 0.8705882352941177, 0.6784313725490196, 1.);
    pub const NAVY: Self = Self::new(0.0, 0.0, 0.5019607843137255, 1.);
    pub const OLD_LACE: Self = Self::new(
        0.9921568627450981,
        0.9607843137254902,
        0.9019607843137255,
        1.,
    );
    pub const OLIVE: Self = Self::new(0.5019607843137255, 0.5019607843137255, 0.0, 1.);
    pub const OLIVE_DRAB: Self = Self::new(
        0.4196078431372549,
        0.5568627450980392,
        0.13725490196078433,
        1.,
    );
    pub const ORANGE: Self = Self::new(1.0, 0.6470588235294118, 0.0, 1.);
    pub const ORANGE_RED: Self = Self::new(1.0, 0.27058823529411763, 0.0, 1.);
    pub const ORCHID: Self = Self::new(
        0.8549019607843137,
        0.4392156862745098,
        0.8392156862745098,
        1.,
    );
    pub const PALE_GOLDEN_ROD: Self = Self::new(
        0.9333333333333333,
        0.9098039215686274,
        0.6666666666666666,
        1.,
    );
    pub const PALE_GREEN: Self =
        Self::new(0.596078431372549, 0.984313725490196, 0.596078431372549, 1.);
    pub const PALE_TURQUOISE: Self = Self::new(
        0.6862745098039216,
        0.9333333333333333,
        0.9333333333333333,
        1.,
    );
    pub const PALE_VIOLET_RED: Self = Self::new(
        0.8470588235294118,
        0.4392156862745098,
        0.5764705882352941,
        1.,
    );
    pub const PAPAYA_WHIP: Self = Self::new(1.0, 0.9372549019607843, 0.8352941176470589, 1.);
    pub const PEACH_PUFF: Self = Self::new(1.0, 0.8549019607843137, 0.7254901960784313, 1.);
    pub const PERU: Self = Self::new(
        0.803921568627451,
        0.5215686274509804,
        0.24705882352941178,
        1.,
    );
    pub const PINK: Self = Self::new(1.0, 0.7529411764705882, 0.796078431372549, 1.);
    pub const PLUM: Self = Self::new(
        0.8666666666666667,
        0.6274509803921569,
        0.8666666666666667,
        1.,
    );
    pub const POWDER_BLUE: Self = Self::new(
        0.6901960784313725,
        0.8784313725490196,
        0.9019607843137255,
        1.,
    );
    pub const PURPLE: Self = Self::new(0.5019607843137255, 0.0, 0.5019607843137255, 1.);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.);
    pub const ROSY_BROWN: Self = Self::new(
        0.7372549019607844,
        0.5607843137254902,
        0.5607843137254902,
        1.,
    );
    pub const ROYAL_BLUE: Self = Self::new(
        0.2549019607843137,
        0.4117647058823529,
        0.8823529411764706,
        1.,
    );
    pub const SADDLE_BROWN: Self = Self::new(
        0.5450980392156862,
        0.27058823529411763,
        0.07450980392156863,
        1.,
    );
    pub const SALMON: Self = Self::new(
        0.9803921568627451,
        0.5019607843137255,
        0.4470588235294118,
        1.,
    );
    pub const SANDY_BROWN: Self = Self::new(
        0.9568627450980393,
        0.6431372549019608,
        0.3764705882352941,
        1.,
    );
    pub const SEA_GREEN: Self = Self::new(
        0.1803921568627451,
        0.5450980392156862,
        0.3411764705882353,
        1.,
    );
    pub const SEA_SHELL: Self = Self::new(1.0, 0.9607843137254902, 0.9333333333333333, 1.);
    pub const SIENNA: Self = Self::new(
        0.6274509803921569,
        0.3215686274509804,
        0.17647058823529413,
        1.,
    );
    pub const SILVER: Self = Self::new(
        0.7529411764705882,
        0.7529411764705882,
        0.7529411764705882,
        1.,
    );
    pub const SKY_BLUE: Self = Self::new(
        0.5294117647058824,
        0.807843137254902,
        0.9215686274509803,
        1.,
    );
    pub const SLATE_BLUE: Self = Self::new(
        0.41568627450980394,
        0.35294117647058826,
        0.803921568627451,
        1.,
    );
    pub const SLATE_GREY: Self = Self::new(
        0.4392156862745098,
        0.5019607843137255,
        0.5647058823529412,
        1.,
    );
    pub const SNOW: Self = Self::new(1.0, 0.9803921568627451, 0.9803921568627451, 1.);
    pub const SPRING_GREEN: Self = Self::new(0.0, 1.0, 0.4980392156862745, 1.);
    pub const STEEL_BLUE: Self = Self::new(
        0.27450980392156865,
        0.5098039215686274,
        0.7058823529411765,
        1.,
    );
    pub const TAN: Self = Self::new(
        0.8235294117647058,
        0.7058823529411765,
        0.5490196078431373,
        1.,
    );
    pub const TEAL: Self = Self::new(0.0, 0.5019607843137255, 0.5019607843137255, 1.);
    pub const THISTLE: Self = Self::new(
        0.8470588235294118,
        0.7490196078431373,
        0.8470588235294118,
        1.,
    );
    pub const TOMATO: Self = Self::new(1.0, 0.38823529411764707, 0.2784313725490196, 1.);
    pub const TURQUOISE: Self = Self::new(
        0.25098039215686274,
        0.8784313725490196,
        0.8156862745098039,
        1.,
    );
    pub const VIOLET: Self = Self::new(
        0.9333333333333333,
        0.5098039215686274,
        0.9333333333333333,
        1.,
    );
    pub const WHEAT: Self = Self::new(
        0.9607843137254902,
        0.8705882352941177,
        0.7019607843137254,
        1.,
    );
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.);
    pub const WHITE_SMOKE: Self = Self::new(
        0.9607843137254902,
        0.9607843137254902,
        0.9607843137254902,
        1.,
    );
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.);
    pub const YELLOW_GREEN: Self = Self::new(
        0.6039215686274509,
        0.803921568627451,
        0.19607843137254902,
        1.,
    );
}
