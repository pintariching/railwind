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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classes() {
        assert_eq!(classes("p-5 m-5"), Ok(("", vec!["p-5", "m-5"])));
    }
}
