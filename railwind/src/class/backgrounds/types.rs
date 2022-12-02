use crate::class::utils::get_value;
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
            BackgroundClip::Text => "text",
        };

        Decl::Single(format!("background-clip: {}", val))
    }
}

#[derive(Debug)]
pub struct BackgroundColor<'a>(pub &'a str);

impl<'a> BackgroundColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BACKGROUND_COLOR)?;
        Some(Decl::Single(format!("background-color: {}", value)))
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
        let value = get_value(self.0, &BACKGROUND_SIZE)?;
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
                Some(Decl::Triple([
                    format!("--tw-gradient-from: {}", value),
                    format!("--tw-gradient-to: {}00", value),
                    "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)".into(),
                ]))
            }
            GradientColorStops::To(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;
                Some(Decl::Single(format!("--tw-gradient-to: {}", value)))
            }
            GradientColorStops::Via(g) => {
                let value = get_value(g, &GRADIENT_COLOR_STOPS)?;
                Some(Decl::Double([
                    format!("--tw-gradient-to: {}00", value),
                    format!(
                        "--tw-gradient-stops: var(--tw-gradient-from), {}, var(--tw-gradient-to)",
                        value
                    ),
                ]))
            }
        }
    }
}
