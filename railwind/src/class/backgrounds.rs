use macro_derive::{ConfigurableEnumParser, ConfigurableParser, EnumParser, IntoDeclaration};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::class::colors::hex_color;
use crate::class::{Decl, IntoDeclaration};
use crate::config::Config;

#[derive(Debug, PartialEq, Hash)]
pub enum Backgrounds<'a> {
    BackgroundAttachment(BackgroundAttachment),
    BackgroundClip(BackgroundClip),
    BackgroundColor(BackgroundColor<'a>),
    BackgroundOrigin(BackgroundOrigin),
    BackgroundPosition(BackgroundPosition<'a>),
    BackgroundRepeat(BackgroundRepeat),
    BackgroundSize(BackgroundSize<'a>),
    BackgroundImage(BackgroundImage<'a>),
    GradientColorStops(GradientColorStops<'a>),
}

pub fn backgrounds<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Backgrounds<'a>> {
    alt((
        preceded(
            tag("bg-"),
            alt((
                map(attachment, Backgrounds::BackgroundAttachment),
                map(clip, Backgrounds::BackgroundClip),
                map(|i| color(i, config), Backgrounds::BackgroundColor),
                map(origin, Backgrounds::BackgroundOrigin),
                map(|i| position(i, config), Backgrounds::BackgroundPosition),
                map(repeat, Backgrounds::BackgroundRepeat),
                map(|i| size(i, config), Backgrounds::BackgroundSize),
                map(|i| image(i, config), Backgrounds::BackgroundImage),
            )),
        ),
        map(
            |i| gradient_color_stops(i, config),
            Backgrounds::GradientColorStops,
        ),
    ))(input)
}

impl<'a> IntoDeclaration for Backgrounds<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Backgrounds::BackgroundAttachment(b) => b.to_decl(),
            Backgrounds::BackgroundClip(b) => b.to_decl(),
            Backgrounds::BackgroundColor(b) => b.to_decl(),
            Backgrounds::BackgroundOrigin(b) => b.to_decl(),
            Backgrounds::BackgroundPosition(b) => b.to_decl(),
            Backgrounds::BackgroundRepeat(b) => b.to_decl(),
            Backgrounds::BackgroundSize(b) => b.to_decl(),
            Backgrounds::BackgroundImage(b) => b.to_decl(),
            Backgrounds::GradientColorStops(b) => b.to_decl(),
        }
    }
}

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(attachment)]
#[decl("background-attachment")]
pub enum BackgroundAttachment {
    #[tag("fixed")]
    Fixed,
    #[tag("local")]
    Local,
    #[tag("scroll")]
    Scroll,
}

#[derive(Debug, PartialEq, Hash, EnumParser)]
#[name(clip)]
pub enum BackgroundClip {
    #[tag("border")]
    Border,
    #[tag("padding")]
    Padding,
    #[tag("content")]
    Content,
    #[tag("text")]
    Text,
}

impl IntoDeclaration for BackgroundClip {
    fn to_decl(self) -> Decl {
        let val = match self {
            Self::Border => "border-box",
            Self::Padding => "padding-box",
            Self::Content => "content-box",
            Self::Text => {
                return Decl::Vec(vec![
                    "-webkit-background-clip: text".into(),
                    "background-clip: text".into(),
                ])
            }
        };

        Decl::String(format!("background-clip: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser)]
#[name(color)]
#[config(backgrounds.get_color)]
pub struct BackgroundColor<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackgroundColor<'a> {
    fn to_decl(self) -> Decl {
        if let Ok((_, color)) = hex_color(self.0) {
            Decl::Double([
                "--tw-bg-opacity: 1".into(),
                format!("background-color: rgb({color} / var(--tw-bg-opacity))"),
            ])
        } else {
            return Decl::String(format!("background-color: {}", self.0));
        }
    }
}

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(origin)]
#[decl("background-origin")]
pub enum BackgroundOrigin {
    #[tag("border")]
    #[decl("border-box")]
    Border,
    #[tag("padding")]
    #[decl("padding-box")]
    Padding,
    #[tag("content")]
    #[decl("content-box")]
    Content,
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(position)]
#[config(backgrounds.get_position)]
#[decl("background-position")]
pub struct BackgroundPosition<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(repeat)]
#[decl("background-repeat")]
pub enum BackgroundRepeat {
    #[tag("repeat")]
    Repeat,
    #[tag("no-repeat")]
    NoRepeat,
    #[tag("repeat-x")]
    RepeatX,
    #[tag("repeat-y")]
    RepeatY,
    #[tag("repeat-round")]
    #[decl("round")]
    RepeatRound,
    #[tag("repeat-space")]
    #[decl("space")]
    RepeatSpace,
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(size)]
#[config(backgrounds.get_size)]
#[decl("background-size")]
pub struct BackgroundSize<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(image)]
#[config(backgrounds.get_image)]
#[decl("background-image")]
pub struct BackgroundImage<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, ConfigurableEnumParser)]
#[name(gradient_color_stops)]
#[config(backgrounds.get_gradient_color_stops)]
pub enum GradientColorStops<'a> {
    #[tag("from")]
    From(&'a str),
    #[tag("to")]
    To(&'a str),
    #[tag("via")]
    Via(&'a str),
}

impl<'a> IntoDeclaration for GradientColorStops<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::From(g) => Decl::Vec(vec![
                format!("--tw-gradient-from: {}", g),
                format!(
                    "--tw-gradient-to: rgb({}) / 0",
                    hex_color(g)
                        .map(|(_, color)| color)
                        .unwrap_or("0 0 0".to_string())
                ),
                "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)".into(),
            ]),
            Self::To(g) => Decl::String(format!("--tw-gradient-to: {}", g)),
            Self::Via(g) => Decl::Vec(vec![
                format!("--tw-gradient-to: {}", g),
                format!(
                    "--tw-gradient-stops: var(--tw-gradient-from), {}, var(--tw-gradient-to)",
                    g
                ),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment() {
        assert_eq!(
            backgrounds("bg-fixed", &Config::default()),
            Ok((
                "",
                Backgrounds::BackgroundAttachment(BackgroundAttachment::Fixed)
            ))
        );
    }

    #[test]
    fn test_clip() {
        assert_eq!(
            backgrounds("bg-content", &Config::default()),
            Ok(("", Backgrounds::BackgroundClip(BackgroundClip::Content)))
        );
    }

    #[test]
    fn test_color() {
        assert_eq!(
            backgrounds("bg-red-500", &Config::default()),
            Ok(("", Backgrounds::BackgroundColor(BackgroundColor("#ef4444"))))
        );
    }

    #[test]
    fn test_config() {
        let mut c = Config::default();

        c.backgrounds.get_mut_color().insert("yellow", "#yellow");

        assert_eq!(
            backgrounds("bg-yellow", &c),
            Ok(("", Backgrounds::BackgroundColor(BackgroundColor("#yellow"))))
        );
    }
}
