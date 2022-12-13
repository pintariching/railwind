use crate::class::utils::{get_value, hex_to_rgb_color, value_is_hex};
use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};

use super::{
    BORDER_COLOR, BORDER_RADIUS, BORDER_WIDTH, DIVIDE_COLOR, DIVIDE_WIDTH, OUTLINE_OFFSET,
    OUTLINE_WIDTH, RING_COLOR, RING_OFFSET_COLOR, RING_OFFSET_WIDTH, RING_WIDTH,
};

#[derive(Debug)]
pub enum BorderRadius<'a> {
    Around(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
    TopLeft(&'a str),
    TopRight(&'a str),
    BottomRight(&'a str),
    BottomLeft(&'a str),
}

impl<'a> BorderRadius<'a> {
    pub fn new(args: &'a str) -> Option<Self> {
        let value = match get_class_name(args) {
            "t" => Self::Top(get_opt_args(args)),
            "r" => Self::Right(get_opt_args(args)),
            "b" => Self::Bottom(get_opt_args(args)),
            "l" => Self::Left(get_opt_args(args)),
            "tl" => Self::TopLeft(get_opt_args(args)),
            "tr" => Self::TopRight(get_opt_args(args)),
            "br" => Self::BottomRight(get_opt_args(args)),
            "bl" => Self::BottomLeft(get_opt_args(args)),
            "" => Self::Around(""),
            _ => Self::Around(args),
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::Around(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Single(format!("border-radius: {}", value))
            }
            Self::Top(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Double([
                    format!("border-top-left-radius: {}", value),
                    format!("border-top-right-radius: {}", value),
                ])
            }
            Self::Right(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Double([
                    format!("border-top-right-radius: {}", value),
                    format!("border-bottom-right-radius: {}", value),
                ])
            }
            Self::Bottom(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Double([
                    format!("border-bottom-right-radius: {}", value),
                    format!("border-bottom-left-radius: {}", value),
                ])
            }
            Self::Left(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Double([
                    format!("border-top-left-radius: {}", value),
                    format!("border-bottom-left-radius: {}", value),
                ])
            }
            Self::TopLeft(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Single(format!("border-top-left-radius: {}", value))
            }
            Self::TopRight(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Single(format!("border-top-right-radius: {}", value))
            }
            Self::BottomRight(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Single(format!("border-bottom-right-radius: {}", value))
            }
            Self::BottomLeft(b) => {
                let value = get_value(b, &BORDER_RADIUS)?;
                Decl::Single(format!("border-bottom-left-radius: {}", value))
            }
        };

        Some(value)
    }
}

#[derive(Debug)]
pub enum BorderWidth<'a> {
    Around(&'a str),
    X(&'a str),
    Y(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
}

impl<'a> BorderWidth<'a> {
    pub fn new(args: &'a str) -> Option<Self> {
        let value = match get_class_name(args) {
            "x" => Self::X(get_opt_args(args)),
            "y" => Self::Y(get_opt_args(args)),
            "t" => Self::Top(get_opt_args(args)),
            "r" => Self::Right(get_opt_args(args)),
            "b" => Self::Bottom(get_opt_args(args)),
            "l" => Self::Left(get_opt_args(args)),
            "border" => Self::Around(""),
            _ => {
                if BORDER_WIDTH.contains_key(get_opt_args(args)) {
                    Self::Around(args)
                } else {
                    return None;
                }
            }
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            Self::Around(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Single(format!("border-width: {}", value))
            }
            Self::X(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Double([
                    format!("border-left-width: {}", value),
                    format!("border-right-width: {}", value),
                ])
            }
            Self::Y(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Double([
                    format!("border-top-width: {}", value),
                    format!("border-bottom-width: {}", value),
                ])
            }
            Self::Top(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Single(format!("border-top-width: {}", value))
            }
            Self::Right(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Single(format!("border-right-width: {}", value))
            }
            Self::Bottom(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Single(format!("border-bottom-width: {}", value))
            }
            Self::Left(b) => {
                let value = get_value(b, &BORDER_WIDTH)?;
                Decl::Single(format!("border-left-width: {}", value))
            }
        };

        Some(value)
    }
}

#[derive(Debug)]
pub enum BorderColor<'a> {
    Around(&'a str),
    X(&'a str),
    Y(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
}

impl<'a> BorderColor<'a> {
    pub fn new(args: &'a str) -> Option<Self> {
        let contains_key = BORDER_COLOR.contains_key(get_opt_args(args));

        let value = match get_class_name(args) {
            "x" if contains_key => Self::X(get_opt_args(args)),
            "y" if contains_key => Self::Y(get_opt_args(args)),
            "t" if contains_key => Self::Top(get_opt_args(args)),
            "r" if contains_key => Self::Right(get_opt_args(args)),
            "b" if contains_key => Self::Bottom(get_opt_args(args)),
            "l" if contains_key => Self::Left(get_opt_args(args)),
            _ => {
                if BORDER_COLOR.contains_key(args) || value_is_hex(args) {
                    Self::Around(args)
                } else {
                    return None;
                }
            }
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let arg = match self {
            BorderColor::Around(arg) => arg,
            BorderColor::X(arg) => arg,
            BorderColor::Y(arg) => arg,
            BorderColor::Top(arg) => arg,
            BorderColor::Right(arg) => arg,
            BorderColor::Bottom(arg) => arg,
            BorderColor::Left(arg) => arg,
        };

        let value = get_value(arg, &BORDER_COLOR)?;

        if let Some(color) = hex_to_rgb_color(&value) {
            let decl = match self {
                BorderColor::Around(_) => Decl::Double([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
                BorderColor::X(_) => Decl::Triple([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-left-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                    format!(
                        "border-right-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
                BorderColor::Y(_) => Decl::Triple([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-top-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                    format!(
                        "border-bottom-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
                BorderColor::Top(_) => Decl::Double([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-top-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
                BorderColor::Right(_) => Decl::Double([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-right-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
                BorderColor::Bottom(_) => Decl::Double([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-bottom-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
                BorderColor::Left(_) => Decl::Double([
                    "--tw-border-opacity: 1".into(),
                    format!(
                        "border-left-color: rgb({} {} {} / var(--tw-border-opacity))",
                        color[0], color[1], color[2]
                    ),
                ]),
            };

            Some(decl)
        } else {
            match self {
                BorderColor::Around(_) => Some(Decl::Single(format!("border-color: {}", value))),
                BorderColor::X(_) => Some(Decl::Double([
                    format!("border-left-color: {}", value),
                    format!("border-right-color: {}", value),
                ])),
                BorderColor::Y(_) => Some(Decl::Double([
                    format!("border-top-color: {}", value),
                    format!("border-bottom-color: {}", value),
                ])),
                BorderColor::Top(_) => Some(Decl::Single(format!("border-top-color: {}", value))),
                BorderColor::Right(_) => {
                    Some(Decl::Single(format!("border-right-color: {}", value)))
                }
                BorderColor::Bottom(_) => {
                    Some(Decl::Single(format!("border-bottom-color: {}", value)))
                }
                BorderColor::Left(_) => Some(Decl::Single(format!("border-left-color: {}", value))),
            }
        }
    }
}

#[derive(Debug)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
    None,
}

impl BorderStyle {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "solid" => Self::Solid,
            "dashed" => Self::Dashed,
            "dotted" => Self::Dotted,
            "double" => Self::Double,
            "hidden" => Self::Hidden,
            "none" => Self::None,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
            Self::Hidden => "hidden",
            Self::None => "none",
        };

        Decl::Single(format!("border-style: {}", val))
    }
}

#[derive(Debug)]
pub enum DivideWidth<'a> {
    X(&'a str),
    Y(&'a str),
    ReverseX,
    ReverseY,
}

impl<'a> DivideWidth<'a> {
    pub fn new(arg: &'a str) -> Option<Self> {
        match get_class_name(arg) {
            "x" => {
                if let Some(value) = get_args(arg) {
                    if value == "reverse" {
                        Some(Self::ReverseX)
                    } else {
                        Some(Self::X(value))
                    }
                } else {
                    Some(Self::X(""))
                }
            }
            "y" => {
                if let Some(value) = get_args(arg) {
                    if value == "reverse" {
                        Some(Self::ReverseY)
                    } else {
                        Some(Self::Y(value))
                    }
                } else {
                    Some(Self::Y(""))
                }
            }
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        dbg!(&self);
        let val = match self {
            Self::X(m) => {
                let value = get_value(m, &DIVIDE_WIDTH)?;
                Decl::Triple([
                    "--tw-divide-x-reverse: 0".into(),
                    format!(
                        "border-right-width: calc({} * var(--tw-divide-x-reverse))",
                        value
                    ),
                    format!(
                        "border-left-width: calc({} * calc(1 - var(--tw-divide-x-reverse)))",
                        value
                    ),
                ])
            }
            Self::Y(m) => {
                let value = get_value(m, &DIVIDE_WIDTH)?;
                Decl::Triple([
                    "--tw-divide-y-reverse: 0".into(),
                    format!(
                        "border-top-width: calc({} * calc(1 - var(--tw-divide-y-reverse)))",
                        value
                    ),
                    format!(
                        "border-bottom-width: calc({} * var(--tw-divide-y-reverse))",
                        value
                    ),
                ])
            }
            Self::ReverseX => Decl::Lit("--tw-divide-x-reverse: 1"),
            Self::ReverseY => Decl::Lit("--tw-divide-y-reverse: 1"),
        };

        Some(val)
    }
}

#[derive(Debug)]
pub struct DivideColor<'a>(pub &'a str);

impl<'a> DivideColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &DIVIDE_COLOR)?;
        Some(Decl::Single(format!("border-color: {}", value)))
    }
}

#[derive(Debug)]
pub enum DivideStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    None,
}

impl DivideStyle {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "solid" => Self::Solid,
            "dashed" => Self::Dashed,
            "dotted" => Self::Dotted,
            "double" => Self::Double,
            "none" => Self::None,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
            Self::None => "none",
        };

        Decl::Single(format!("border-style: {}", val))
    }
}

#[derive(Debug)]
pub struct OutlineWidth<'a>(pub &'a str);

impl<'a> OutlineWidth<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &OUTLINE_WIDTH)?;
        Some(Decl::Single(format!("outline-width: {}", value)))
    }
}

#[derive(Debug)]
pub struct OutlineColor<'a>(pub &'a str);

impl<'a> OutlineColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &DIVIDE_COLOR)?;
        Some(Decl::Single(format!("outline-color: {}", value)))
    }
}

#[derive(Debug)]
pub enum OutlineStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

impl OutlineStyle {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "none" => Self::None,
            "outline" => Self::Solid,
            "dashed" => Self::Dashed,
            "dotted" => Self::Dotted,
            "double" => Self::Double,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::None => {
                return Decl::Double([
                    "outline: 2px solid transparent".into(),
                    "outline-offset: 2px".into(),
                ])
            }
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
        };

        Decl::Single(format!("border-style: {}", val))
    }
}

#[derive(Debug)]
pub struct OutlineOffset<'a>(pub &'a str);

impl<'a> OutlineOffset<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &OUTLINE_OFFSET)?;
        Some(Decl::Single(format!("outline-offset: {}", value)))
    }
}

#[derive(Debug)]
pub enum RingWidth<'a> {
    Value(&'a str),
    Inset,
}

impl<'a> RingWidth<'a> {
    pub fn new(arg: &'a str) -> Option<Self> {
        let value = match get_class_name(arg) {
            "ring" => Self::Value(""),
            "inset" => Self::Inset,
            _ => {
                if RING_WIDTH.contains_key(arg) {
                    Self::Value(arg)
                } else {
                    return None;
                }
            }
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let val = match self {
            Self::Value(r) => {
                let value = get_value(r, &RING_WIDTH)?;
                Decl::Single(format!("box-shadow: {}", value))
            }
            Self::Inset => Decl::Lit("--tw-ring-inset: inset"),
        };

        Some(val)
    }
}

#[derive(Debug)]
pub struct RingColor<'a>(pub &'a str);

impl<'a> RingColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &RING_COLOR)?;
        Some(Decl::Single(format!("--tw-ring-color: {}", value)))
    }
}

#[derive(Debug)]
pub struct RingOffsetWidth<'a>(pub &'a str);

impl<'a> RingOffsetWidth<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &RING_OFFSET_WIDTH)?;
        Some(Decl::Double([
            format!("--tw-ring-offset-width: {}", value),
            "box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)".into(),
        ]))
    }
}

#[derive(Debug)]
pub struct RingOffsetColor<'a>(pub &'a str);

impl<'a> RingOffsetColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &RING_OFFSET_COLOR)?;
        Some(Decl::Double([
            format!("--tw-ring-offset-color: {}", value),
            "box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)".into(),
        ]))
    }
}
