mod types;

pub use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Decl;
use crate::utils::get_args;
use crate::utils::{get_class_name, get_opt_args};

lazy_static! {
    pub static ref BORDER_RADIUS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("border_radius.ron")).unwrap();
    pub static ref BORDER_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("border_width.ron")).unwrap();
    pub static ref BORDER_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref DIVIDE_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("divide_width.ron")).unwrap();
    pub static ref DIVIDE_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref OUTLINE_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("outline_width.ron")).unwrap();
    pub static ref OUTLINE_OFFSET: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("border_width.ron")).unwrap();
    pub static ref RING_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("ring_width.ron")).unwrap();
    pub static ref RING_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref RING_OFFSET_WIDTH: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("outline_width.ron")).unwrap();
    pub static ref RING_OFFSET_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
}

#[derive(Debug)]
pub enum Borders<'a> {
    BorderRadius(BorderRadius<'a>),
    BorderWidth(BorderWidth<'a>),
    BorderColor(BorderColor<'a>),
    BorderStyle(BorderStyle),
    DivideWidth(DivideWidth<'a>),
    DivideColor(DivideColor<'a>),
    DivideStyle(DivideStyle),
    OutlineWidth(OutlineWidth<'a>),
    OutlineColor(OutlineColor<'a>),
    OutlineStyle(OutlineStyle),
    OutlineOffset(OutlineOffset<'a>),
    RingWidth(RingWidth<'a>),
    RingColor(RingColor<'a>),
    RingOffsetWidth(RingOffsetWidth<'a>),
    RingOffsetColor(RingOffsetColor<'a>),
}

impl<'a> Borders<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let borders = match get_class_name(value) {
            "rounded" => Borders::BorderRadius(BorderRadius::new(get_opt_args(value))?),
            "border" => {
                if let Some(args) = get_args(value) {
                    if let Some(style) = BorderStyle::new(args) {
                        Borders::BorderStyle(style)
                    } else if let Some(border_color) = BorderColor::new(args) {
                        Borders::BorderColor(border_color)
                    } else if let Some(width) = BorderWidth::new(args) {
                        Borders::BorderWidth(width)
                    } else {
                        return None;
                    }
                } else {
                    Borders::BorderWidth(BorderWidth::new("border")?)
                }
            }
            "divide" => {
                let args = get_args(value)?;
                if let Some(style) = DivideStyle::new(args) {
                    Borders::DivideStyle(style)
                } else if let Some(width) = DivideWidth::new(args) {
                    Borders::DivideWidth(width)
                } else {
                    return None;
                }
            }
            "outline" => match get_class_name(get_opt_args(value)) {
                "offset" => Borders::OutlineOffset(OutlineOffset(get_args(get_opt_args(value))?)),
                _ => {
                    if let Some(style) = OutlineStyle::new(get_opt_args(value)) {
                        Borders::OutlineStyle(style)
                    } else if OUTLINE_WIDTH.contains_key(get_args(value)?) {
                        Borders::OutlineWidth(OutlineWidth(get_args(value)?))
                    } else {
                        Borders::OutlineColor(OutlineColor(get_args(value)?))
                    }
                }
            },
            "ring" => match get_class_name(get_opt_args(value)) {
                "offset" => {
                    let args = get_args(value)?;
                    if RING_OFFSET_WIDTH.contains_key(get_args(args)?) {
                        Borders::RingOffsetWidth(RingOffsetWidth(get_args(args)?))
                    } else {
                        Borders::RingOffsetColor(RingOffsetColor(get_args(args)?))
                    }
                }
                _ => {
                    if let Some(width) = RingWidth::new(get_opt_args(value)) {
                        Borders::RingWidth(width)
                    } else {
                        Borders::RingColor(RingColor(get_args(value)?))
                    }
                }
            },
            _ => return None,
        };

        Some(borders)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Borders::BorderRadius(b) => b.to_decl(),
            Borders::BorderWidth(b) => b.to_decl(),
            Borders::BorderColor(b) => b.to_decl(),
            Borders::BorderStyle(b) => Some(b.to_decl()),
            Borders::DivideWidth(b) => b.to_decl(),
            Borders::DivideColor(b) => b.to_decl(),
            Borders::DivideStyle(b) => Some(b.to_decl()),
            Borders::OutlineWidth(b) => b.to_decl(),
            Borders::OutlineColor(b) => b.to_decl(),
            Borders::OutlineStyle(b) => Some(b.to_decl()),
            Borders::OutlineOffset(b) => b.to_decl(),
            Borders::RingWidth(b) => b.to_decl(),
            Borders::RingColor(b) => b.to_decl(),
            Borders::RingOffsetWidth(b) => b.to_decl(),
            Borders::RingOffsetColor(b) => b.to_decl(),
        }
    }
}
