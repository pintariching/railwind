use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

use crate::class::{Decl, IntoDeclaration};

#[derive(Debug, PartialEq, Hash)]
pub enum Accessibility {
    ScreenReaders(ScreenReaders),
}

pub fn accessibility(input: &str) -> IResult<&str, Accessibility> {
    map(screen_readers, Accessibility::ScreenReaders)(input)
}

impl IntoDeclaration for Accessibility {
    fn to_decl(self) -> Decl {
        match self {
            Accessibility::ScreenReaders(a) => a.to_decl(),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum ScreenReaders {
    SrOnly,
    NotSrOnly,
}

fn screen_readers(input: &str) -> IResult<&str, ScreenReaders> {
    alt((
        map(tag("sr-only"), |_| ScreenReaders::SrOnly),
        map(tag("not-sr-only"), |_| ScreenReaders::NotSrOnly),
    ))(input)
}

impl IntoDeclaration for ScreenReaders {
    fn to_decl(self) -> super::Decl {
        match self {
            Self::SrOnly => Decl::LitVec(vec![
                "position: absolute",
                "width: 1px",
                "height: 1px",
                "padding: 0",
                "margin: -1px",
                "overflow: hidden",
                "clip: rect(0, 0, 0, 0)",
                "white-space: nowrap",
                "border-width: 0",
            ]),
            Self::NotSrOnly => Decl::LitVec(vec![
                "position: static",
                "width: auto",
                "height: auto",
                "padding: 0",
                "margin: 0",
                "overflow: visible",
                "clip: auto",
                "white-space: normal",
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_readers() {
        assert_eq!(screen_readers("sr-only"), Ok(("", ScreenReaders::SrOnly)));
        assert_eq!(
            screen_readers("not-sr-only"),
            Ok(("", ScreenReaders::NotSrOnly))
        );
    }
}
