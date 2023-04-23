use crate::class::utils::get_value;
use crate::class::Decl;
use crate::utils::{get_class_name, get_opt_args};
use crate::warning::WarningType;

use super::BORDER_SPACING;

#[derive(Debug, PartialEq, Hash)]
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

#[derive(Debug, PartialEq, Hash)]
pub enum BorderSpacing<'a> {
    All(&'a str),
    X(&'a str),
    Y(&'a str),
}

impl<'a> BorderSpacing<'a> {
    pub fn new(args: &'a str) -> Self {
        match get_class_name(args) {
            "x" => Self::X(get_opt_args(args)),
            "y" => Self::Y(get_opt_args(args)),
            _ => Self::All(args),
        }
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::X(s) => {
                let value = get_value(s, &BORDER_SPACING)?;
                Ok(Decl::Double([
                    format!("--tw-border-spacing-x: {}", value),
                    "border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y)".into(),
                ]))
            }
            Self::Y(s) => {
                let value = get_value(s, &BORDER_SPACING)?;
                Ok(Decl::Double([
                    format!("--tw-border-spacing-y: {}", value),
                    "border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y)".into(),
                ]))
            }
            Self::All(s) => {
                let value = get_value(s, &BORDER_SPACING)?;
                Ok(Decl::Triple([
                    format!("--tw-border-spacing-x: {}", value),
                    format!("--tw-border-spacing-y: {}", value),
                    "border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y)".into(),
                ]))
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Layout {
    Auto,
    Fixed,
}

impl Layout {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "auto" => Ok(Self::Auto),
            "fixed" => Ok(Self::Fixed),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Layout".into(),
                vec!["auto", "fixed"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Fixed => "fixed",
        };

        Decl::Single(format!("table-layout: {}", val))
    }
}
