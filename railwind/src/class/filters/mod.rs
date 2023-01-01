mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BLUR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("blur.ron")).unwrap();
    pub static ref BRIGHTNESS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("brightness.ron")).unwrap();
    pub static ref CONTRAST: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("contrast.ron")).unwrap();
    pub static ref DROP_SHADOW: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("drop_shadow.ron")).unwrap();
    pub static ref GRAYSCALE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grayscale.ron")).unwrap();
    pub static ref HUE_ROTATE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("hue_rotate.ron")).unwrap();
    pub static ref INVERT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("invert.ron")).unwrap();
    pub static ref SATURATE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("saturate.ron")).unwrap();
    pub static ref SEPIA: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("sepia.ron")).unwrap();
    pub static ref OPACITY: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("opacity.ron")).unwrap();
}

#[derive(Debug)]
pub enum Filter<'a> {
    Blur(Blur<'a>),
    Brightness(Brightness<'a>),
    Contrast(Contrast<'a>),
    DropShadow(DropShadow<'a>),
    Grayscale(Grayscale<'a>),
    HueRotate(HueRotate<'a>),
    Invert(Invert<'a>),
    Saturate(Saturate<'a>),
    Sepia(Sepia<'a>),
    BackdropBlur(BackdropBlur<'a>),
    BackdropBrightness(BackdropBrightness<'a>),
    BackdropContrast(BackdropContrast<'a>),
    BackdropGrayscale(BackdropGrayscale<'a>),
    BackdropHueRotate(BackdropHueRotate<'a>),
    BackdropInvert(BackdropInvert<'a>),
    BackdropOpacity(BackdropOpacity<'a>),
    BackdropSaturate(BackdropSaturate<'a>),
    BackdropSepia(BackdropSepia<'a>),
}

impl<'a> Filter<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let class_name = get_class_name(value);

        let filter = match class_name {
            "blur" => {
                if let Some(args) = get_args(value) {
                    Filter::Blur(Blur(args))
                } else {
                    Filter::Blur(Blur(""))
                }
            }
            "brightness" => Filter::Brightness(Brightness(get_args(value)?)),
            "contrast" => Filter::Contrast(Contrast(get_args(value)?)),
            "drop" => match get_class_name(get_args(value)?) {
                "shadow" => {
                    if let Some(args) = get_args(get_args(value)?) {
                        Filter::DropShadow(DropShadow(args))
                    } else {
                        Filter::DropShadow(DropShadow(""))
                    }
                }
                _ => return None,
            },
            "grayscale" => {
                if let Some(args) = get_args(value) {
                    Filter::Grayscale(Grayscale(args))
                } else {
                    Filter::Grayscale(Grayscale(""))
                }
            }
            "hue" | "-hue" => match get_class_name(get_args(value)?) {
                "rotate" => {
                    Filter::HueRotate(HueRotate::new(class_name, get_args(get_args(value)?)?))
                }
                _ => return None,
            },
            "invert" => {
                if let Some(args) = get_args(value) {
                    Filter::Invert(Invert(args))
                } else {
                    Filter::Invert(Invert(""))
                }
            }
            "saturate" => Filter::Saturate(Saturate(get_args(value)?)),
            "sepia" => {
                if let Some(args) = get_args(value) {
                    Filter::Sepia(Sepia(args))
                } else {
                    Filter::Sepia(Sepia(""))
                }
            }
            "backdrop" | "-backdrop" => {
                let args = get_args(value)?;
                let sub_class_name = get_class_name(args);
                match sub_class_name {
                    "blur" => {
                        if let Some(args) = get_args(get_args(value)?) {
                            Filter::BackdropBlur(BackdropBlur(args))
                        } else {
                            Filter::BackdropBlur(BackdropBlur(""))
                        }
                    }
                    "brightness" => Filter::BackdropBrightness(BackdropBrightness(get_args(args)?)),
                    "contrast" => Filter::BackdropContrast(BackdropContrast(get_args(args)?)),
                    "grayscale" => {
                        if let Some(args) = get_args(args) {
                            Filter::BackdropGrayscale(BackdropGrayscale(args))
                        } else {
                            Filter::BackdropGrayscale(BackdropGrayscale(""))
                        }
                    }
                    "hue" => match get_class_name(get_args(get_args(value)?)?) {
                        "rotate" => Filter::BackdropHueRotate(BackdropHueRotate::new(
                            class_name,
                            get_args(get_args(args)?)?,
                        )),
                        _ => return None,
                    },
                    "invert" => {
                        if let Some(args) = get_args(args) {
                            Filter::BackdropInvert(BackdropInvert(args))
                        } else {
                            Filter::BackdropInvert(BackdropInvert(""))
                        }
                    }
                    "opacity" => Filter::BackdropOpacity(BackdropOpacity(get_args(args)?)),
                    "saturate" => Filter::BackdropSaturate(BackdropSaturate(get_args(args)?)),
                    "sepia" => {
                        if let Some(args) = get_args(args) {
                            Filter::BackdropSepia(BackdropSepia(args))
                        } else {
                            Filter::BackdropSepia(BackdropSepia(""))
                        }
                    }
                    _ => return None,
                }
            }
            _ => return None,
        };
        Some(filter)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Filter::Blur(s) => s.to_decl(),
            Filter::Brightness(s) => s.to_decl(),
            Filter::Contrast(s) => s.to_decl(),
            Filter::DropShadow(s) => s.to_decl(),
            Filter::Grayscale(s) => s.to_decl(),
            Filter::HueRotate(s) => s.to_decl(),
            Filter::Invert(s) => s.to_decl(),
            Filter::Saturate(s) => s.to_decl(),
            Filter::Sepia(s) => s.to_decl(),
            Filter::BackdropBlur(s) => s.to_decl(),
            Filter::BackdropBrightness(s) => s.to_decl(),
            Filter::BackdropContrast(s) => s.to_decl(),
            Filter::BackdropGrayscale(s) => s.to_decl(),
            Filter::BackdropHueRotate(s) => s.to_decl(),
            Filter::BackdropInvert(s) => s.to_decl(),
            Filter::BackdropOpacity(s) => s.to_decl(),
            Filter::BackdropSaturate(s) => s.to_decl(),
            Filter::BackdropSepia(s) => s.to_decl(),
        }
    }
}
