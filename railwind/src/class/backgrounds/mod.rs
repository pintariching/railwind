use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::combinator::{map, map_opt};
use nom::sequence::{preceded, terminated};
use nom::IResult;

use crate::class::utils::{arbitrary, hex_to_rgb_color, keyword_dash};
use crate::class::{Decl, IntoDeclaration};
use crate::Config;

use super::utils::{hashmap_value, keyword_value};

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

pub fn background<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Backgrounds<'a>> {
    preceded(
        preceded(tag("bg"), tag("-")),
        alt((
            map(attachment, Backgrounds::BackgroundAttachment),
            map(clip, Backgrounds::BackgroundClip),
            map(|i| color(i, config), Backgrounds::BackgroundColor),
            map(origin, Backgrounds::BackgroundOrigin),
            map(|i| position(i, config), Backgrounds::BackgroundPosition),
            map(repeat, Backgrounds::BackgroundRepeat),
            map(|i| size(i, config), Backgrounds::BackgroundSize),
            map(|i| image(i, config), Backgrounds::BackgroundImage),
            map(
                |i| gradient_color_stops(i, config),
                Backgrounds::GradientColorStops,
            ),
        )),
    )(input)
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

#[derive(Debug, PartialEq, Hash)]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
}

fn attachment(input: &str) -> IResult<&str, BackgroundAttachment> {
    alt((
        map(tag("fixed"), |_| BackgroundAttachment::Fixed),
        map(tag("local"), |_| BackgroundAttachment::Local),
        map(tag("scroll"), |_| BackgroundAttachment::Scroll),
    ))(input)
}

impl IntoDeclaration for BackgroundAttachment {
    fn to_decl(self) -> Decl {
        let val = match self {
            Self::Fixed => "fixed",
            Self::Local => "local",
            Self::Scroll => "scroll",
        };

        Decl::String(format!("background-attachment: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum BackgroundClip {
    Border,
    Padding,
    Content,
    Text,
}

fn clip(input: &str) -> IResult<&str, BackgroundClip> {
    alt((
        map(tag("border"), |_| BackgroundClip::Border),
        map(tag("padding"), |_| BackgroundClip::Padding),
        map(tag("content"), |_| BackgroundClip::Content),
        map(tag("text"), |_| BackgroundClip::Text),
    ))(input)
}

impl IntoDeclaration for BackgroundClip {
    fn to_decl(self) -> Decl {
        let val = match self {
            Self::Border => "border-box",
            Self::Padding => "padding-box",
            Self::Content => "content-box",
            Self::Text => {
                return Decl::Double([
                    "-webkit-background-clip: text".into(),
                    "background-clip: text".into(),
                ])
            }
        };

        Decl::String(format!("background-clip: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackgroundColor<'a>(pub &'a str);

fn color<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BackgroundColor<'a>> {
    map(
        hashmap_value(&config.get_backgrounds().color),
        BackgroundColor,
    )(input)
}

impl<'a> IntoDeclaration for BackgroundColor<'a> {
    fn to_decl(self) -> Decl {
        if let Some(color) = hex_to_rgb_color(self.0) {
            Decl::Double([
                "--tw-bg-opacity: 1".into(),
                format!(
                    "background-color: rgb({} {} {} / var(--tw-bg-opacity))",
                    color[0], color[1], color[2]
                ),
            ])
        } else {
            return Decl::String(format!("background-color: {}", self.0));
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum BackgroundOrigin {
    Border,
    Padding,
    Content,
}

fn origin(input: &str) -> IResult<&str, BackgroundOrigin> {
    alt((
        map(tag("border"), |_| BackgroundOrigin::Border),
        map(tag("padding"), |_| BackgroundOrigin::Padding),
        map(tag("content"), |_| BackgroundOrigin::Content),
    ))(input)
}

impl IntoDeclaration for BackgroundOrigin {
    fn to_decl(self) -> Decl {
        let val = match self {
            Self::Border => "border-box",
            Self::Padding => "padding-box",
            Self::Content => "content-box",
        };

        Decl::String(format!("background-origin: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackgroundPosition<'a>(pub &'a str);

fn position<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BackgroundPosition<'a>> {
    map(
        hashmap_value(&config.get_backgrounds().position),
        BackgroundPosition,
    )(input)
}

impl<'a> IntoDeclaration for BackgroundPosition<'a> {
    fn to_decl(self) -> Decl {
        Decl::String(format!("background-position: {}", self.0))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum BackgroundRepeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
    RepeatRound,
    RepeatSpace,
}

fn repeat(input: &str) -> IResult<&str, BackgroundRepeat> {
    alt((
        map(tag("repeat"), |_| BackgroundRepeat::Repeat),
        map(tag("no-repeat"), |_| BackgroundRepeat::NoRepeat),
        map(tag("repeat-x"), |_| BackgroundRepeat::RepeatX),
        map(tag("repeat-y"), |_| BackgroundRepeat::RepeatY),
        map(tag("repeat-round"), |_| BackgroundRepeat::RepeatRound),
        map(tag("repeat-space"), |_| BackgroundRepeat::RepeatSpace),
    ))(input)
}

impl IntoDeclaration for BackgroundRepeat {
    fn to_decl(self) -> Decl {
        let val = match self {
            Self::Repeat => "repeat",
            Self::NoRepeat => "no-repeat",
            Self::RepeatX => "repeat-x",
            Self::RepeatY => "repeat-y",
            Self::RepeatRound => "round",
            Self::RepeatSpace => "space",
        };

        Decl::String(format!("background-repeat: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackgroundSize<'a>(pub &'a str);

fn size<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BackgroundSize<'a>> {
    map(
        hashmap_value(&config.get_backgrounds().size),
        BackgroundSize,
    )(input)
}

impl<'a> IntoDeclaration for BackgroundSize<'a> {
    fn to_decl(self) -> Decl {
        Decl::String(format!("background-size: {}", self.0))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackgroundImage<'a>(pub &'a str);

fn image<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, BackgroundImage<'a>> {
    map(
        hashmap_value(&config.get_backgrounds().image),
        BackgroundImage,
    )(input)
}

impl<'a> IntoDeclaration for BackgroundImage<'a> {
    fn to_decl(self) -> Decl {
        Decl::String(format!("background-image: {}", self.0))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum GradientColorStops<'a> {
    From(&'a str),
    To(&'a str),
    Via(&'a str),
}

fn gradient_color_stops<'a>(
    input: &'a str,
    config: &'a Config,
) -> IResult<&'a str, GradientColorStops<'a>> {
    let g = |keyword| keyword_value(keyword, &config.get_backgrounds().gradient_color_stops);

    alt((
        map(g("from"), GradientColorStops::From),
        map(g("to"), GradientColorStops::To),
        map(g("via"), GradientColorStops::Via),
    ))(input)
}

impl<'a> IntoDeclaration for GradientColorStops<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Self::From(g) => Decl::Vec(vec![
                format!("--tw-gradient-from: {}", g),
                format!("--tw-gradient-to: {}", g),
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
            background("bg-fixed", &Config::new()),
            Ok((
                "",
                Backgrounds::BackgroundAttachment(BackgroundAttachment::Fixed)
            ))
        );
    }

    #[test]
    fn test_clip() {
        assert_eq!(
            background("bg-content", &Config::new()),
            Ok(("", Backgrounds::BackgroundClip(BackgroundClip::Content)))
        );
    }

    #[test]
    fn test_color() {
        assert_eq!(
            background("bg-red-500", &Config::new()),
            Ok(("", Backgrounds::BackgroundColor(BackgroundColor("#ef4444"))))
        );
    }

    #[test]
    fn test_config() {
        let mut c = Config::new();

        c.get_mut_backgrounds().color.insert("yellow", "#yellow");

        assert_eq!(
            background("bg-yellow", &c),
            Ok(("", Backgrounds::BackgroundColor(BackgroundColor("#yellow"))))
        );
    }
}
