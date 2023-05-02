use macro_derive::{ConfigurableParser, EnumParser, IntoDeclaration};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::config::Config;

#[derive(Debug, PartialEq, Hash)]
pub enum Effects<'a> {
    BoxShadow(BoxShadow<'a>),
    BoxShadowColor(BoxShadowColor<'a>),
    Opacity(Opacity<'a>),
    MixBlendMode(MixBlendMode),
    BackgroundBlendMode(BackgroundBlendMode),
}

pub fn effects<'a>(input: &'a str, config: &'a Config) -> IResult<&'a str, Effects<'a>> {
    alt((
        preceded(
            tag("shadow-"),
            alt((
                map(|i| box_shadow(i, config), Effects::BoxShadow),
                map(|i| box_shadow_color(i, config), Effects::BoxShadowColor),
            )),
        ),
        map(tag("shadow"), |_| Effects::BoxShadow(BoxShadow(""))),
        preceded(
            tag("opacity-"),
            map(|i| opacity(i, config), Effects::Opacity),
        ),
        preceded(
            tag("mix-blend-"),
            map(mix_blend_mode, Effects::MixBlendMode),
        ),
        preceded(
            tag("bg-blend-"),
            map(background_blend_mode, Effects::BackgroundBlendMode),
        ),
    ))(input)
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(box_shadow)]
#[config(effects.get_box_shadow)]
#[decl("box-shadow")]
pub struct BoxShadow<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(box_shadow_color)]
#[config(effects.get_box_shadow_color)]
#[decl("--tw-shadow-color")]
pub struct BoxShadowColor<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, ConfigurableParser, IntoDeclaration)]
#[name(opacity)]
#[config(effects.get_opacity)]
#[decl("opacity")]
pub struct Opacity<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(mix_blend_mode)]
#[decl("mix-blend-mode")]
pub enum MixBlendMode {
    #[tag("normal")]
    Normal,
    #[tag("multiply")]
    Multiply,
    #[tag("screen")]
    Screen,
    #[tag("overlay")]
    Overlay,
    #[tag("darken")]
    Darken,
    #[tag("lighten")]
    Lighten,
    #[tag("color-dodge")]
    ColorDodge,
    #[tag("color-burn")]
    ColorBurn,
    #[tag("hard-light")]
    HardLight,
    #[tag("soft-light")]
    SoftLight,
    #[tag("difference")]
    Difference,
    #[tag("exclusion")]
    Exclusion,
    #[tag("hue")]
    Hue,
    #[tag("saturation")]
    Saturation,
    #[tag("color")]
    Color,
    #[tag("luminosity")]
    Luminosity,
    #[tag("plus-lighter")]
    PlusLighter,
}

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(background_blend_mode)]
#[decl("background-blend-mode")]
pub enum BackgroundBlendMode {
    #[tag("normal")]
    Normal,
    #[tag("multiply")]
    Multiply,
    #[tag("screen")]
    Screen,
    #[tag("overlay")]
    Overlay,
    #[tag("darken")]
    Darken,
    #[tag("lighten")]
    Lighten,
    #[tag("color-dodge")]
    ColorDodge,
    #[tag("color-burn")]
    ColorBurn,
    #[tag("hard-light")]
    HardLight,
    #[tag("soft-light")]
    SoftLight,
    #[tag("difference")]
    Difference,
    #[tag("exclusion")]
    Exclusion,
    #[tag("hue")]
    Hue,
    #[tag("saturation")]
    Saturation,
    #[tag("color")]
    Color,
    #[tag("luminosity")]
    Luminosity,
}
