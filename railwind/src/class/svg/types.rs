use crate::class::utils::get_value;
use crate::class::Decl;
use crate::warning::WarningType;

use super::COLORS;

#[derive(Debug)]
pub struct Fill<'a>(pub &'a str);

impl<'a> Fill<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLORS)?;
        Ok(Decl::Single(format!("fill: {}", value)))
    }
}

#[derive(Debug)]
pub struct Stroke<'a>(pub &'a str);

impl<'a> Stroke<'a> {
    pub fn new(arg: &'a str) -> Self {
        Self(arg)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLORS)?;
        Ok(Decl::Single(format!("stroke: {}", value)))
    }
}

#[derive(Debug)]
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
            StrokeWidth::Zero => "0",
            StrokeWidth::One => "1",
            StrokeWidth::Two => "2",
        };

        Decl::Single(format!("stroke-width: {}", val))
    }
}
