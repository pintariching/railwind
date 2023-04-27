use crate::class::utils::get_value;
use crate::class::Decl;
use crate::warning::WarningType;

use super::COLORS;

#[derive(Debug, PartialEq, Hash)]
pub struct Fill<'a>(pub &'a str);

impl<'a> Fill<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLORS)?;
        Ok(Decl::String(format!("fill: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Stroke<'a>(pub &'a str);

impl<'a> Stroke<'a> {
    pub fn new(arg: &'a str) -> Self {
        Self(arg)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLORS)?;
        Ok(Decl::String(format!("stroke: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum StrokeWidth {
    Zero,
    One,
    Two,
}

impl StrokeWidth {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "0" => Self::Zero,
            "1" => Self::One,
            "2" => Self::Two,
            _ => return None,
        };
        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
        };

        Decl::String(format!("stroke-width: {}", val))
    }
}
