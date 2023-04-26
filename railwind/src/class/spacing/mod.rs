use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::{bytes::complete::tag, combinator::map};
use nom::{combinator::map_opt, sequence::preceded};

use crate::class::utils::{arbitrary, keyword_dash, negative_keyword_dash};
use crate::class::{Decl, IntoDeclaration};
use crate::CONFIG;

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
    let p = |keyword| {
        preceded(
            keyword_dash(keyword),
            alt((
                arbitrary,
                map_opt(is_not(" "), |value| {
                    CONFIG.lock().unwrap().spacing.padding.get(value).copied()
                }),
            )),
        )
    };

    alt((
        map(p("p"), Padding::All),
        map(p("pt"), Padding::Top),
        map(p("pr"), Padding::Right),
        map(p("pb"), Padding::Bottom),
        map(p("pl"), Padding::Left),
        map(p("px"), Padding::X),
        map(p("py"), Padding::Y),
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
    let m = |keyword| {
        alt((
            preceded(
                keyword_dash(keyword),
                alt((
                    map(arbitrary, String::from),
                    map_opt(is_not(" "), |value| {
                        CONFIG
                            .lock()
                            .unwrap()
                            .spacing
                            .margin
                            .get(value)
                            .map(|m| m.to_string())
                    }),
                )),
            ),
            preceded(
                negative_keyword_dash(keyword),
                alt((
                    map(arbitrary, |m| format!("-{m}")),
                    map_opt(is_not(" "), |value| {
                        CONFIG
                            .lock()
                            .unwrap()
                            .spacing
                            .margin
                            .get(value)
                            .map(|m| format!("-{m}"))
                    }),
                )),
            ),
        ))
    };

    alt((
        map(m("m"), Margin::All),
        map(m("mt"), Margin::Top),
        map(m("mr"), Margin::Right),
        map(m("mb"), Margin::Bottom),
        map(m("ml"), Margin::Left),
        map(m("mx"), Margin::X),
        map(m("my"), Margin::Y),
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
    let s = |keyword| {
        alt((
            preceded(
                keyword_dash(keyword),
                alt((
                    map(arbitrary, String::from),
                    map_opt(is_not(" "), |value| {
                        CONFIG
                            .lock()
                            .unwrap()
                            .spacing
                            .space_between
                            .get(value)
                            .map(|s| s.to_string())
                    }),
                )),
            ),
            preceded(
                negative_keyword_dash(keyword),
                alt((
                    map(arbitrary, |s| format!("-{s}")),
                    map_opt(is_not(" "), |value| {
                        CONFIG
                            .lock()
                            .unwrap()
                            .spacing
                            .padding
                            .get(value)
                            .map(|s| format!("-{s}"))
                    }),
                )),
            ),
        ))
    };

    alt((
        map(s("space-x"), SpaceBetween::X),
        map(s("space-y"), SpaceBetween::Y),
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
