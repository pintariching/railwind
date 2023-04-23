use crate::class::utils::{get_value, get_value_neg};
use crate::class::Decl;
use crate::utils::{get_class_name, get_opt_args};
use crate::warning::WarningType;

use super::{COLORS, CURSOR, MARGIN, PADDING};

const TOUCH_ACTION_STYLE: &str =
    "touch-action: var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)";

#[derive(Debug, PartialEq, Hash)]
pub struct AccentColor<'a>(pub &'a str);

impl<'a> AccentColor<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLORS)?;
        Ok(Decl::Single(format!("accent-color: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Appearance {
    None,
}

impl<'a> Appearance {
    pub fn new(arg: &'a str) -> Result<Self, WarningType> {
        match arg {
            "none" => Ok(Self::None),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Appearance".into(),
                vec!["none"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::None => Decl::Double([
                "-webkit-appearance: none".into(),
                "        appearance: none".into(),
            ]),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Cursor<'a>(pub &'a str);

impl<'a> Cursor<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &CURSOR)?;
        Ok(Decl::Single(format!("cursor: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct CaretColor<'a>(pub &'a str);

impl<'a> CaretColor<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLORS)?;
        Ok(Decl::Single(format!("caret-color: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum PointerEvents {
    None,
    Auto,
}

impl<'a> PointerEvents {
    pub fn new(arg: &'a str) -> Result<Self, WarningType> {
        match arg {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Pointer Events".into(),
                vec!["none", "auto"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::None => Decl::Single("pointer-events: none".into()),
            Self::Auto => Decl::Single("pointer-events: auto".into()),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Resize {
    None,
    Y,
    X,
    Both,
}

impl<'a> Resize {
    pub fn new(value: &'a str) -> Result<Self, WarningType> {
        match value {
            "resize-none" => Ok(Self::None),
            "resize-y" => Ok(Self::Y),
            "resize-x" => Ok(Self::X),
            "resize" => Ok(Self::Both),
            _ => Err(WarningType::InvalidArg(
                value.into(),
                "Resize".into(),
                vec!["resize-none", "resize-y", "resize-x", "resize"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::None => Decl::Lit("resize: none"),
            Self::Y => Decl::Lit("resize: vertical"),
            Self::X => Decl::Lit("resize: horizontal"),
            Self::Both => Decl::Lit("resize: both"),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

impl<'a> ScrollBehavior {
    pub fn new(arg: &'a str) -> Option<Self> {
        match arg {
            "auto" => Some(Self::Auto),
            "smooth" => Some(Self::Smooth),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::Auto => Decl::Lit("scroll-behavior: auto"),
            Self::Smooth => Decl::Lit("scroll-behavior: smooth"),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
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

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::All(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Single(format!("scroll-margin: {}", value)))
            }
            Self::X(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Double([
                    format!("scroll-margin-left: {}", value),
                    format!("scroll-margin-right: {}", value),
                ]))
            }
            Self::Y(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Double([
                    format!("scroll-margin-top: {}", value),
                    format!("scroll-margin-bottom: {}", value),
                ]))
            }
            Self::Top(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Single(format!("scroll-margin-top: {}", value)))
            }
            Self::Right(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Single(format!("scroll-margin-right: {}", value)))
            }
            Self::Bottom(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Single(format!("scroll-margin-bottom: {}", value)))
            }
            Self::Left(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Ok(Decl::Single(format!("scroll-margin-left: {}", value)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
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

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::All(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Single(format!("scroll-padding: {}", value)))
            }
            Self::X(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Double([
                    format!("scroll-padding-left: {}", value),
                    format!("scroll-padding-right: {}", value),
                ]))
            }
            Self::Y(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Double([
                    format!("scroll-padding-top: {}", value),
                    format!("scroll-padding-bottom: {}", value),
                ]))
            }
            Self::Top(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Single(format!("scroll-padding-top: {}", value)))
            }
            Self::Right(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Single(format!("scroll-padding-right: {}", value)))
            }
            Self::Bottom(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Single(format!("scroll-padding-bottom: {}", value)))
            }
            Self::Left(p) => {
                let value = get_value(p, &PADDING)?;
                Ok(Decl::Single(format!("scroll-padding-left: {}", value)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum ScrollSnapAlign {
    Start,
    End,
    Center,
    None,
}

impl<'a> ScrollSnapAlign {
    pub fn new(arg: &'a str) -> Option<Self> {
        match arg {
            "start" => Some(Self::Start),
            "end" => Some(Self::End),
            "center" => Some(Self::Center),
            "align-none" => Some(Self::None),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::Start => Decl::Lit("scroll-snap-align: start"),
            Self::End => Decl::Lit("scroll-snap-align: end"),
            Self::Center => Decl::Lit("scroll-snap-align: center"),
            Self::None => Decl::Lit("scroll-snap-align: none"),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum ScrollSnapStop {
    Normal,
    Always,
}

impl<'a> ScrollSnapStop {
    pub fn new(arg: &'a str) -> Option<Self> {
        match arg {
            "normal" => Some(Self::Normal),
            "always" => Some(Self::Always),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::Normal => Decl::Lit("scroll-snap-stop: normal"),
            Self::Always => Decl::Lit("scroll-snap-stop: always"),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
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
        match arg {
            "none" => Some(Self::None),
            "x" => Some(Self::X),
            "y" => Some(Self::Y),
            "both" => Some(Self::Both),
            "mandatory" => Some(Self::Mandatory),
            "proximity" => Some(Self::Proximity),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::None => Decl::Lit("scroll-snap-type: none"),
            Self::X => Decl::Lit("scroll-snap-type: x var(--tw-scroll-snap-strictness)"),
            Self::Y => Decl::Lit("scroll-snap-type: y var(--tw-scroll-snap-strictness)"),
            Self::Both => Decl::Lit("scroll-snap-type: both var(--tw-scroll-snap-strictness)"),
            Self::Mandatory => Decl::Lit("--tw-scroll-snap-strictness: mandatory"),
            Self::Proximity => Decl::Lit("--tw-scroll-snap-strictness: proximity"),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
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
    pub fn new(arg: &'a str) -> Result<Self, WarningType> {
        match arg {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "pan-x" => Ok(Self::PanX),
            "pan-left" => Ok(Self::PanLeft),
            "pan-right" => Ok(Self::PanRight),
            "pan-y" => Ok(Self::PanY),
            "pan-up" => Ok(Self::PanUp),
            "pan-down" => Ok(Self::PanDown),
            "pinch-zoom" => Ok(Self::PinchZoom),
            "manipulation" => Ok(Self::Manipulation),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Touch Action".into(),
                vec![
                    "auto",
                    "none",
                    "pan-x",
                    "pan-left",
                    "pan-right",
                    "pan-y",
                    "pan-up",
                    "pan-down",
                    "pinch-zoom",
                    "manipulation",
                ],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        match self {
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
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum UserSelect {
    None,
    Text,
    All,
    Auto,
}

impl UserSelect {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "none" => Ok(Self::None),
            "text" => Ok(Self::Text),
            "all" => Ok(Self::All),
            "auto" => Ok(Self::Auto),
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "User Select".into(),
                    vec!["none", "text", "all", "auto"],
                ))
            }
        }
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::None => "none",
            Self::Text => "text",
            Self::All => "all",
            Self::Auto => "auto",
        };

        Decl::Double([
            format!("-webkit-user-select: {}", value),
            format!("        user-select: {}", value),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum WillChange {
    Auto,
    Scroll,
    Contents,
    Transform,
}

impl WillChange {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "change-auto" => Ok(Self::Auto),
            "change-scroll" => Ok(Self::Scroll),
            "change-contents" => Ok(Self::Contents),
            "change-transform" => Ok(Self::Transform),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Will Change".into(),
                vec![
                    "change-auto",
                    "change-scroll",
                    "change-contents",
                    "change-transform",
                ],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::Auto => "auto",
            Self::Scroll => "scroll-position",
            Self::Contents => "contents",
            Self::Transform => "transform",
        };

        Decl::Single(format!("will-change: {}", value))
    }
}
