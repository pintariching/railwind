mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};
use crate::warning::WarningType;

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

#[derive(Debug, PartialEq, Hash)]
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
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let class_name = get_class_name(value);

        let filter = match class_name {
            "blur" => {
                if let Ok(args) = get_args(value) {
                    Self::Blur(Blur(args))
                } else {
                    Self::Blur(Blur(""))
                }
            }
            "brightness" => Self::Brightness(Brightness(get_args(value)?)),
            "contrast" => Self::Contrast(Contrast(get_args(value)?)),
            "drop" => match get_class_name(get_args(value)?) {
                "shadow" => {
                    if let Ok(args) = get_args(get_args(value)?) {
                        Self::DropShadow(DropShadow(args))
                    } else {
                        Self::DropShadow(DropShadow(""))
                    }
                }
                v => {
                    return Err(WarningType::InvalidArg(
                        v.into(),
                        "Drop Shadow".into(),
                        vec!["shadow"],
                    ))
                }
            },
            "grayscale" => {
                if let Ok(args) = get_args(value) {
                    Self::Grayscale(Grayscale(args))
                } else {
                    Self::Grayscale(Grayscale(""))
                }
            }
            "hue" | "-hue" => match get_class_name(get_args(value)?) {
                "rotate" => {
                    Self::HueRotate(HueRotate::new(class_name, get_args(get_args(value)?)?))
                }
                v => {
                    return Err(WarningType::InvalidArg(
                        v.into(),
                        "Hue Rotate".into(),
                        vec!["rotate"],
                    ))
                }
            },
            "invert" => {
                if let Ok(args) = get_args(value) {
                    Self::Invert(Invert(args))
                } else {
                    Self::Invert(Invert(""))
                }
            }
            "saturate" => Self::Saturate(Saturate(get_args(value)?)),
            "sepia" => {
                if let Ok(args) = get_args(value) {
                    Self::Sepia(Sepia(args))
                } else {
                    Self::Sepia(Sepia(""))
                }
            }
            "backdrop" | "-backdrop" => {
                let args = get_args(value)?;
                let sub_class_name = get_class_name(args);
                match sub_class_name {
                    "blur" => {
                        if let Ok(args) = get_args(get_args(value)?) {
                            Self::BackdropBlur(BackdropBlur(args))
                        } else {
                            Self::BackdropBlur(BackdropBlur(""))
                        }
                    }
                    "brightness" => Self::BackdropBrightness(BackdropBrightness(get_args(args)?)),
                    "contrast" => Self::BackdropContrast(BackdropContrast(get_args(args)?)),
                    "grayscale" => {
                        if let Ok(args) = get_args(args) {
                            Self::BackdropGrayscale(BackdropGrayscale(args))
                        } else {
                            Self::BackdropGrayscale(BackdropGrayscale(""))
                        }
                    }
                    "hue" => match get_class_name(get_args(get_args(value)?)?) {
                        "rotate" => Self::BackdropHueRotate(BackdropHueRotate::new(
                            class_name,
                            get_args(get_args(args)?)?,
                        )),
                        v => {
                            return Err(WarningType::InvalidArg(
                                v.into(),
                                "Backdrop Hue Rotate".into(),
                                vec!["rotate"],
                            ))
                        }
                    },
                    "invert" => {
                        if let Ok(args) = get_args(args) {
                            Self::BackdropInvert(BackdropInvert(args))
                        } else {
                            Self::BackdropInvert(BackdropInvert(""))
                        }
                    }
                    "opacity" => Self::BackdropOpacity(BackdropOpacity(get_args(args)?)),
                    "saturate" => Self::BackdropSaturate(BackdropSaturate(get_args(args)?)),
                    "sepia" => {
                        if let Ok(args) = get_args(args) {
                            Self::BackdropSepia(BackdropSepia(args))
                        } else {
                            Self::BackdropSepia(BackdropSepia(""))
                        }
                    }
                    v => {
                        return Err(WarningType::InvalidArg(
                            v.into(),
                            "Backdrop".into(),
                            vec![
                                "blur",
                                "brightness",
                                "contrast",
                                "grayscale",
                                "hue",
                                "invert",
                                "opacity",
                                "saturate",
                                "sepia",
                            ],
                        ))
                    }
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(filter))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Blur(s) => s.to_decl(),
            Self::Brightness(s) => s.to_decl(),
            Self::Contrast(s) => s.to_decl(),
            Self::DropShadow(s) => s.to_decl(),
            Self::Grayscale(s) => s.to_decl(),
            Self::HueRotate(s) => s.to_decl(),
            Self::Invert(s) => s.to_decl(),
            Self::Saturate(s) => s.to_decl(),
            Self::Sepia(s) => s.to_decl(),
            Self::BackdropBlur(s) => s.to_decl(),
            Self::BackdropBrightness(s) => s.to_decl(),
            Self::BackdropContrast(s) => s.to_decl(),
            Self::BackdropGrayscale(s) => s.to_decl(),
            Self::BackdropHueRotate(s) => s.to_decl(),
            Self::BackdropInvert(s) => s.to_decl(),
            Self::BackdropOpacity(s) => s.to_decl(),
            Self::BackdropSaturate(s) => s.to_decl(),
            Self::BackdropSepia(s) => s.to_decl(),
        }
    }
}
