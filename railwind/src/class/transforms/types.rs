use crate::class::utils::{get_value, get_value_neg};
use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};

use super::{ORIGIN, ROTATE, SCALE, SKEW, TRANSLATE};

const TRANSFORM_STYLE: &str = "transform: translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))";

#[derive(Debug)]
pub struct TranslateX<'a>(pub &'a str, bool);

impl<'a> TranslateX<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &TRANSLATE)?;
        Some(Decl::Double([
                format!("--tw-translate-x: {}", value),
                TRANSFORM_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct TranslateY<'a>(pub &'a str, bool);

impl<'a> TranslateY<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &TRANSLATE)?;
        Some(Decl::Double([
                format!("--tw-translate-y: {}", value),
                TRANSFORM_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct Rotate<'a>(pub &'a str, bool);

impl<'a> Rotate<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &ROTATE)?;
        Some(Decl::Double([
                format!("--tw-rotate: {}", value),
                TRANSFORM_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct SkewX<'a>(pub &'a str, bool);

impl<'a> SkewX<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &SKEW)?;
        Some(Decl::Double([
                format!("--tw-skew-x: {}", value),
                TRANSFORM_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub struct SkewY<'a>(pub &'a str, bool);

impl<'a> SkewY<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &SKEW)?;
        Some(Decl::Double([
                format!("--tw-skew-y: {}", value),
                TRANSFORM_STYLE.into(),
        ]))
    }
}

#[derive(Debug)]
pub enum Scale<'a> {
    All(&'a str, bool),
    X(&'a str, bool),
    Y(&'a str, bool),
}

impl<'a> Scale<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let negative = value.starts_with('-');
        let args = get_args(value)?;
        let value = match get_class_name(args) {
            "x" => Self::X(get_opt_args(args), negative),
            "y" => Self::Y(get_opt_args(args), negative),
            _ => Self::All(args, negative),
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::All(s, n) => {
                let value = get_value_neg(n, s, &SCALE)?;
                Some(Decl::Triple([
                   format!("--tw-scale-x: {}", value),
                   format!("--tw-scale-y: {}", value),
                   TRANSFORM_STYLE.into(),
                ]))
            }
            Self::X(s, n) => {
                let value = get_value_neg(n, s, &SCALE)?;
                Some(Decl::Double([
                   format!("--tw-scale-x: {}", value),
                   TRANSFORM_STYLE.into(),
                ]))
            }
            Self::Y(s, n) => {
                let value = get_value_neg(n, s, &SCALE)?;
                Some(Decl::Double([
                   format!("--tw-scale-y: {}", value),
                   TRANSFORM_STYLE.into(),
                ]))
            }
        }
    }
}

#[derive(Debug)]
pub struct Origin<'a>(pub &'a str);

impl<'a> Origin<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &ORIGIN)?;
        Some(Decl::Single(format!("transform-origin: {}", value)))
    }
}
