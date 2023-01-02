mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COLORS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
}

#[derive(Debug)]
pub enum Svg<'a> {
    Fill(Fill<'a>),
    Stroke(Stroke<'a>),
    StrokeWidth(StrokeWidth<'a>),
}

impl<'a> Svg<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let args = get_args(value)?;
        let class_name = get_class_name(value);

        let svg = match class_name {
            "fill" => Svg::Fill(Fill(args)),
            "stroke" => {
                if let Some(stroke) = StrokeWidth::new(args) {
                    Svg::StrokeWidth(stroke)
                } else if let Some(stroke) = Stroke::new(args) {
                    Svg::Stroke(stroke)
                } else {
                    return None;
                }
            }
            _ => return None,
        };
        Some(svg)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Svg::Fill(s) => s.to_decl(),
            Svg::Stroke(s) => s.to_decl(),
            Svg::StrokeWidth(s) => s.to_decl(),
        }
    }
}
