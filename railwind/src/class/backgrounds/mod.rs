mod types;

pub use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{utils::value_is_hex, Decl};
use crate::utils::{get_args, get_class_name};

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

#[derive(Debug)]
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
    pub fn new(value: &'a str) -> Option<Self> {
        let args = get_args(value)?;

        let backgrounds = match get_class_name(value) {
            "bg" => match get_class_name(args) {
                "clip" => Backgrounds::BackgroundClip(BackgroundClip::new(get_args(args)?)?),
                "origin" => Backgrounds::BackgroundOrigin(BackgroundOrigin::new(get_args(args)?)?),
                "gradient" | "none" if BACKGROUND_IMAGE.contains_key(args) => {
                    Backgrounds::BackgroundImage(BackgroundImage(args))
                }
                _ => {
                    if let Some(attachment) = BackgroundAttachment::new(args) {
                        Backgrounds::BackgroundAttachment(attachment)
                    } else if let Some(repeat) = BackgroundRepeat::new(args) {
                        Backgrounds::BackgroundRepeat(repeat)
                    } else if BACKGROUND_SIZE.contains_key(args) || args.starts_with("[length:") {
                        Backgrounds::BackgroundSize(BackgroundSize(args))
                    } else if args.contains("url(") {
                        Backgrounds::BackgroundImage(BackgroundImage(args))
                    } else if BACKGROUND_POSITION.contains_key(args)
                        || (!value_is_hex(args) && !&BACKGROUND_COLOR.contains_key(args))
                    {
                        Backgrounds::BackgroundPosition(BackgroundPosition(args))
                    } else {
                        Backgrounds::BackgroundColor(BackgroundColor(args))
                    }
                }
            },
            "from" | "via" | "to" => Backgrounds::GradientColorStops(GradientColorStops::new(
                get_class_name(value),
                args,
            )?),
            _ => return None,
        };

        Some(backgrounds)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Backgrounds::BackgroundAttachment(b) => Some(b.to_decl()),
            Backgrounds::BackgroundClip(b) => Some(b.to_decl()),
            Backgrounds::BackgroundColor(b) => b.to_decl(),
            Backgrounds::BackgroundOrigin(b) => Some(b.to_decl()),
            Backgrounds::BackgroundPosition(b) => b.to_decl(),
            Backgrounds::BackgroundRepeat(b) => Some(b.to_decl()),
            Backgrounds::BackgroundSize(b) => b.to_decl(),
            Backgrounds::BackgroundImage(b) => b.to_decl(),
            Backgrounds::GradientColorStops(b) => b.to_decl(),
        }
    }
}
