use indexmap::IndexSet;
use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{multispace1, space1};
use nom::combinator::{map, map_opt};
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use std::collections::HashMap;

use crate::class::Class;

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

fn class_attr(input: &str) -> IResult<&str, IndexSet<Class>> {
    map(
        delimited(
            preceded(take_until("class=\""), tag("class=\"")),
            classes,
            tag("\""),
        ),
        |classes| classes.into_iter().filter_map(|c| Class::new(c)).collect(),
    )(input)
}

fn classes(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(multispace1, is_not("\"\n\t "))(input)
}

lazy_static! {
    pub static ref PADDING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("class/spacing/padding.ron")).unwrap();
}

fn arbitrary(input: &str) -> IResult<&str, &str> {
    delimited(tag("["), is_not("]"), tag("]"))(input)
}

fn p<'a>(
    keyword: &'a str,
) -> impl FnMut(&'a str) -> Result<(&str, &str), nom::Err<nom::error::Error<&str>>> {
    preceded(
        terminated(tag(keyword), tag("-")),
        alt((arbitrary, map_opt(is_not(" "), |v| PADDING.get(v).copied()))),
    )
}

fn padding(input: &str) -> IResult<&str, Padding> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        assert_eq!(padding("p-5"), Ok(("", Padding::All("1.25rem"))));
        assert_eq!(padding("p-[3.251rem]"), Ok(("", Padding::All("3.251rem"))));
    }

    #[test]
    fn test_classes() {
        assert_eq!(classes("p-5 m-5"), Ok(("", vec!["p-5", "m-5"])));
    }
}
