use crate::class::Decl;
use crate::{class::utils::get_value, warning::WarningType};

use super::{ANIMATION, DELAY, DURATION, TIMING_FUNCTION, TRANSITION};

#[derive(Debug)]
pub struct Transition<'a>(pub &'a str);

impl<'a> Transition<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TRANSITION)?;

        if value.as_str() == "none" {
            Ok(Decl::Single(format!("transition-property: {value}")))
        } else {
            Ok(Decl::Triple([
                format!("transition-property: {value}"),
                "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                "transition-duration: 150ms".into(),
            ]))
        }
    }
}

#[derive(Debug)]
pub struct Duration<'a>(pub &'a str);

impl<'a> Duration<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &DURATION)?;
        Ok(Decl::Single(format!("transition-duration: {}", value)))
    }
}

#[derive(Debug)]
pub struct TimingFunction<'a>(pub &'a str);

impl<'a> TimingFunction<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TIMING_FUNCTION)?;
        Ok(Decl::Single(format!(
            "transition-timing-function: {}",
            value
        )))
    }
}

#[derive(Debug)]
pub struct Delay<'a>(pub &'a str);

impl<'a> Delay<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &DELAY)?;
        Ok(Decl::Single(format!("transition-delay: {}", value)))
    }
}

#[derive(Debug)]
pub struct Animation<'a>(pub &'a str);

impl<'a> Animation<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &ANIMATION)?;

        if value.as_str() == "none" {
            Ok(Decl::Single(format!("animation: {value}")))
        } else {
            Ok(Decl::FullClass(value))
        }
    }
}
