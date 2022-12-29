mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

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
    pub fn new(value: &'a str) -> Option<Self> {
        let args = get_args(value)?;
        let class_name = get_class_name(value);

        let transform = match class_name {
            "translate" | "-translate" => match get_class_name(args) {
                "x" => Transform::TranslateX(TranslateX::new(class_name, get_args(args)?)),

                "y" => Transform::TranslateY(TranslateY::new(class_name, get_args(args)?)),
                _ => return None,
            },
            "rotate" | "-rotate" => Transform::Rotate(Rotate::new(class_name, args)),
            "skew" | "-skew" => match get_class_name(args) {
                "x" => Transform::SkewX(SkewX::new(class_name, get_args(args)?)),

                "y" => Transform::SkewY(SkewY::new(class_name, get_args(args)?)),
                _ => return None,
            },
            "scale" | "-scale" => Transform::Scale(Scale::new(value)?),
            "origin" => Transform::Origin(Origin(args)),
            _ => return None,
        };
        Some(transform)
    }
    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Transform::TranslateX(s) => s.to_decl(),
            Transform::TranslateY(s) => s.to_decl(),
            Transform::Rotate(s) => s.to_decl(),
            Transform::SkewX(s) => s.to_decl(),
            Transform::SkewY(s) => s.to_decl(),
            Transform::Scale(s) => s.to_decl(),
            Transform::Origin(s) => s.to_decl(),
        }
    }
}
