use macro_derive::{ConfigurableParser, EnumParser, IntoDeclaration};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::class::colors::hex_color;
use crate::class::utils::{arbitrary_hashmap_value, keyword_dash, keyword_value};
use crate::class::{Decl, IntoDeclaration};
use crate::config::Config;

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

pub fn borders<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Borders<'a>> {
    alt((
        preceded(
            keyword_dash("rounded"),
            map(|i| border_radius(i, config), Borders::BorderRadius),
        ),
        preceded(
            keyword_dash("border"),
            alt((
                //map(|i| border_style(i, config), Borders::BorderStyle),
                map(|i| border_color(i, config), Borders::BorderColor),
                map(|i| border_width(i, config), Borders::BorderWidth),
            )),
        ),
    ))(input)
}

#[derive(Debug, PartialEq, Hash)]
pub enum BorderRadius<'a> {
    Around(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
    TopLeft(&'a str),
    TopRight(&'a str),
    BottomRight(&'a str),
    BottomLeft(&'a str),
}

fn border_radius<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BorderRadius<'a>> {
    let radius = || config.borders.get_border_radius();

    alt((
        map(keyword_value("t", radius), BorderRadius::Top),
        map(keyword_value("r", radius), BorderRadius::Right),
        map(keyword_value("b", radius), BorderRadius::Right),
        map(keyword_value("l", radius), BorderRadius::Right),
        map(keyword_value("tl", radius), BorderRadius::Right),
        map(keyword_value("tr", radius), BorderRadius::Right),
        map(keyword_value("br", radius), BorderRadius::Right),
        map(keyword_value("bl", radius), BorderRadius::Right),
        map(arbitrary_hashmap_value(radius), BorderRadius::Around),
    ))(input)
}

impl<'a> IntoDeclaration for BorderRadius<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::Around(r) => Decl::String(format!("border-radius: {}", r)),
            Self::Top(r) => Decl::Vec(vec![
                format!("border-top-left-radius: {}", r),
                format!("border-top-right-radius: {}", r),
            ]),
            Self::Right(r) => Decl::Vec(vec![
                format!("border-top-right-radius: {}", r),
                format!("border-bottom-right-radius: {}", r),
            ]),
            Self::Bottom(r) => Decl::Vec(vec![
                format!("border-bottom-right-radius: {}", r),
                format!("border-bottom-left-radius: {}", r),
            ]),
            Self::Left(r) => Decl::Vec(vec![
                format!("border-top-left-radius: {}", r),
                format!("border-bottom-left-radius: {}", r),
            ]),
            Self::TopLeft(r) => Decl::String(format!("border-top-left-radius: {}", r)),
            Self::TopRight(r) => Decl::String(format!("border-top-right-radius: {}", r)),
            Self::BottomRight(r) => Decl::String(format!("border-bottom-right-radius: {}", r)),
            Self::BottomLeft(r) => Decl::String(format!("border-bottom-left-radius: {}", r)),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum BorderWidth<'a> {
    Around(&'a str),
    X(&'a str),
    Y(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
}

fn border_width<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BorderWidth<'a>> {
    let width = || config.borders.get_border_width();

    alt((
        map(keyword_value("x", width), BorderWidth::X),
        map(keyword_value("y", width), BorderWidth::Y),
        map(keyword_value("t", width), BorderWidth::Top),
        map(keyword_value("r", width), BorderWidth::Right),
        map(keyword_value("b", width), BorderWidth::Bottom),
        map(keyword_value("l", width), BorderWidth::Left),
        map(arbitrary_hashmap_value(width), BorderWidth::Around),
    ))(input)
}

impl<'a> IntoDeclaration for BorderWidth<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::Around(w) => Decl::String(format!("border-width: {}", w)),
            Self::X(w) => Decl::Vec(vec![
                format!("border-left-width: {}", w),
                format!("border-right-width: {}", w),
            ]),
            Self::Y(w) => Decl::Vec(vec![
                format!("border-top-width: {}", w),
                format!("border-bottom-width: {}", w),
            ]),
            Self::Top(w) => Decl::String(format!("border-top-width: {}", w)),
            Self::Right(w) => Decl::String(format!("border-right-width: {}", w)),
            Self::Bottom(w) => Decl::String(format!("border-bottom-width: {}", w)),
            Self::Left(w) => Decl::String(format!("border-left-width: {}", w)),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum BorderColor<'a> {
    Around(&'a str),
    X(&'a str),
    Y(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
}

fn border_color<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BorderColor<'a>> {
    let color = || config.borders.get_border_color();

    alt((
        map(keyword_value("x", color), BorderColor::X),
        map(keyword_value("y", color), BorderColor::Y),
        map(keyword_value("t", color), BorderColor::Top),
        map(keyword_value("r", color), BorderColor::Right),
        map(keyword_value("b", color), BorderColor::Bottom),
        map(keyword_value("l", color), BorderColor::Left),
        map(arbitrary_hashmap_value(color), BorderColor::Around),
    ))(input)
}

impl<'a> IntoDeclaration for BorderColor<'a> {
    fn to_decl(self) -> Decl {
        let value = match self {
            BorderColor::Around(v) => v,
            BorderColor::X(v) => v,
            BorderColor::Y(v) => v,
            BorderColor::Top(v) => v,
            BorderColor::Right(v) => v,
            BorderColor::Bottom(v) => v,
            BorderColor::Left(v) => v,
        };

        if let Ok((_, color)) = hex_color(&value) {
            let decl = match self {
                Self::Around(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
                Self::X(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-left-color: rgb({color} / var(--tw-border-opacity))",),
                    format!("border-right-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
                Self::Y(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-top-color: rgb({color} / var(--tw-border-opacity))",),
                    format!("border-bottom-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
                Self::Top(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-top-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
                Self::Right(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-right-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
                Self::Bottom(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-bottom-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
                Self::Left(_) => Decl::Vec(vec![
                    "--tw-border-opacity: 1".into(),
                    format!("border-left-color: rgb({color} / var(--tw-border-opacity))",),
                ]),
            };

            decl
        } else {
            match self {
                Self::Around(_) => Decl::String(format!("border-color: {}", value)),
                Self::X(_) => Decl::Vec(vec![
                    format!("border-left-color: {}", value),
                    format!("border-right-color: {}", value),
                ]),
                Self::Y(_) => Decl::Vec(vec![
                    format!("border-top-color: {}", value),
                    format!("border-bottom-color: {}", value),
                ]),
                Self::Top(_) => Decl::String(format!("border-top-color: {}", value)),
                Self::Right(_) => Decl::String(format!("border-right-color: {}", value)),
                Self::Bottom(_) => Decl::String(format!("border-bottom-color: {}", value)),
                Self::Left(_) => Decl::String(format!("border-left-color: {}", value)),
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(border_style)]
#[decl("border-style")]
pub enum BorderStyle {
    #[tag("solid")]
    Solid,
    #[tag("dashed")]
    Dashed,
    #[tag("dotted")]
    Dotted,
    #[tag("double")]
    Double,
    #[tag("hidden")]
    Hidden,
    #[tag("none")]
    None,
}

#[derive(Debug, PartialEq, Hash)]
pub enum DivideWidth<'a> {
    X(&'a str),
    Y(&'a str),
    ReverseX,
    ReverseY,
}

fn divide_width<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, DivideWidth<'a>> {
    let width = || config.borders.get_divide_width();

    alt((
        map(keyword_value("x", width), DivideWidth::X),
        map(keyword_value("y", width), DivideWidth::Y),
        map(tag("x-reverse"), |_| DivideWidth::ReverseX),
        map(tag("y-reverse"), |_| DivideWidth::ReverseY),
    ))(input)
}

impl<'a> IntoDeclaration for DivideWidth<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::X(w) => Decl::Vec(vec![
                "--tw-divide-x-reverse: 0".into(),
                format!(
                    "border-right-width: calc({} * var(--tw-divide-x-reverse))",
                    w
                ),
                format!(
                    "border-left-width: calc({} * calc(1 - var(--tw-divide-x-reverse)))",
                    w
                ),
            ]),
            Self::Y(w) => Decl::Vec(vec![
                "--tw-divide-y-reverse: 0".into(),
                format!(
                    "border-top-width: calc({} * calc(1 - var(--tw-divide-y-reverse)))",
                    w
                ),
                format!(
                    "border-bottom-width: calc({} * var(--tw-divide-y-reverse))",
                    w
                ),
            ]),
            Self::ReverseX => Decl::Lit("--tw-divide-x-reverse: 1"),
            Self::ReverseY => Decl::Lit("--tw-divide-y-reverse: 1"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(divide_color)]
#[config(borders.get_divide_color)]
#[decl("border-color")]
pub struct DivideColor<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(divide_style)]
#[decl("border-style")]
pub enum DivideStyle {
    #[tag("solid")]
    Solid,
    #[tag("dashed")]
    Dashed,
    #[tag("dotted")]
    Dotted,
    #[tag("double")]
    Double,
    #[tag("none")]
    None,
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(outline_width)]
#[config(borders.get_outline_width)]
#[decl("outline-width")]
pub struct OutlineWidth<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(outline_color)]
#[config(borders.get_outline_color)]
#[decl("outline-color")]
pub struct OutlineColor<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, EnumParser)]
#[name(outline_style)]
pub enum OutlineStyle {
    #[tag("none")]
    None,
    #[tag("outline")]
    Solid,
    #[tag("dashed")]
    Dashed,
    #[tag("dotted")]
    Dotted,
    #[tag("double")]
    Double,
}

impl IntoDeclaration for OutlineStyle {
    fn to_decl(self) -> Decl {
        let val = match self {
            Self::None => {
                return Decl::Vec(vec![
                    "outline: 2px solid transparent".into(),
                    "outline-offset: 2px".into(),
                ])
            }
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
        };

        Decl::String(format!("border-style: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(outline_offset)]
#[config(borders.get_outline_offset)]
#[decl("outline-offset")]
pub struct OutlineOffset<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash)]
pub enum RingWidth<'a> {
    Value(&'a str),
    Inset,
}

#[derive(Debug, PartialEq, Hash)]
pub struct RingColor<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash)]
pub struct RingOffsetWidth<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash)]
pub struct RingOffsetColor<'a>(pub &'a str);

// pub fn borders<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Borders<'a>> {
//     todo!()
// }

// // impl<'a> Borders<'a> {
// //     pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
// //         let borders = match get_class_name(value) {
// //             "rounded" => Self::BorderRadius(BorderRadius::new(get_opt_args(value))),
// //             "border" => {
// //                 if let Ok(args) = get_args(value) {
// //                     if let Some(style) = BorderStyle::new(args) {
// //                         Self::BorderStyle(style)
// //                     } else if let Some(border_color) = BorderColor::new(args) {
// //                         Self::BorderColor(border_color)
// //                     } else if let Some(width) = BorderWidth::new(args) {
// //                         Self::BorderWidth(width)
// //                     } else {
// //                         // TODO: Add warnings here
// //                         return Ok(None);
// //                     }
// //                 } else {
// //                     Self::BorderWidth(BorderWidth::Around(""))
// //                 }
// //             }
// //             "divide" => {
// //                 let args = get_args(value)?;
// //                 if let Some(style) = DivideStyle::new(args) {
// //                     Self::DivideStyle(style)
// //                 } else if let Some(width) = DivideWidth::new(args) {
// //                     Self::DivideWidth(width)
// //                 } else {
// //                     // TODO: Add warnings here
// //                     return Ok(None);
// //                 }
// //             }
// //             "outline" => match get_class_name(get_opt_args(value)) {
// //                 "offset" => Self::OutlineOffset(OutlineOffset(get_args(get_opt_args(value))?)),
// //                 _ => {
// //                     if let Some(style) = OutlineStyle::new(get_opt_args(value)) {
// //                         Self::OutlineStyle(style)
// //                     } else if OUTLINE_WIDTH.contains_key(get_args(value)?) {
// //                         Self::OutlineWidth(OutlineWidth(get_args(value)?))
// //                     } else {
// //                         Self::OutlineColor(OutlineColor(get_args(value)?))
// //                     }
// //                 }
// //             },
// //             "ring" => match get_class_name(get_opt_args(value)) {
// //                 "offset" => {
// //                     let args = get_args(value)?;
// //                     if RING_OFFSET_WIDTH.contains_key(get_args(args)?) {
// //                         Self::RingOffsetWidth(RingOffsetWidth(get_args(args)?))
// //                     } else {
// //                         Self::RingOffsetColor(RingOffsetColor(get_args(args)?))
// //                     }
// //                 }
// //                 _ => {
// //                     if let Some(width) = RingWidth::new(get_opt_args(value)) {
// //                         Self::RingWidth(width)
// //                     } else {
// //                         Self::RingColor(RingColor(get_args(value)?))
// //                     }
// //                 }
// //             },
// //             _ => return Ok(None),
// //         };

// //         Ok(Some(borders))
// //     }

// //     pub fn to_decl(self) -> Result<Decl, WarningType> {
// //         match self {
// //             Self::BorderRadius(b) => b.to_decl(),
// //             Self::BorderWidth(b) => b.to_decl(),
// //             Self::BorderColor(b) => b.to_decl(),
// //             Self::BorderStyle(b) => Ok(b.to_decl()),
// //             Self::DivideWidth(b) => b.to_decl(),
// //             Self::DivideColor(b) => b.to_decl(),
// //             Self::DivideStyle(b) => Ok(b.to_decl()),
// //             Self::OutlineWidth(b) => b.to_decl(),
// //             Self::OutlineColor(b) => b.to_decl(),
// //             Self::OutlineStyle(b) => Ok(b.to_decl()),
// //             Self::OutlineOffset(b) => b.to_decl(),
// //             Self::RingWidth(b) => b.to_decl(),
// //             Self::RingColor(b) => b.to_decl(),
// //             Self::RingOffsetWidth(b) => b.to_decl(),
// //             Self::RingOffsetColor(b) => b.to_decl(),
// //         }
// //     }
// // }
