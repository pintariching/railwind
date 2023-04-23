mod types;

pub use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{utils::value_is_hex, Decl};
use crate::{
    utils::{get_args, get_class_name},
    warning::WarningType,
};

lazy_static! {
    pub static ref BACKGROUND_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref BACKGROUND_POSITION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("background_position.ron")).unwrap();
    pub static ref BACKGROUND_SIZE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("background_size.ron")).unwrap();
    pub static ref BACKGROUND_IMAGE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("background_image.ron")).unwrap();
    pub static ref GRADIENT_COLOR_STOPS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
}

#[derive(Debug, PartialEq, Hash)]
pub enum Backgrounds<'a> {
    BackgroundAttachment(BackgroundAttachment),
    BackgroundClip(BackgroundClip),
    BackgroundColor(BackgroundColor<'a>),
    BackgroundOrigin(BackgroundOrigin),
    BackgroundPosition(BackgroundPosition<'a>),
    BackgroundRepeat(BackgroundRepeat),
    BackgroundSize(BackgroundSize<'a>),
    BackgroundImage(BackgroundImage<'a>),
    GradientColorStops(GradientColorStops<'a>),
}

impl<'a> Backgrounds<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let args = if let Ok(str) = get_args(value) {
            str
        } else {
            return Ok(None);
        };

        let backgrounds = match get_class_name(value) {
            "bg" => match get_class_name(args) {
                "clip" => Self::BackgroundClip(BackgroundClip::new(get_args(args)?)?),
                "origin" => Self::BackgroundOrigin(BackgroundOrigin::new(get_args(args)?)?),
                "gradient" | "none" if BACKGROUND_IMAGE.contains_key(args) => {
                    Self::BackgroundImage(BackgroundImage(args))
                }
                _ => {
                    if let Some(attachment) = BackgroundAttachment::new(args) {
                        Self::BackgroundAttachment(attachment)
                    } else if let Some(repeat) = BackgroundRepeat::new(args) {
                        Self::BackgroundRepeat(repeat)
                    } else if BACKGROUND_SIZE.contains_key(args) || args.starts_with("[length:") {
                        Self::BackgroundSize(BackgroundSize(args))
                    } else if args.contains("url(") {
                        Self::BackgroundImage(BackgroundImage(args))
                    } else if BACKGROUND_POSITION.contains_key(args)
                        || (!value_is_hex(args) && !&BACKGROUND_COLOR.contains_key(args))
                    {
                        Self::BackgroundPosition(BackgroundPosition(args))
                    } else {
                        Self::BackgroundColor(BackgroundColor(args))
                    }
                }
            },
            "from" | "via" | "to" => {
                Self::GradientColorStops(GradientColorStops::new(get_class_name(value), args)?)
            }
            _ => return Ok(None),
        };

        Ok(Some(backgrounds))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::BackgroundAttachment(b) => Ok(b.to_decl()),
            Self::BackgroundClip(b) => Ok(b.to_decl()),
            Self::BackgroundColor(b) => b.to_decl(),
            Self::BackgroundOrigin(b) => Ok(b.to_decl()),
            Self::BackgroundPosition(b) => b.to_decl(),
            Self::BackgroundRepeat(b) => Ok(b.to_decl()),
            Self::BackgroundSize(b) => b.to_decl(),
            Self::BackgroundImage(b) => b.to_decl(),
            Self::GradientColorStops(b) => b.to_decl(),
        }
    }
}
