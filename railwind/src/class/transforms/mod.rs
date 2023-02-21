mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("translate.ron")).unwrap();
    pub static ref ROTATE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("rotate.ron")).unwrap();
    pub static ref SKEW: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("skew.ron")).unwrap();
    pub static ref SCALE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("scale.ron")).unwrap();
    pub static ref ORIGIN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("origin.ron")).unwrap();
}

#[derive(Debug)]
pub enum Transform<'a> {
    TranslateX(TranslateX<'a>),
    TranslateY(TranslateY<'a>),
    Rotate(Rotate<'a>),
    SkewX(SkewX<'a>),
    SkewY(SkewY<'a>),
    Scale(Scale<'a>),
    Origin(Origin<'a>),
}

impl<'a> Transform<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let args = if let Ok(str) = get_args(value) {
            str
        } else {
            return Ok(None);
        };

        let class_name = get_class_name(value);

        let transform = match class_name {
            "translate" | "-translate" => match get_class_name(args) {
                "x" => Self::TranslateX(TranslateX::new(class_name, get_args(args)?)),

                "y" => Self::TranslateY(TranslateY::new(class_name, get_args(args)?)),
                v => {
                    return Err(WarningType::InvalidArg(
                        v.into(),
                        "Translate X / Y".into(),
                        vec!["x", "y"],
                    ))
                }
            },
            "rotate" | "-rotate" => Self::Rotate(Rotate::new(class_name, args)),
            "skew" | "-skew" => match get_class_name(args) {
                "x" => Self::SkewX(SkewX::new(class_name, get_args(args)?)),

                "y" => Self::SkewY(SkewY::new(class_name, get_args(args)?)),
                v => {
                    return Err(WarningType::InvalidArg(
                        v.into(),
                        "Skew X / Y".into(),
                        vec!["x", "y"],
                    ))
                }
            },
            "scale" | "-scale" => Self::Scale(Scale::new(value)?),
            "origin" => Self::Origin(Origin(args)),
            _ => return Ok(None),
        };

        Ok(Some(transform))
    }
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::TranslateX(s) => s.to_decl(),
            Self::TranslateY(s) => s.to_decl(),
            Self::Rotate(s) => s.to_decl(),
            Self::SkewX(s) => s.to_decl(),
            Self::SkewY(s) => s.to_decl(),
            Self::Scale(s) => s.to_decl(),
            Self::Origin(s) => s.to_decl(),
        }
    }
}
