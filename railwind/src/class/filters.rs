use macro_derive::KeywordConfigurableParser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::class::Decl;

use crate::class::utils::neg_keyword_value;
use crate::class::IntoDeclaration;
use crate::config::Config;

const FILTER_STYLE: &str = "filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)";
const WEBKIT_BACKDROP_FILTER_STYLE: &str = "-webkit-backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)";
const BACKDROP_FILTER_STYLE: &str = "        backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)";

#[derive(Debug, PartialEq, Hash)]
pub enum Filter<'a> {
    Blur(Blur<'a>),
    Brightness(Brightness<'a>),
    Contrast(Contrast<'a>),
    DropShadow(DropShadow<'a>),
    Grayscale(Grayscale<'a>),
    HueRotate(HueRotate),
    Invert(Invert<'a>),
    Saturate(Saturate<'a>),
    Sepia(Sepia<'a>),
    BackdropBlur(BackdropBlur<'a>),
    BackdropBrightness(BackdropBrightness<'a>),
    BackdropContrast(BackdropContrast<'a>),
    BackdropGrayscale(BackdropGrayscale<'a>),
    BackdropHueRotate(BackdropHueRotate),
    BackdropInvert(BackdropInvert<'a>),
    BackdropOpacity(BackdropOpacity<'a>),
    BackdropSaturate(BackdropSaturate<'a>),
    BackdropSepia(BackdropSepia<'a>),
}

pub fn filter<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Filter<'a>> {
    alt((
        map(|i| blur(i, config), Filter::Blur),
        map(|i| brightness(i, config), Filter::Brightness),
        map(|i| contrast(i, config), Filter::Contrast),
        map(|i| drop_shadow(i, config), Filter::DropShadow),
        map(|i| grayscale(i, config), Filter::Grayscale),
        map(|i| hue_rotate(i, config), Filter::HueRotate),
        map(|i| invert(i, config), Filter::Invert),
        map(|i| saturate(i, config), Filter::Saturate),
        map(|i| sepia(i, config), Filter::Sepia),
        map(|i| backdrop_opacity(i, config), Filter::BackdropOpacity),
        preceded(
            tag("backdrop-"),
            alt((
                map(
                    |i| blur(i, config),
                    |b| Filter::BackdropBlur(BackdropBlur(b.0)),
                ),
                map(
                    |i| brightness(i, config),
                    |b| Filter::BackdropBrightness(BackdropBrightness(b.0)),
                ),
                map(
                    |i| contrast(i, config),
                    |b| Filter::BackdropContrast(BackdropContrast(b.0)),
                ),
                map(
                    |i| grayscale(i, config),
                    |b| Filter::BackdropGrayscale(BackdropGrayscale(b.0)),
                ),
                map(
                    |i| hue_rotate(i, config),
                    |b| Filter::BackdropHueRotate(BackdropHueRotate(b.0)),
                ),
                map(
                    |i| invert(i, config),
                    |b| Filter::BackdropInvert(BackdropInvert(b.0)),
                ),
                map(
                    |i| saturate(i, config),
                    |b| Filter::BackdropSaturate(BackdropSaturate(b.0)),
                ),
                map(
                    |i| sepia(i, config),
                    |b| Filter::BackdropSepia(BackdropSepia(b.0)),
                ),
            )),
        ),
    ))(input)
}

impl<'a> IntoDeclaration for Filter<'a> {
    fn to_decl(self) -> Decl {
        match self {
            Filter::Blur(f) => f.to_decl(),
            Filter::Brightness(f) => f.to_decl(),
            Filter::Contrast(f) => f.to_decl(),
            Filter::DropShadow(f) => f.to_decl(),
            Filter::Grayscale(f) => f.to_decl(),
            Filter::HueRotate(f) => f.to_decl(),
            Filter::Invert(f) => f.to_decl(),
            Filter::Saturate(f) => f.to_decl(),
            Filter::Sepia(f) => f.to_decl(),
            Filter::BackdropBlur(f) => f.to_decl(),
            Filter::BackdropBrightness(f) => f.to_decl(),
            Filter::BackdropContrast(f) => f.to_decl(),
            Filter::BackdropGrayscale(f) => f.to_decl(),
            Filter::BackdropHueRotate(f) => f.to_decl(),
            Filter::BackdropInvert(f) => f.to_decl(),
            Filter::BackdropOpacity(f) => f.to_decl(),
            Filter::BackdropSaturate(f) => f.to_decl(),
            Filter::BackdropSepia(f) => f.to_decl(),
        }
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(blur)]
#[keyword("blur")]
#[empty_args(true)]
#[config(filters.get_blur)]
pub struct Blur<'a>(pub &'a str);

impl<'a> IntoDeclaration for Blur<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-blur: blur({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(brightness)]
#[keyword("brightness")]
#[empty_args(false)]
#[config(filters.get_brightness)]
pub struct Brightness<'a>(pub &'a str);

impl<'a> IntoDeclaration for Brightness<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-brightness: brightness({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(contrast)]
#[keyword("contrast")]
#[empty_args(false)]
#[config(filters.get_contrast)]
pub struct Contrast<'a>(pub &'a str);

impl<'a> IntoDeclaration for Contrast<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-contrast: contrast({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(drop_shadow)]
#[keyword("drop-shadow")]
#[empty_args(true)]
#[config(filters.get_drop_shadow)]
pub struct DropShadow<'a>(pub &'a str);

impl<'a> IntoDeclaration for DropShadow<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-drop-shadow: drop-shadow({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(grayscale)]
#[keyword("grayscale")]
#[empty_args(true)]
#[config(filters.get_grayscale)]
pub struct Grayscale<'a>(pub &'a str);

impl<'a> IntoDeclaration for Grayscale<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-grayscale: grayscale({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct HueRotate(pub String);

fn hue_rotate<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, HueRotate> {
    map(
        neg_keyword_value("hue-rotate", config.filters.get_hue_rotate()),
        HueRotate,
    )(input)
}

impl IntoDeclaration for HueRotate {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-hue-rotate: hue-rotate({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(invert)]
#[keyword("invert")]
#[empty_args(true)]
#[config(filters.get_invert)]
pub struct Invert<'a>(pub &'a str);

impl<'a> IntoDeclaration for Invert<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-invert: invert({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(saturate)]
#[keyword("saturate")]
#[empty_args(false)]
#[config(filters.get_saturate)]
pub struct Saturate<'a>(pub &'a str);

impl<'a> IntoDeclaration for Saturate<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-saturate: saturate({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(sepia)]
#[keyword("sepia")]
#[empty_args(true)]
#[config(filters.get_sepia)]
pub struct Sepia<'a>(pub &'a str);

impl<'a> IntoDeclaration for Sepia<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-sepia: sepia({})", self.0),
            FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropBlur<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropBlur<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-blur: blur({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropBrightness<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropBrightness<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-brightness: brightness({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropContrast<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropContrast<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-contrast: contrast({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropGrayscale<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropGrayscale<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-grayscale: grayscale({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropHueRotate(pub String);

impl<'a> IntoDeclaration for BackdropHueRotate {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-hue-rotate: hue-rotate({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropInvert<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropInvert<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-invert: invert({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash, KeywordConfigurableParser)]
#[name(backdrop_opacity)]
#[keyword("backdrop-opacity")]
#[empty_args(false)]
#[config(filters.get_backdrop_opacity)]
pub struct BackdropOpacity<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropOpacity<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-opacity: opacity({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropSaturate<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropSaturate<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-saturate: saturate({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct BackdropSepia<'a>(pub &'a str);

impl<'a> IntoDeclaration for BackdropSepia<'a> {
    fn to_decl(self) -> Decl {
        Decl::Vec(vec![
            format!("--tw-backdrop-sepia: sepia({})", self.0),
            WEBKIT_BACKDROP_FILTER_STYLE.into(),
            BACKDROP_FILTER_STYLE.into(),
        ])
    }
}
