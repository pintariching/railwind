use crate::class::utils::{get_value, get_value_neg};
use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

use super::{MARGIN, PADDING};

#[derive(Debug)]
pub enum Padding<'a> {
    All(&'a str),
    Top(&'a str),
    Right(&'a str),
    Bottom(&'a str),
    Left(&'a str),
    X(&'a str),
    Y(&'a str),
}

impl<'a> Padding<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Option<Self> {
        match name {
            "p" => Some(Self::All(arg)),
            "pt" => Some(Self::Top(arg)),
            "pr" => Some(Self::Right(arg)),
            "pb" => Some(Self::Bottom(arg)),
            "pl" => Some(Self::Left(arg)),
            "px" => Some(Self::X(arg)),
            "py" => Some(Self::Y(arg)),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::All(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("padding: {}", value)))
            }
            Self::Top(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("padding-top: {}", value)))
            }
            Self::Right(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("padding-right: {}", value)))
            }
            Self::Bottom(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("padding-bottom: {}", value)))
            }
            Self::Left(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Single(format!("padding-left: {}", value)))
            }
            Self::X(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Double([
                    format!("padding-left: {}", value),
                    format!("padding-right: {}", value),
                ]))
            }
            Self::Y(p) => {
                let value = get_value(p, &PADDING)?;
                Some(Decl::Double([
                    format!("padding-top: {}", value),
                    format!("padding-bottom: {}", value),
                ]))
            }
        }
    }
}

#[derive(Debug)]
pub enum Margin<'a> {
    All(&'a str, bool),
    Top(&'a str, bool),
    Right(&'a str, bool),
    Bottom(&'a str, bool),
    Left(&'a str, bool),
    X(&'a str, bool),
    Y(&'a str, bool),
}

impl<'a> Margin<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Option<Self> {
        let negative = name.starts_with('-');
        let name = if negative { &name[1..] } else { name };

        match name {
            "m" => Some(Self::All(arg, negative)),
            "mt" => Some(Self::Top(arg, negative)),
            "mr" => Some(Self::Right(arg, negative)),
            "mb" => Some(Self::Bottom(arg, negative)),
            "ml" => Some(Self::Left(arg, negative)),
            "mx" => Some(Self::X(arg, negative)),
            "my" => Some(Self::Y(arg, negative)),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::All(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("margin: {}", value)))
            }
            Self::Top(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("margin-top: {}", value)))
            }
            Self::Right(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("margin-right: {}", value)))
            }
            Self::Bottom(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("margin-bottom: {}", value)))
            }
            Self::Left(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Single(format!("margin-left: {}", value)))
            }
            Self::X(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Double([
                    format!("margin-left: {}", value),
                    format!("margin-right: {}", value),
                ]))
            }
            Self::Y(m, n) => {
                let value = get_value_neg(n, m, &MARGIN)?;
                Some(Decl::Double([
                    format!("margin-top: {}", value),
                    format!("margin-bottom: {}", value),
                ]))
            }
        }
    }
}

#[derive(Debug)]
pub enum SpaceBetween<'a> {
    X(&'a str, bool),
    Y(&'a str, bool),
}

impl<'a> SpaceBetween<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Option<Self> {
        let negative = name.starts_with('-');

        if !name.ends_with("space") {
            return None;
        }

        match get_class_name(arg) {
            "x" => Some(Self::X(get_args(arg)?, negative)),
            "y" => Some(Self::Y(get_args(arg)?, negative)),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::X(s, n) => {
                if s == "reverse" {
                    return Some(Decl::Lit("--tw-space-x-reverse: 1"));
                }

                let value = get_value_neg(n, s, &MARGIN)?;
                Some(Decl::Triple([
                    "--tw-space-x-reverse: 0".into(),
                    format!("margin-right: calc({} * var(--tw-space-x-reverse))", value),
                    format!(
                        "margin-left: calc({} * calc(1 - var(--tw-space-x-reverse)))",
                        value
                    ),
                ]))
            }
            Self::Y(s, n) => {
                if s == "reverse" {
                    return Some(Decl::Lit("--tw-space-y-reverse: 1"));
                }

                let value = get_value_neg(n, s, &MARGIN)?;
                Some(Decl::Triple([
                    "--tw-space-y-reverse: 0".into(),
                    format!(
                        "margin-top: calc({} * calc(1 - var(--tw-space-y-reverse)))",
                        value
                    ),
                    format!("margin-bottom: calc({} * var(--tw-space-y-reverse))", value),
                ]))
            }
        }
    }
}
