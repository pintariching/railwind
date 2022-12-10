use crate::class::utils::{get_value, hex_to_rgb_color};
use crate::class::Decl;

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
            BackgroundAttachment::Fixed => "fixed",
            BackgroundAttachment::Local => "local",
            BackgroundAttachment::Scroll => "scroll",
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
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "border" => Self::Border,
            "padding" => Self::Padding,
            "content" => Self::Content,
            "text" => Self::Text,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            BackgroundClip::Border => "border-box",
            BackgroundClip::Padding => "padding-box",
            BackgroundClip::Content => "content-box",
            BackgroundClip::Text => {
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
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BACKGROUND_COLOR)?;

        if let Some(color) = hex_to_rgb_color(&value) {
            Some(Decl::Double([
                "--tw-bg-opacity: 1".into(),
                format!(
                    "background-color: rgb({} {} {} / var(--tw-bg-opacity))",
                    color[0], color[1], color[2]
                ),
            ]))
        } else {
            return Some(Decl::Single(format!("background-color: {}", value)));
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
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "border" => Self::Border,
            "padding" => Self::Padding,
            "content" => Self::Content,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            BackgroundOrigin::Border => "border-box",
            BackgroundOrigin::Padding => "padding-box",
            BackgroundOrigin::Content => "content-box",
        };

        Decl::Single(format!("background-origin: {}", val))
    }
}

#[derive(Debug)]
pub struct BackgroundPosition<'a>(pub &'a str);

impl<'a> BackgroundPosition<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BACKGROUND_POSITION)?;
        Some(Decl::Single(format!("background-position: {}", value)))
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
            BackgroundRepeat::Repeat => "repeat",
            BackgroundRepeat::NoRepeat => "no-repeat",
            BackgroundRepeat::RepeatX => "repeat-x",
            BackgroundRepeat::RepeatY => "repeat-y",
            BackgroundRepeat::RepeatRound => "round",
            BackgroundRepeat::RepeatSpace => "space",
        };

        Decl::Single(format!("background-repeat: {}", val))
    }
}

#[derive(Debug)]
pub struct BackgroundSize<'a>(pub &'a str);

impl<'a> BackgroundSize<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let mut value = get_value(self.0, &BACKGROUND_SIZE)?;

        if value.starts_with("length:") {
            value = value[7..].into();
        }

        Some(Decl::Single(format!("background-size: {}", value)))
    }
}

#[derive(Debug)]
pub struct BackgroundImage<'a>(pub &'a str);

impl<'a> BackgroundImage<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BACKGROUND_IMAGE)?;
        Some(Decl::Single(format!("background-image: {}", value)))
    }
}

#[derive(Debug)]
pub enum GradientColorStops<'a> {
    From(&'a str),
    To(&'a str),
    Via(&'a str),
}

impl<'a> GradientColorStops<'a> {
    pub fn new(class_name: &str, args: &'a str) -> Option<Self> {
        let value = match class_name {
            "from" => Self::From(args),
            "to" => Self::To(args),
            "via" => Self::Via(args),
            _ => return None,
        };

        Some(value)
    }
    pub fn to_decl(self) -> Option<Decl> {
        match self {
            GradientColorStops::From(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;

                if let Some(color) = hex_to_rgb_color(&value) {
                    Some(Decl::Triple([
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
                        _ => return None,
                    };

                    Some(Decl::Triple([
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
            GradientColorStops::To(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;
                Some(Decl::Single(format!("--tw-gradient-to: {}", value)))
            }
            GradientColorStops::Via(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;

                if let Some(color) = hex_to_rgb_color(&value) {
                    Some(Decl::Double([
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
                        _ => return None,
                    };

                    Some(Decl::Double([
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
