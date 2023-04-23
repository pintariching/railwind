mod types;

pub use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Decl;
use crate::utils::get_args;
use crate::utils::{get_class_name, get_opt_args};
use crate::warning::WarningType;

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

#[derive(Debug, PartialEq, Hash)]
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
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let borders = match get_class_name(value) {
            "rounded" => Self::BorderRadius(BorderRadius::new(get_opt_args(value))),
            "border" => {
                if let Ok(args) = get_args(value) {
                    if let Some(style) = BorderStyle::new(args) {
                        Self::BorderStyle(style)
                    } else if let Some(border_color) = BorderColor::new(args) {
                        Self::BorderColor(border_color)
                    } else if let Some(width) = BorderWidth::new(args) {
                        Self::BorderWidth(width)
                    } else {
                        // TODO: Add warnings here
                        return Ok(None);
                    }
                } else {
                    Self::BorderWidth(BorderWidth::Around(""))
                }
            }
            "divide" => {
                let args = get_args(value)?;
                if let Some(style) = DivideStyle::new(args) {
                    Self::DivideStyle(style)
                } else if let Some(width) = DivideWidth::new(args) {
                    Self::DivideWidth(width)
                } else {
                    // TODO: Add warnings here
                    return Ok(None);
                }
            }
            "outline" => match get_class_name(get_opt_args(value)) {
                "offset" => Self::OutlineOffset(OutlineOffset(get_args(get_opt_args(value))?)),
                _ => {
                    if let Some(style) = OutlineStyle::new(get_opt_args(value)) {
                        Self::OutlineStyle(style)
                    } else if OUTLINE_WIDTH.contains_key(get_args(value)?) {
                        Self::OutlineWidth(OutlineWidth(get_args(value)?))
                    } else {
                        Self::OutlineColor(OutlineColor(get_args(value)?))
                    }
                }
            },
            "ring" => match get_class_name(get_opt_args(value)) {
                "offset" => {
                    let args = get_args(value)?;
                    if RING_OFFSET_WIDTH.contains_key(get_args(args)?) {
                        Self::RingOffsetWidth(RingOffsetWidth(get_args(args)?))
                    } else {
                        Self::RingOffsetColor(RingOffsetColor(get_args(args)?))
                    }
                }
                _ => {
                    if let Some(width) = RingWidth::new(get_opt_args(value)) {
                        Self::RingWidth(width)
                    } else {
                        Self::RingColor(RingColor(get_args(value)?))
                    }
                }
            },
            _ => return Ok(None),
        };

        Ok(Some(borders))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::BorderRadius(b) => b.to_decl(),
            Self::BorderWidth(b) => b.to_decl(),
            Self::BorderColor(b) => b.to_decl(),
            Self::BorderStyle(b) => Ok(b.to_decl()),
            Self::DivideWidth(b) => b.to_decl(),
            Self::DivideColor(b) => b.to_decl(),
            Self::DivideStyle(b) => Ok(b.to_decl()),
            Self::OutlineWidth(b) => b.to_decl(),
            Self::OutlineColor(b) => b.to_decl(),
            Self::OutlineStyle(b) => Ok(b.to_decl()),
            Self::OutlineOffset(b) => b.to_decl(),
            Self::RingWidth(b) => b.to_decl(),
            Self::RingColor(b) => b.to_decl(),
            Self::RingOffsetWidth(b) => b.to_decl(),
            Self::RingOffsetColor(b) => b.to_decl(),
        }
    }
}
