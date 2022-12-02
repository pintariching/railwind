use crate::class::utils::{get_tuple_value, get_value};
use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

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
            "t" => Self::Top(get_args(args)?),
            "r" => Self::Right(get_args(args)?),
            "b" => Self::Bottom(get_args(args)?),
            "l" => Self::Left(get_args(args)?),
            "tl" => Self::TopLeft(get_args(args)?),
            "tr" => Self::TopRight(get_args(args)?),
            "br" => Self::BottomRight(get_args(args)?),
            "bl" => Self::BottomLeft(get_args(args)?),
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
                    format!("border-bottom-left-radius: {}", value),
                    format!("border-bottom-right-radius: {}", value),
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
            "x" => Self::X(get_args(args)?),
            "y" => Self::Y(get_args(args)?),
            "t" => Self::Top(get_args(args)?),
            "r" => Self::Right(get_args(args)?),
            "b" => Self::Bottom(get_args(args)?),
            "l" => Self::Left(get_args(args)?),

            _ => {
                if BORDER_WIDTH.contains_key(get_args(args)?) {
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
pub struct BorderColor<'a>(pub &'a str);

impl<'a> BorderColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BORDER_COLOR)?;
        Some(Decl::Single(format!("border-color: {}", value)))
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
                if arg == "x" {
                    Some(Self::X(""))
                } else {
                    Some(Self::X(get_args(arg)?))
                }
            }
            "y" => {
                if arg == "y" {
                    Some(Self::Y(""))
                } else {
                    Some(Self::Y(get_args(arg)?))
                }
            }
            "x-reverse" => Some(Self::ReverseX),
            "y-reverse" => Some(Self::ReverseY),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        let val = match self {
            Self::X(m) => {
                let value = get_tuple_value(m, &DIVIDE_WIDTH)?;
                Decl::Double([
                    format!("border-right-width: {}", value.0),
                    format!("border-left-width: {}", value.1),
                ])
            }
            Self::Y(m) => {
                let value = get_tuple_value(m, &DIVIDE_WIDTH)?;
                Decl::Double([
                    format!("border-top-width: {}", value.1),
                    format!("border-bottom-width    : {}", value.0),
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
