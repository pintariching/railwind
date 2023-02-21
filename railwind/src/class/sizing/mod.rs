mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("width.ron")).unwrap();
    pub static ref MIN_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("min_width.ron")).unwrap();
    pub static ref MAX_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("max_width.ron")).unwrap();
    pub static ref HEIGHT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("height.ron")).unwrap();
    pub static ref MIN_HEIGHT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("min_height.ron")).unwrap();
    pub static ref MAX_HEIGHT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("max_height.ron")).unwrap();
}

#[derive(Debug)]
pub enum Sizing<'a> {
    Width(Width<'a>),
    MinWidth(MinWidth<'a>),
    MaxWidth(MaxWidth<'a>),
    Height(Height<'a>),
    MinHeight(MinHeight<'a>),
    MaxHeight(MaxHeight<'a>),
}

impl<'a> Sizing<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let args = if let Ok(str) = get_args(value) {
            str
        } else {
            return Ok(None);
        };

        let sizing = match get_class_name(value) {
            "w" => Self::Width(Width(args)),
            "h" => Self::Height(Height(args)),
            "min" => match get_class_name(args) {
                "w" => Self::MinWidth(MinWidth(get_args(args)?)),
                "h" => Self::MinHeight(MinHeight(get_args(args)?)),
                v => {
                    return Err(WarningType::InvalidArg(
                        v.into(),
                        "Min Width / Height".into(),
                        vec!["w", "h"],
                    ))
                }
            },
            "max" => match get_class_name(args) {
                "w" => Self::MaxWidth(MaxWidth(get_args(args)?)),
                "h" => Self::MaxHeight(MaxHeight(get_args(args)?)),
                v => {
                    return Err(WarningType::InvalidArg(
                        v.into(),
                        "Max Width / Height".into(),
                        vec!["w", "h"],
                    ))
                }
            },
            _ => return Ok(None),
        };

        Ok(Some(sizing))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Width(s) => s.to_decl(),
            Self::MinWidth(s) => s.to_decl(),
            Self::MaxWidth(s) => s.to_decl(),
            Self::Height(s) => s.to_decl(),
            Self::MinHeight(s) => s.to_decl(),
            Self::MaxHeight(s) => s.to_decl(),
        }
    }
}
