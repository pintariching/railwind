use crate::class::utils::{get_value, get_value_neg};
use crate::class::Decl;
use crate::utils::{get_class_name, get_opt_args};

use super::{COLORS, CURSOR, MARGIN, PADDING};

const TOUCH_ACTION_STYLE: &str =
    "touch-action: var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)";

#[derive(Debug)]
pub struct AccentColor<'a>(pub &'a str);

impl<'a> AccentColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &COLORS)?;
        Some(Decl::Single(format!("accent-color: {}", value)))
    }
}

#[derive(Debug)]
pub enum Appearance {
    None,
}

impl<'a> Appearance {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "none" => Self::None,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::None => Decl::Double([
                "-webkit-appearance: none".into(),
                "        appearance: none".into(),
            ]),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub struct Cursor<'a>(pub &'a str);

impl<'a> Cursor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &CURSOR)?;
        Some(Decl::Single(format!("cursor: {}", value)))
    }
}

#[derive(Debug)]
pub struct CaretColor<'a>(pub &'a str);

impl<'a> CaretColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &COLORS)?;
        Some(Decl::Single(format!("caret-color: {}", value)))
    }
}

#[derive(Debug)]
pub enum PointerEvents {
    None,
    Auto,
}

impl<'a> PointerEvents {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "none" => Self::None,
            "auto" => Self::Auto,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::None => Decl::Single("pointer-events: none".into()),
            Self::Auto => Decl::Single("pointer-events: auto".into()),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub enum Resize {
    None,
    Y,
    X,
    Both,
}

impl<'a> Resize {
    pub fn new(value: &'a str) -> Option<Self> {
        let value = match value {
            "resize-none" => Self::None,
            "resize-y" => Self::Y,
            "resize-x" => Self::X,
            "resize" => Self::Both,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::None => Decl::Lit("resize: none"),
            Self::Y => Decl::Lit("resize: vertical"),
            Self::X => Decl::Lit("resize: horizontal"),
            Self::Both => Decl::Lit("resize: both"),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

impl<'a> ScrollBehavior {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "auto" => Self::Auto,
            "smooth" => Self::Smooth,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::Auto => Decl::Lit("scroll-behavior: auto"),
            Self::Smooth => Decl::Lit("scroll-behavior: smooth"),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub enum ScrollMargin<'a> {
    All(&'a str, bool),
    X(&'a str, bool),
    Y(&'a str, bool),
    Top(&'a str, bool),
    Right(&'a str, bool),
    Bottom(&'a str, bool),
    Left(&'a str, bool),
}

impl<'a> ScrollMargin<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Option<Self> {
        let negative = name.starts_with('-');

        match get_class_name(arg) {
            "m" => Some(Self::All(get_opt_args(arg), negative)),
            "mx" => Some(Self::X(get_opt_args(arg), negative)),
            "my" => Some(Self::Y(get_opt_args(arg), negative)),
            "mt" => Some(Self::Top(get_opt_args(arg), negative)),
            "mr" => Some(Self::Right(get_opt_args(arg), negative)),
            "mb" => Some(Self::Bottom(get_opt_args(arg), negative)),
            "ml" => Some(Self::Left(get_opt_args(arg), negative)),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::All(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("scroll-margin: {}", value)))
            }
            Self::X(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Double([
                    format!("scroll-margin-left: {}", value),
                    format!("scroll-margin-right: {}", value),
                ]))
            }
            Self::Y(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Double([
                    format!("scroll-margin-top: {}", value),
                    format!("scroll-margin-bottom: {}", value),
                ]))
            }
            Self::Top(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("scroll-margin-top: {}", value)))
            }
            Self::Right(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("scroll-margin-right: {}", value)))
            }
            Self::Bottom(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("scroll-margin-bottom: {}", value)))
            }
            Self::Left(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("scroll-margin-left: {}", value)))
            }
        }
    }
}

#[derive(Debug)]
pub enum ScrollPadding<'a> {
    All(&'a str),
    X(&'a str),
    Y(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
}

impl<'a> ScrollPadding<'a> {
    pub fn new(class_name: &'a str, arg: &'a str) -> Option<Self> {
        if class_name == "-scroll" {
            return None;
        }
        match get_class_name(arg) {
            "p" => Some(Self::All(get_opt_args(arg))),
            "px" => Some(Self::X(get_opt_args(arg))),
            "py" => Some(Self::Y(get_opt_args(arg))),
            "pt" => Some(Self::Top(get_opt_args(arg))),
            "pr" => Some(Self::Right(get_opt_args(arg))),
            "pb" => Some(Self::Bottom(get_opt_args(arg))),
            "pl" => Some(Self::Left(get_opt_args(arg))),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::All(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("scroll-padding: {}", value)))
            }
            Self::X(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Double([
                    format!("scroll-padding-left: {}", value),
                    format!("scroll-padding-right: {}", value),
                ]))
            }
            Self::Y(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Double([
                    format!("scroll-padding-top: {}", value),
                    format!("scroll-padding-bottom: {}", value),
                ]))
            }
            Self::Top(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("scroll-padding-top: {}", value)))
            }
            Self::Right(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("scroll-padding-right: {}", value)))
            }
            Self::Bottom(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("scroll-padding-bottom: {}", value)))
            }
            Self::Left(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("scroll-padding-left: {}", value)))
            }
        }
    }
}

#[derive(Debug)]
pub enum ScrollSnapAlign {
    Start,
    End,
    Center,
    None,
}

impl<'a> ScrollSnapAlign {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "align-none" => Self::None,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::Start => Decl::Lit("scroll-snap-align: start"),
            Self::End => Decl::Lit("scroll-snap-align: end"),
            Self::Center => Decl::Lit("scroll-snap-align: center"),
            Self::None => Decl::Lit("scroll-snap-align: none"),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub enum ScrollSnapStop {
    Normal,
    Always,
}

impl<'a> ScrollSnapStop {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "normal" => Self::Normal,
            "always" => Self::Always,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::Normal => Decl::Lit("scroll-snap-stop: normal"),
            Self::Always => Decl::Lit("scroll-snap-stop: always"),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub enum ScrollSnapType {
    None,
    X,
    Y,
    Both,
    Mandatory,
    Proximity,
}

impl<'a> ScrollSnapType {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "none" => Self::None,
            "x" => Self::X,
            "y" => Self::Y,
            "both" => Self::Both,
            "mandatory" => Self::Mandatory,
            "proximity" => Self::Proximity,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::None => Decl::Lit("scroll-snap-type: none"),
            Self::X => Decl::Lit("scroll-snap-type: x var(--tw-scroll-snap-strictness)"),
            Self::Y => Decl::Lit("scroll-snap-type: y var(--tw-scroll-snap-strictness)"),
            Self::Both => Decl::Lit("scroll-snap-type: both var(--tw-scroll-snap-strictness)"),
            Self::Mandatory => Decl::Lit("--tw-scroll-snap-strictness: mandatory"),
            Self::Proximity => Decl::Lit("--tw-scroll-snap-strictness: proximity"),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub enum TouchAction {
    Auto,
    None,
    PanX,
    PanLeft,
    PanRight,
    PanY,
    PanUp,
    PanDown,
    PinchZoom,
    Manipulation,
}

impl<'a> TouchAction {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "auto" => Self::Auto,
            "none" => Self::None,
            "pan-x" => Self::PanX,
            "pan-left" => Self::PanLeft,
            "pan-right" => Self::PanRight,
            "pan-y" => Self::PanY,
            "pan-up" => Self::PanUp,
            "pan-down" => Self::PanDown,
            "pinch-zoom" => Self::PinchZoom,
            "manipulation" => Self::Manipulation,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::Auto => Decl::Lit("touch-action: auto"),
            Self::None => Decl::Lit("touch-action: none"),
            Self::PanX => Decl::Double(["--tw-pan-x: pan-x".into(), TOUCH_ACTION_STYLE.into()]),
            Self::PanLeft => {
                Decl::Double(["--tw-pan-x: pan-left".into(), TOUCH_ACTION_STYLE.into()])
            }
            Self::PanRight => {
                Decl::Double(["--tw-pan-x: pan-right".into(), TOUCH_ACTION_STYLE.into()])
            }
            Self::PanY => Decl::Double(["--tw-pan-y: pan-y".into(), TOUCH_ACTION_STYLE.into()]),
            Self::PanUp => Decl::Double(["--tw-pan-y: pan-up".into(), TOUCH_ACTION_STYLE.into()]),
            Self::PanDown => {
                Decl::Double(["--tw-pan-y: pan-down".into(), TOUCH_ACTION_STYLE.into()])
            }
            Self::PinchZoom => Decl::Double([
                "--tw-pinch-zoom: pinch-zoom".into(),
                TOUCH_ACTION_STYLE.into(),
            ]),
            Self::Manipulation => Decl::Lit("touch-action: manipulation"),
        };
        Some(value)
    }
}

#[derive(Debug)]
pub struct UserSelect<'a>(pub &'a str);

impl<'a> UserSelect<'a> {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "none" => Self("none"),
            "text" => Self("text"),
            "all" => Self("all"),
            "auto" => Self("auto"),
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        Some(Decl::Double([
            format!("-webkit-user-select: {}", self.0),
            format!("        user-select: {}", self.0),
        ]))
    }
}

#[derive(Debug)]
pub struct WillChange<'a>(pub &'a str);

impl<'a> WillChange<'a> {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match arg {
            "change-auto" => Self("auto"),
            "change-scroll" => Self("scroll-position"),
            "change-contents" => Self("contents"),
            "change-transform" => Self("transform"),
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        Some(Decl::Single(format!("will-change: {}", self.0)))
    }
}
