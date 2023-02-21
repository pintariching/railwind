use crate::class::utils::{get_value, hex_to_rgb_color};
use crate::class::Decl;
use crate::warning::WarningType;

use super::{
    BACKGROUND_COLOR, BACKGROUND_IMAGE, BACKGROUND_POSITION, BACKGROUND_SIZE, GRADIENT_COLOR_STOPS,
};

#[derive(Debug)]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
}

impl BackgroundAttachment {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "fixed" => Self::Fixed,
            "local" => Self::Local,
            "scroll" => Self::Scroll,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Fixed => "fixed",
            Self::Local => "local",
            Self::Scroll => "scroll",
        };

        Decl::Single(format!("background-attachment: {}", val))
    }
}

#[derive(Debug)]
pub enum BackgroundClip {
    Border,
    Padding,
    Content,
    Text,
}

impl BackgroundClip {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "border" => Ok(Self::Border),
            "padding" => Ok(Self::Padding),
            "content" => Ok(Self::Content),
            "text" => Ok(Self::Text),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Background Clip".into(),
                vec!["border", "padding", "content", "text"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
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

        Decl::Single(format!("background-clip: {}", val))
    }
}

#[derive(Debug)]
pub struct BackgroundColor<'a>(pub &'a str);

impl<'a> BackgroundColor<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &BACKGROUND_COLOR)?;

        if let Some(color) = hex_to_rgb_color(&value) {
            Ok(Decl::Double([
                "--tw-bg-opacity: 1".into(),
                format!(
                    "background-color: rgb({} {} {} / var(--tw-bg-opacity))",
                    color[0], color[1], color[2]
                ),
            ]))
        } else {
            return Ok(Decl::Single(format!("background-color: {}", value)));
        }
    }
}

#[derive(Debug)]
pub enum BackgroundOrigin {
    Border,
    Padding,
    Content,
}

impl BackgroundOrigin {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "border" => Ok(Self::Border),
            "padding" => Ok(Self::Padding),
            "content" => Ok(Self::Content),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Background Origin".into(),
                vec!["border", "padding", "content"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Border => "border-box",
            Self::Padding => "padding-box",
            Self::Content => "content-box",
        };

        Decl::Single(format!("background-origin: {}", val))
    }
}

#[derive(Debug)]
pub struct BackgroundPosition<'a>(pub &'a str);

impl<'a> BackgroundPosition<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &BACKGROUND_POSITION)?;
        Ok(Decl::Single(format!("background-position: {}", value)))
    }
}

#[derive(Debug)]
pub enum BackgroundRepeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
    RepeatRound,
    RepeatSpace,
}

impl BackgroundRepeat {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "repeat" => Self::Repeat,
            "no-repeat" => Self::NoRepeat,
            "repeat-x" => Self::RepeatX,
            "repeat-y" => Self::RepeatY,
            "repeat-round" => Self::RepeatRound,
            "repeat-space" => Self::RepeatSpace,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Repeat => "repeat",
            Self::NoRepeat => "no-repeat",
            Self::RepeatX => "repeat-x",
            Self::RepeatY => "repeat-y",
            Self::RepeatRound => "round",
            Self::RepeatSpace => "space",
        };

        Decl::Single(format!("background-repeat: {}", val))
    }
}

#[derive(Debug)]
pub struct BackgroundSize<'a>(pub &'a str);

impl<'a> BackgroundSize<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let mut value = get_value(self.0, &BACKGROUND_SIZE)?;

        if value.starts_with("length:") {
            value = value[7..].into();
        }

        Ok(Decl::Single(format!("background-size: {}", value)))
    }
}

#[derive(Debug)]
pub struct BackgroundImage<'a>(pub &'a str);

impl<'a> BackgroundImage<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &BACKGROUND_IMAGE)?;
        Ok(Decl::Single(format!("background-image: {}", value)))
    }
}

#[derive(Debug)]
pub enum GradientColorStops<'a> {
    From(&'a str),
    To(&'a str),
    Via(&'a str),
}

impl<'a> GradientColorStops<'a> {
    pub fn new(class_name: &str, args: &'a str) -> Result<Self, WarningType> {
        match class_name {
            "from" => Ok(Self::From(args)),
            "to" => Ok(Self::To(args)),
            "via" => Ok(Self::Via(args)),
            _ => Err(WarningType::InvalidArg(
                args.into(),
                "Gradient Color Stops".into(),
                vec!["from", "to", "via"],
            )),
        }
    }
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::From(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;

                if let Some(color) = hex_to_rgb_color(&value) {
                    Ok(Decl::Triple([
                        format!("--tw-gradient-from: {}", value),
                        format!(
                            "--tw-gradient-to: rgb({} {} {} / 0)",
                            color[0], color[1], color[2]
                        ),
                        "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)"
                            .into(),
                    ]))
                } else {
                    let color = match value.as_str() {
                        "inherit" => [255, 255, 255],
                        "currentColor" => [255, 255, 255],
                        "transparent" => [0, 0, 0],
                        _ => return Err(WarningType::ValueNotFound(value)),
                    };

                    Ok(Decl::Triple([
                        format!("--tw-gradient-from: {}", value),
                        format!(
                            "--tw-gradient-to: rgb({} {} {} / 0)",
                            color[0], color[1], color[2]
                        ),
                        "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)"
                            .into(),
                    ]))
                }
            }
            Self::To(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;
                Ok(Decl::Single(format!("--tw-gradient-to: {}", value)))
            }
            Self::Via(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;

                if let Some(color) = hex_to_rgb_color(&value) {
                    Ok(Decl::Double([
                        format!(
                            "--tw-gradient-to: rgb({} {} {} / 0)",
                            color[0], color[1], color[2]
                        ),
                        format!("--tw-gradient-stops: var(--tw-gradient-from), {}, var(--tw-gradient-to)", value),
                    ]))
                } else {
                    let color = match value.as_str() {
                        "inherit" => [255, 255, 255],
                        "currentColor" => [255, 255, 255],
                        "transparent" => [0, 0, 0],
                        _ => return Err(WarningType::ValueNotFound(value)),
                    };

                    Ok(Decl::Double([
                        format!(
                            "--tw-gradient-to: rgb({} {} {} / 0)",
                            color[0], color[1], color[2]
                        ),
                        format!(
                            "--tw-gradient-stops: var(--tw-gradient-from), {}, var(--tw-gradient-to)",
                            value
                        ),
                    ]))
                }
            }
        }
    }
}
