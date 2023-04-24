use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::combinator::{map, map_opt};
use nom::sequence::{preceded, terminated};
use nom::IResult;

use crate::class::Decl;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::utils::{arbitrary, negative, pos_neg_val, pos_val, positive};
use super::IntoDeclaration;

lazy_static! {
    pub static ref MARGIN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("margin.ron")).unwrap();
    pub static ref PADDING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("padding.ron")).unwrap();
    pub static ref SPACE_BETWEEN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("space_between.ron")).unwrap();
}

#[derive(Debug, PartialEq, Hash)]
pub enum Spacing<'a> {
    Padding(Padding<'a>),
    Margin(Margin),
    SpaceBetween(SpaceBetween),
}

pub fn spacing(input: &str) -> IResult<&str, Spacing> {
    alt((
        map(padding, Spacing::Padding),
        map(margin, Spacing::Margin),
        map(space_between, Spacing::SpaceBetween),
    ))(input)
}

// impl<'a> Spacing<'a> {
//     pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
//         let class_name = get_class_name(value);
//         let args = if let Ok(str) = get_args(value) {
//             str
//         } else {
//             return Ok(None);
//         };

//         let spacing = if let Some(padding) = Padding::new(class_name, args) {
//             Self::Padding(padding)
//         } else if let Some(margin) = Margin::new(class_name, args) {
//             Self::Margin(margin)
//         } else {
//             if let Some(sb) = SpaceBetween::new(class_name, args)? {
//                 Self::SpaceBetween(sb)
//             } else {
//                 return Ok(None);
//             }
//         };

//         Ok(Some(spacing))
//     }

// pub fn to_decl(self) -> Result<Decl, WarningType> {
//     match self {
//         Self::Padding(s) => s.to_decl(),
//         Self::Margin(s) => s.to_decl(),
//         Self::SpaceBetween(s) => s.to_decl(),
//     }
// }
// }

#[derive(Debug, PartialEq, Hash)]
pub enum Padding<'a> {
    All(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
    X(&'a str),
    Y(&'a str),
}

fn padding(input: &str) -> IResult<&str, Padding> {
    alt((
        map(pos_val("p", &PADDING), Padding::All),
        map(pos_val("pt", &PADDING), Padding::Top),
        map(pos_val("pr", &PADDING), Padding::Right),
        map(pos_val("pb", &PADDING), Padding::Bottom),
        map(pos_val("pl", &PADDING), Padding::Left),
        map(pos_val("px", &PADDING), Padding::X),
        map(pos_val("py", &PADDING), Padding::Y),
    ))(input)
}

impl<'a> IntoDeclaration for Padding<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::All(p) => Decl::String(format!("padding: {}", p)),
            Self::Top(p) => Decl::String(format!("padding-top: {}", p)),
            Self::Right(p) => Decl::String(format!("padding-right: {}", p)),
            Self::Bottom(p) => Decl::String(format!("padding-bottom: {}", p)),
            Self::Left(p) => Decl::String(format!("padding-left: {}", p)),
            Self::X(p) => Decl::Double([
                format!("padding-left: {}", p),
                format!("padding-right: {}", p),
            ]),
            Self::Y(p) => Decl::Double([
                format!("padding-top: {}", p),
                format!("padding-bottom: {}", p),
            ]),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Margin {
    All(String),
    Top(String),
    Right(String),
    Bottom(String),
    Left(String),
    X(String),
    Y(String),
}

fn margin(input: &str) -> IResult<&str, Margin> {
    alt((
        map(pos_neg_val("m", &MARGIN), Margin::All),
        map(pos_neg_val("mt", &MARGIN), Margin::Top),
        map(pos_neg_val("mr", &MARGIN), Margin::Right),
        map(pos_neg_val("mb", &MARGIN), Margin::Bottom),
        map(pos_neg_val("ml", &MARGIN), Margin::Left),
        map(pos_neg_val("mx", &MARGIN), Margin::X),
        map(pos_neg_val("my", &MARGIN), Margin::Y),
    ))(input)
}

impl IntoDeclaration for Margin {
    fn to_decl(self) -> Decl {
        match self {
            Self::All(m) => Decl::String(format!("margin: {}", m)),
            Self::Top(m) => Decl::String(format!("margin-top: {}", m)),
            Self::Right(m) => Decl::String(format!("margin-right: {}", m)),
            Self::Bottom(m) => Decl::String(format!("margin-bottom: {}", m)),
            Self::Left(m) => Decl::String(format!("margin-left: {}", m)),
            Self::X(m) => Decl::Double([
                format!("margin-left: {}", m),
                format!("margin-right: {}", m),
            ]),
            Self::Y(m) => Decl::Double([
                format!("margin-top: {}", m),
                format!("margin-bottom: {}", m),
            ]),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum SpaceBetween {
    X(String),
    Y(String),
    ReverseX,
    ReverseY,
}

fn space_between(input: &str) -> IResult<&str, SpaceBetween> {
    alt((
        map(pos_neg_val("space-x", &SPACE_BETWEEN), SpaceBetween::X),
        map(pos_neg_val("space-y", &SPACE_BETWEEN), SpaceBetween::Y),
        map(tag("space-x-reverse"), |_| SpaceBetween::ReverseX),
        map(tag("space-y-reverse"), |_| SpaceBetween::ReverseY),
    ))(input)
}

impl IntoDeclaration for SpaceBetween {
    fn to_decl(self) -> Decl {
        match self {
            SpaceBetween::X(s) => Decl::Vec(vec![
                "--tw-space-x-reverse: 0".into(),
                format!("margin-right: calc({} * var(--tw-space-x-reverse))", s),
                format!(
                    "margin-left: calc({} * calc(1 - var(--tw-space-x-reverse)))",
                    s
                ),
            ]),
            SpaceBetween::Y(s) => Decl::Vec(vec![
                "--tw-space-y-reverse: 0".into(),
                format!(
                    "margin-top: calc({} * calc(1 - var(--tw-space-y-reverse)))",
                    s
                ),
                format!("margin-bottom: calc({} * var(--tw-space-y-reverse))", s),
            ]),
            SpaceBetween::ReverseX => Decl::Lit("--tw-space-x-reverse: 1"),
            SpaceBetween::ReverseY => Decl::Lit("--tw-space-y-reverse: 1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        assert_eq!(padding("p-5"), Ok(("", Padding::All("1.25rem"))));
        assert_eq!(padding("p-[3.251rem]"), Ok(("", Padding::All("3.251rem"))));
    }

    #[test]
    fn test_margin() {
        assert_eq!(margin("m-5"), Ok(("", Margin::All("1.25rem".to_string()))));
        assert_eq!(
            margin("-m-5"),
            Ok(("", Margin::All("-1.25rem".to_string())))
        );
        assert_eq!(
            margin("m-[3.14px]"),
            Ok(("", Margin::All("3.14px".to_string())))
        );
        assert_eq!(
            margin("-m-[3.14px]"),
            Ok(("", Margin::All("-3.14px".to_string())))
        );
    }

    #[test]
    fn test_space_between() {
        assert_eq!(
            space_between("space-x-5"),
            Ok(("", SpaceBetween::X("1.25rem".to_string())))
        );
        assert_eq!(
            space_between("-space-x-5"),
            Ok(("", SpaceBetween::X("-1.25rem".to_string())))
        );
        assert_eq!(
            space_between("space-x-[42rem]"),
            Ok(("", SpaceBetween::X("42rem".to_string())))
        );
        assert_eq!(
            space_between("space-x-[-42rem]"),
            Ok(("", SpaceBetween::X("-42rem".to_string())))
        );
        assert_eq!(
            space_between("space-x-reverse"),
            Ok(("", SpaceBetween::ReverseX))
        );
        assert_eq!(
            space_between("space-y-reverse"),
            Ok(("", SpaceBetween::ReverseY))
        );
    }
}
