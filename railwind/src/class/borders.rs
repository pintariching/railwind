use macro_derive::{ConfigurableParser, EnumParser, IntoDeclaration};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::class::colors::hex_color;
use crate::class::utils::{arbitrary_hashmap_value, keyword_value};
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
            tag("rounded-"),
            map(|i| border_radius(i, config), Borders::BorderRadius),
        ),
        map(tag("rounded"), |_| {
            Borders::BorderRadius(BorderRadius::Around(""))
        }),
        preceded(
            tag("border-"),
            alt((
                map(|i| border_width(i, config), Borders::BorderWidth),
                map(|i| border_color(i, config), Borders::BorderColor),
                map(border_style, Borders::BorderStyle),
            )),
        ),
        map(tag("border"), |_| {
            Borders::BorderWidth(BorderWidth::Around(""))
        }),
        preceded(
            tag("divide-"),
            alt((
                map(|i| divide_width(i, config), Borders::DivideWidth),
                map(|i| divide_color(i, config), Borders::DivideColor),
                map(divide_style, Borders::DivideStyle),
            )),
        ),
        preceded(
            tag("outline-"),
            alt((
                map(|i| outline_width(i, config), Borders::OutlineWidth),
                map(|i| outline_color(i, config), Borders::OutlineColor),
                map(outline_style, Borders::OutlineStyle),
                map(|i| outline_offset(i, config), Borders::OutlineOffset),
            )),
        ),
        map(tag("outline"), |_| {
            Borders::OutlineStyle(OutlineStyle::Solid)
        }),
        preceded(
            tag("ring-"),
            alt((
                map(|i| ring_width(i, config), Borders::RingWidth),
                map(|i| ring_color(i, config), Borders::RingColor),
                preceded(
                    tag("offset-"),
                    alt((
                        map(|i| ring_offset_width(i, config), Borders::RingOffsetWidth),
                        map(|i| ring_offset_color(i, config), Borders::RingOffsetColor),
                    )),
                ),
            )),
        ),
        map(tag("ring"), |_| Borders::RingWidth(RingWidth::Value(""))),
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
        map(tag("x"), |_| DivideWidth::X("")),
        map(tag("y"), |_| DivideWidth::Y("")),
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

fn ring_width<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, RingWidth<'a>> {
    let w = || config.borders.get_ring_width();

    alt((
        map(arbitrary_hashmap_value(w), RingWidth::Value),
        map(tag("inset"), |_| RingWidth::Inset),
    ))(input)
}

impl<'a> IntoDeclaration for RingWidth<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::Value(w) => Decl::String(format!("box-shadow: {}", w)),
            Self::Inset => Decl::Lit("--tw-ring-inset: inset"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(ring_color)]
#[config(borders.get_ring_color)]
#[decl("--tw-ring-color")]
pub struct RingColor<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, ConfigurableParser)]
#[name(ring_offset_width)]
#[config(borders.get_ring_offset_width)]
pub struct RingOffsetWidth<'a>(pub &'a str);

impl<'a> IntoDeclaration for RingOffsetWidth<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-ring-offset-width: {}", self.0),
            "box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)".into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser)]
#[name(ring_offset_color)]
#[config(borders.get_ring_offset_color)]
pub struct RingOffsetColor<'a>(pub &'a str);

impl<'a> IntoDeclaration for RingOffsetColor<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-ring-offset-color: {}", self.0),
            "box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)".into(),
        ])
    }
}
