use crate::class::utils::get_value;
use crate::class::Decl;
use crate::utils::{get_class_name, get_opt_args};

use super::BORDER_SPACING;

#[derive(Debug)]
pub enum BorderCollapse {
    Collapse,
    Separate,
}

impl BorderCollapse {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "collapse" => Self::Collapse,
            "separate" => Self::Separate,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Collapse => "collapse",
            Self::Separate => "separate",
        };

        Decl::Single(format!("border-collapse: {}", val))
    }
}

#[derive(Debug)]
pub enum BorderSpacing<'a> {
    All(&'a str),
    X(&'a str),
    Y(&'a str),
}

impl<'a> BorderSpacing<'a> {
    pub fn new(args: &'a str) -> Option<Self> {
        let value = match get_class_name(args) {
            "x" => Self::X(get_opt_args(args)),
            "y" => Self::Y(get_opt_args(args)),
            _ => {
                if BORDER_SPACING.contains_key(get_opt_args(args)) {
                    Self::All(args)
                } else {
                    return None;
                }
            }
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::X(s) => {
                let value = get_value(s, &BORDER_SPACING)?;
                Some(Decl::Double([
                    format!("--tw-border-spacing-x: {}", value),
                    "border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y)".into(),
                ]))
            }
            Self::Y(s) => {
                let value = get_value(s, &BORDER_SPACING)?;
                Some(Decl::Double([
                    format!("--tw-border-spacing-y: {}", value),
                    "border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y)".into(),
                ]))
            }
            Self::All(s) => {
                let value = get_value(s, &BORDER_SPACING)?;
                Some(Decl::Triple([
                    format!("--tw-border-spacing-x: {}", value),
                    format!("--tw-border-spacing-y: {}", value),
                    "border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y)".into(),
                ]))
            }
        }
    }
}

#[derive(Debug)]
pub enum Layout {
    Auto,
    Fixed,
}

impl Layout {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "auto" => Self::Auto,
            "fixed" => Self::Fixed,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Fixed => "fixed",
        };

        Decl::Single(format!("table-layout: {}", val))
    }
}
