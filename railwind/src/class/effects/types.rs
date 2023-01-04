use crate::class::utils::get_value;
use crate::class::Decl;
use regex::{ Regex, Captures };

use super::{BOX_SHADOW, BOX_SHADOW_COLOR, OPACITY};

const BOX_SHADOW_STYLE: &str = "box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)";

#[derive(Debug)]
pub struct BoxShadow<'a>(pub &'a str);

impl<'a> BoxShadow<'a> {
    pub fn new(arg: &'a str) -> Self {
        match arg {
            "shadow" => Self(""),
            _ => Self(arg),
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        let tw_shadow = get_value(self.0, &BOX_SHADOW)?;
        let re = Regex::new(r"rgb\(.*?\)").unwrap();
        let tw_shadow_colored = re.replace_all(&tw_shadow,
            |_: &Captures| {
                String::from("var(--tw-shadow-color)")
            }
        );
        Some(Decl::Triple([
            format!("--tw-shadow: {}", tw_shadow),
            format!("--tw-shadow-colored: {}", tw_shadow_colored),
            BOX_SHADOW_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BoxShadowColor<'a>(pub &'a str);
impl<'a> BoxShadowColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BOX_SHADOW_COLOR)?;
        Some(Decl::Double([
            format!("--tw-shadow-color: {}", value),
            "--tw-shadow: var(--tw-shadow-colored)".into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Opacity<'a>(pub &'a str);
impl<'a> Opacity<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &OPACITY)?;
        Some(Decl::Single(format!("opacity: {}", value)))
    }
}

#[derive(Debug)]
pub enum MixBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    PlusLighter,
}

impl MixBlendMode {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "normal" => Self::Normal,
            "multiply" => Self::Multiply,
            "screen" => Self::Screen,
            "overlay" => Self::Overlay,
            "darken" => Self::Darken,
            "lighten" => Self::Lighten,
            "color-dodge" => Self::ColorDodge,
            "color-burn" => Self::ColorBurn,
            "hard-light" => Self::HardLight,
            "soft-light" => Self::SoftLight,
            "difference" => Self::Difference,
            "exclusion" => Self::Exclusion,
            "hue" => Self::Hue,
            "saturation" => Self::Saturation,
            "color" => Self::Color,
            "luminosity" => Self::Luminosity,
            "plus-lighter" => Self::PlusLighter,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Normal => "normal",
            Self::Multiply => "multiply",
            Self::Screen => "screen",
            Self::Overlay => "overlay",
            Self::Darken => "darken",
            Self::Lighten => "lighten",
            Self::ColorDodge => "color-dodge",
            Self::ColorBurn => "color-burn",
            Self::HardLight => "hard-light",
            Self::SoftLight => "soft-light",
            Self::Difference => "difference",
            Self::Exclusion => "exclusion",
            Self::Hue => "hue",
            Self::Saturation => "saturation",
            Self::Color => "color",
            Self::Luminosity => "luminosity",
            Self::PlusLighter => "plus-lighter",
        };

        Decl::Single(format!("mix-blend-mode: {}", val))
    }
}

#[derive(Debug)]
pub enum BackgroundBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BackgroundBlendMode {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "normal" => Self::Normal,
            "multiply" => Self::Multiply,
            "screen" => Self::Screen,
            "overlay" => Self::Overlay,
            "darken" => Self::Darken,
            "lighten" => Self::Lighten,
            "color-dodge" => Self::ColorDodge,
            "color-burn" => Self::ColorBurn,
            "hard-light" => Self::HardLight,
            "soft-light" => Self::SoftLight,
            "difference" => Self::Difference,
            "exclusion" => Self::Exclusion,
            "hue" => Self::Hue,
            "saturation" => Self::Saturation,
            "color" => Self::Color,
            "luminosity" => Self::Luminosity,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Normal => "normal",
            Self::Multiply => "multiply",
            Self::Screen => "screen",
            Self::Overlay => "overlay",
            Self::Darken => "darken",
            Self::Lighten => "lighten",
            Self::ColorDodge => "color-dodge",
            Self::ColorBurn => "color-burn",
            Self::HardLight => "hard-light",
            Self::SoftLight => "soft-light",
            Self::Difference => "difference",
            Self::Exclusion => "exclusion",
            Self::Hue => "hue",
            Self::Saturation => "saturation",
            Self::Color => "color",
            Self::Luminosity => "luminosity",
        };

        Decl::Single(format!("background-blend-mode: {}", val))
    }
}
