use crate::class::utils::{get_value, get_value_neg};
use crate::class::Decl;
// use crate::utils::{get_args, get_class_name, get_opt_args};

use super::{
    BLUR, BRIGHTNESS, CONTRAST, DROP_SHADOW, GRAYSCALE, HUE_ROTATE, INVERT, OPACITY, SATURATE,
    SEPIA,
};

const FILTER_STYLE: &str = "filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)";
const WEBKIT_BACKDROP_FILTER_STYLE: &str = "-webkit-backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)";
const BACKDROP_FILTER_STYLE: &str = "        backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)";

#[derive(Debug)]
pub struct Blur<'a>(pub &'a str);

impl<'a> Blur<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BLUR)?;
        Some(Decl::Double([
            format!("--tw-blur: blur({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Brightness<'a>(pub &'a str);

impl<'a> Brightness<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BRIGHTNESS)?;
        Some(Decl::Double([
            format!("--tw-brightness: brightness({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Contrast<'a>(pub &'a str);

impl<'a> Contrast<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &CONTRAST)?;
        Some(Decl::Double([
            format!("--tw-contrast: contrast({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct DropShadow<'a>(pub &'a str);

impl<'a> DropShadow<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &DROP_SHADOW)?;
        Some(Decl::Double([
            format!("--tw-drop-shadow: drop-shadow({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Grayscale<'a>(pub &'a str);

impl<'a> Grayscale<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GRAYSCALE)?;
        Some(Decl::Double([
            format!("--tw-grayscale: grayscale({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct HueRotate<'a>(pub &'a str, bool);

impl<'a> HueRotate<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &HUE_ROTATE)?;
        Some(Decl::Double([
            format!("--tw-hue-rotate: hue-rotate({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Invert<'a>(pub &'a str);

impl<'a> Invert<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &INVERT)?;
        Some(Decl::Double([
            format!("--tw-invert: invert({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Saturate<'a>(pub &'a str);

impl<'a> Saturate<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &SATURATE)?;
        Some(Decl::Double([
            format!("--tw-saturate: saturate({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Sepia<'a>(pub &'a str);

impl<'a> Sepia<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &SEPIA)?;
        Some(Decl::Double([
            format!("--tw-sepia: sepia({})", value),
            FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropBlur<'a>(pub &'a str);

impl<'a> BackdropBlur<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BLUR)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-blur: blur({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropBrightness<'a>(pub &'a str);

impl<'a> BackdropBrightness<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BRIGHTNESS)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-brightness: brightness({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropContrast<'a>(pub &'a str);

impl<'a> BackdropContrast<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &CONTRAST)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-contrast: contrast({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropGrayscale<'a>(pub &'a str);

impl<'a> BackdropGrayscale<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GRAYSCALE)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-grayscale: grayscale({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropHueRotate<'a>(pub &'a str, bool);

impl<'a> BackdropHueRotate<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &HUE_ROTATE)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-hue-rotate: hue-rotate({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropInvert<'a>(pub &'a str);

impl<'a> BackdropInvert<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &INVERT)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-invert: invert({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropOpacity<'a>(pub &'a str);

impl<'a> BackdropOpacity<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &OPACITY)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-opacity: opacity({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropSaturate<'a>(pub &'a str);

impl<'a> BackdropSaturate<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &SATURATE)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-saturate: saturate({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct BackdropSepia<'a>(pub &'a str);

impl<'a> BackdropSepia<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &SEPIA)?;
        Some(Decl::Triple([
            format!("--tw-backdrop-sepia: sepia({})", value),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ]))
    }
}
