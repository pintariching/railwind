// use crate::class::utils::{get_value, get_value_neg};
use crate::class::utils::get_value;
use crate::class::Decl;
// use crate::utils::{get_args, get_class_name, get_opt_args};

use super::COLORS;

#[derive(Debug)]
pub struct Fill<'a>(pub &'a str);

impl<'a> Fill<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &COLORS)?;
        Some(Decl::Single(format!("fill: {}", value)))
    }
}

#[derive(Debug)]
pub struct Stroke<'a>(pub &'a str);

impl<'a> Stroke<'a> {
    pub fn new(arg: &'a str) -> Option<Self> {
        Some(Self(arg))
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &COLORS)?;
        Some(Decl::Single(format!("stroke: {}", value)))
    }
}

#[derive(Debug)]
pub struct StrokeWidth<'a>(pub &'a str);

impl<'a> StrokeWidth<'a> {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "0" => Self("0"),
            "1" => Self("1"),
            "2" => Self("2"),
            _ => return None,
        };
        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        Some(Decl::Single(format!("stroke-width: {}", self.0)))
    }
}
