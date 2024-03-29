use macro_derive::ConfigurableEnumParser;
use nom::branch::alt;
use nom::IResult;
use nom::{bytes::complete::tag, combinator::map};

use crate::class::utils::{keyword_value, neg_keyword_value};
use crate::class::Decl;
use crate::class::IntoDeclaration;
use crate::config::Config;

#[derive(Debug, PartialEq, Hash)]
pub enum Spacing<'a> {
    Padding(Padding<'a>),
    Margin(Margin),
    SpaceBetween(SpaceBetween),
}

pub fn spacing<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Spacing<'a>> {
    alt((
        map(|i| padding(i, config), Spacing::Padding),
        map(|i| margin(i, config), Spacing::Margin),
        map(|i| space_between(i, config), Spacing::SpaceBetween),
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

#[derive(Debug, PartialEq, Hash, ConfigurableEnumParser)]
#[name(padding)]
#[config(spacing.get_padding)]
pub enum Padding<'a> {
    #[tag("p")]
    All(&'a str),
    #[tag("pt")]
    Top(&'a str),
    #[tag("pr")]
    Right(&'a str),
    #[tag("pb")]
    Bottom(&'a str),
    #[tag("pl")]
    Left(&'a str),
    #[tag("px")]
    X(&'a str),
    #[tag("py")]
    Y(&'a str),
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

fn margin<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Margin> {
    let margin = config.spacing.get_margin();

    alt((
        map(neg_keyword_value("m", margin), Margin::All),
        map(neg_keyword_value("mt", margin), Margin::Top),
        map(neg_keyword_value("mr", margin), Margin::Right),
        map(neg_keyword_value("mb", margin), Margin::Bottom),
        map(neg_keyword_value("ml", margin), Margin::Left),
        map(neg_keyword_value("mx", margin), Margin::X),
        map(neg_keyword_value("my", margin), Margin::Y),
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

fn space_between<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, SpaceBetween> {
    let space_between = config.spacing.get_space_between();

    alt((
        map(neg_keyword_value("space-x", space_between), SpaceBetween::X),
        map(neg_keyword_value("space-y", space_between), SpaceBetween::Y),
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
        assert_eq!(
            padding("p-5", &Config::default()),
            Ok(("", Padding::All("1.25rem")))
        );
        assert_eq!(
            padding("p-[3.251rem]", &Config::default()),
            Ok(("", Padding::All("3.251rem")))
        );
    }

    #[test]
    fn test_margin() {
        assert_eq!(
            margin("m-5", &Config::default()),
            Ok(("", Margin::All("1.25rem".to_string())))
        );
        assert_eq!(
            margin("-m-5", &Config::default()),
            Ok(("", Margin::All("-1.25rem".to_string())))
        );
        assert_eq!(
            margin("m-[3.14px]", &Config::default()),
            Ok(("", Margin::All("3.14px".to_string())))
        );
        assert_eq!(
            margin("-m-[3.14px]", &Config::default()),
            Ok(("", Margin::All("-3.14px".to_string())))
        );
    }

    #[test]
    fn test_space_between() {
        assert_eq!(
            space_between("space-x-5", &Config::default()),
            Ok(("", SpaceBetween::X("1.25rem".to_string())))
        );
        assert_eq!(
            space_between("-space-x-5", &Config::default()),
            Ok(("", SpaceBetween::X("-1.25rem".to_string())))
        );
        assert_eq!(
            space_between("space-x-[42rem]", &Config::default()),
            Ok(("", SpaceBetween::X("42rem".to_string())))
        );
        assert_eq!(
            space_between("space-x-[-42rem]", &Config::default()),
            Ok(("", SpaceBetween::X("-42rem".to_string())))
        );
        assert_eq!(
            space_between("space-x-reverse", &Config::default()),
            Ok(("", SpaceBetween::ReverseX))
        );
        assert_eq!(
            space_between("space-y-reverse", &Config::default()),
            Ok(("", SpaceBetween::ReverseY))
        );
    }
}
