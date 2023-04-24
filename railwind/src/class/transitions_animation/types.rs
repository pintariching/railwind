use crate::class::utils::get_arbitrary_value;
use crate::class::Decl;
use crate::{class::utils::get_value, warning::WarningType};

use super::{ANIMATION, DELAY, DURATION, TIMING_FUNCTION};

#[derive(Debug, PartialEq, Hash)]
pub enum Transition {
    None,
    All,
    Transition,
    Colors,
    Opacity,
    Shadow,
    Transform,
    Arbitrary(String),
}

impl Transition {
    pub fn new(value: &str) -> Result<Self, WarningType> {
        match value {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            "" => Ok(Self::Transition),
            "colors" => Ok(Self::Colors),
            "opacity" => Ok(Self::Opacity),
            "shadow" => Ok(Self::Shadow),
            "transform" => Ok(Self::Transform),
            _ => {
                if let Some(arbitrary) = get_arbitrary_value(value) {
                    Ok(Self::Arbitrary(arbitrary))
                } else {
                    Err(WarningType::InvalidArg(
                        value.into(),
                        "Transition".into(),
                        vec![
                            "none",
                            "all",
                            "",
                            "colors",
                            "opacity",
                            "shadow",
                            "transform",
                        ],
                    ))
                }
            }
        }
    }
    pub fn to_decl(self) -> Decl {
        match self {
            Self::None => Decl::String("transition-property: none".into()),
            Self::All => Decl::Triple([
                "transition-property: all".into(),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
            Self::Transition => Decl::Vec(vec![
                "transition-property: color, background-color, border-color, outline-color, fill, stroke, opacity, box-shadow, transform, filter, -webkit-text-decoration-color, -webkit-backdrop-filter".into(),
                "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter".into(),
                "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter, -webkit-text-decoration-color, -webkit-backdrop-filter".into(),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
            Self::Colors => Decl::Vec(vec![
                "transition-property: color, background-color, border-color, outline-color, fill, stroke, -webkit-text-decoration-color".into(),
                "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke".into(),
                "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, -webkit-text-decoration-color".into(),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
            Self::Opacity => Decl::Triple([
                "transition-property: opacity".into(),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
            Self::Shadow => Decl::Triple([
                "transition-property: box-shadow".into(),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
            Self::Transform => Decl::Triple([
                "transition-property: transform".into(),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
            Self::Arbitrary(v) => Decl::Triple([
                format!("transition-property: {v}"),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Duration<'a>(pub &'a str);

impl<'a> Duration<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &DURATION)?;
        Ok(Decl::String(format!("transition-duration: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct TimingFunction<'a>(pub &'a str);

impl<'a> TimingFunction<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TIMING_FUNCTION)?;
        Ok(Decl::String(format!(
            "transition-timing-function: {}",
            value
        )))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Delay<'a>(pub &'a str);

impl<'a> Delay<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &DELAY)?;
        Ok(Decl::String(format!("transition-delay: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Animation<'a>(pub &'a str);

impl<'a> Animation<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &ANIMATION)?;

        if value.as_str() == "none" {
            Ok(Decl::String(format!("animation: {value}")))
        } else {
            Ok(Decl::FullClass(value))
        }
    }
}
