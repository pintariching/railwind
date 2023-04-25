use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use std::collections::HashMap;

use crate::class::utils::{keyword_value, pos_neg_keyword_value};
use crate::class::Decl;
use crate::class::IntoDeclaration;

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

impl<'a> IntoDeclaration for Spacing<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Spacing::Padding(s) => s.to_decl(),
            Spacing::Margin(s) => s.to_decl(),
            Spacing::SpaceBetween(s) => s.to_decl(),
        }
    }
}

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
        map(keyword_value("p", &PADDING), Padding::All),
        map(keyword_value("pt", &PADDING), Padding::Top),
        map(keyword_value("pr", &PADDING), Padding::Right),
        map(keyword_value("pb", &PADDING), Padding::Bottom),
        map(keyword_value("pl", &PADDING), Padding::Left),
        map(keyword_value("px", &PADDING), Padding::X),
        map(keyword_value("py", &PADDING), Padding::Y),
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
        map(pos_neg_keyword_value("m", &MARGIN), Margin::All),
        map(pos_neg_keyword_value("mt", &MARGIN), Margin::Top),
        map(pos_neg_keyword_value("mr", &MARGIN), Margin::Right),
        map(pos_neg_keyword_value("mb", &MARGIN), Margin::Bottom),
        map(pos_neg_keyword_value("ml", &MARGIN), Margin::Left),
        map(pos_neg_keyword_value("mx", &MARGIN), Margin::X),
        map(pos_neg_keyword_value("my", &MARGIN), Margin::Y),
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
        map(
            pos_neg_keyword_value("space-x", &SPACE_BETWEEN),
            SpaceBetween::X,
        ),
        map(
            pos_neg_keyword_value("space-y", &SPACE_BETWEEN),
            SpaceBetween::Y,
        ),
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
