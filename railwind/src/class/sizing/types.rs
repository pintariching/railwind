use crate::class::Decl;
use crate::{class::utils::get_value, warning::WarningType};

use super::{HEIGHT, MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH, WIDTH};

#[derive(Debug, PartialEq, Hash)]
pub struct Width<'a>(pub &'a str);

impl<'a> Width<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &WIDTH)?;

        if value == "fit-content" {
            return Ok(Decl::Double([
                "width: -moz-fit-content".into(),
                format!("width: {}", value),
            ]));
        }

        Ok(Decl::Single(format!("width: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct MinWidth<'a>(pub &'a str);

impl<'a> MinWidth<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &MIN_WIDTH)?;

        if value == "fit-content" {
            return Ok(Decl::Double([
                "min-width: -moz-fit-content".into(),
                format!("min-width: {}", value),
            ]));
        }

        Ok(Decl::Single(format!("min-width: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct MaxWidth<'a>(pub &'a str);

impl<'a> MaxWidth<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &MAX_WIDTH)?;

        if value == "fit-content" {
            return Ok(Decl::Double([
                "max-width: -moz-fit-content".into(),
                format!("max-width: {}", value),
            ]));
        }

        Ok(Decl::Single(format!("max-width: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Height<'a>(pub &'a str);

impl<'a> Height<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &HEIGHT)?;

        if value == "fit-content" {
            return Ok(Decl::Double([
                "height: -moz-fit-content".into(),
                format!("height: {}", value),
            ]));
        }

        Ok(Decl::Single(format!("height: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct MinHeight<'a>(pub &'a str);

impl<'a> MinHeight<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &MIN_HEIGHT)?;

        if value == "fit-content" {
            return Ok(Decl::Double([
                "min-height: -moz-fit-content".into(),
                format!("min-height: {}", value),
            ]));
        }

        Ok(Decl::Single(format!("min-height: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct MaxHeight<'a>(pub &'a str);

impl<'a> MaxHeight<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &MAX_HEIGHT)?;

        if value == "fit-content" {
            return Ok(Decl::Double([
                "max-height: -moz-fit-content".into(),
                format!("max-height: {}", value),
            ]));
        }

        Ok(Decl::Single(format!("max-height: {}", value)))
    }
}
