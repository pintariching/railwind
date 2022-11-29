mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

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
    pub fn new(value: &'a str) -> Option<Self> {
        let args = get_args(value)?;

        let sizing = match get_class_name(value) {
            "w" => Sizing::Width(Width(args)),
            "h" => Sizing::Height(Height(args)),
            "min" => match get_class_name(args) {
                "w" => Sizing::MinWidth(MinWidth(get_args(args)?)),
                "h" => Sizing::MinHeight(MinHeight(get_args(args)?)),
                _ => return None,
            },
            "max" => match get_class_name(args) {
                "w" => Sizing::MaxWidth(MaxWidth(get_args(args)?)),
                "h" => Sizing::MaxHeight(MaxHeight(get_args(args)?)),
                _ => return None,
            },
            _ => return None,
        };

        Some(sizing)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Sizing::Width(s) => s.to_decl(),
            Sizing::MinWidth(s) => s.to_decl(),
            Sizing::MaxWidth(s) => s.to_decl(),
            Sizing::Height(s) => s.to_decl(),
            Sizing::MinHeight(s) => s.to_decl(),
            Sizing::MaxHeight(s) => s.to_decl(),
        }
    }
}
