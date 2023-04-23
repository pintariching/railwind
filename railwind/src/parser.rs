use indexmap::IndexSet;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::IResult;

use crate::class::Class;

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

fn class(input: &str) -> IResult<&str, Class> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::class::{
        types::{Margin, Padding},
        Spacing,
    };

    use super::*;

    #[test]
    fn test_class_attr() {
        assert_eq!(
            class_attr("class=\"p-5 m-5\""),
            Ok((
                "",
                IndexSet::from([
                    Class::Spacing(Spacing::Padding(Padding::All("5"))),
                    Class::Spacing(Spacing::Margin(Margin::All("5", false)))
                ])
            ))
        );
    }

    #[test]
    fn test_classes() {
        assert_eq!(classes("p-5 m-5"), Ok(("", vec!["p-5", "m-5"])));
    }
}
