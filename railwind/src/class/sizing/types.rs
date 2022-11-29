use crate::class::utils::get_value;
use crate::class::Decl;

use super::{HEIGHT, MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH, WIDTH};

#[derive(Debug)]
pub struct Width<'a>(pub &'a str);

impl<'a> Width<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &WIDTH)?;
        Some(Decl::Single(format!("width: {}", value)))
    }
}

#[derive(Debug)]
pub struct MinWidth<'a>(pub &'a str);

impl<'a> MinWidth<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &MIN_WIDTH)?;
        Some(Decl::Single(format!("min-width: {}", value)))
    }
}

#[derive(Debug)]
pub struct MaxWidth<'a>(pub &'a str);

impl<'a> MaxWidth<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &MAX_WIDTH)?;
        Some(Decl::Single(format!("max-width: {}", value)))
    }
}

#[derive(Debug)]
pub struct Height<'a>(pub &'a str);

impl<'a> Height<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &HEIGHT)?;
        Some(Decl::Single(format!("height: {}", value)))
    }
}

#[derive(Debug)]
pub struct MinHeight<'a>(pub &'a str);

impl<'a> MinHeight<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &MIN_HEIGHT)?;
        Some(Decl::Single(format!("min-height: {}", value)))
    }
}

#[derive(Debug)]
pub struct MaxHeight<'a>(pub &'a str);

impl<'a> MaxHeight<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &MAX_HEIGHT)?;
        Some(Decl::Single(format!("max-height: {}", value)))
    }
}
