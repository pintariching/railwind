mod aspect_ratio;
mod container;

use aspect_ratio::AspectRatio;

use crate::{traits::IntoDeclaration, warning::WarningType};

#[derive(Debug, PartialEq)]
pub enum Layout {
    AspectRatio(AspectRatio),
}

impl Layout {
    pub fn new(class_name: &str, args: &[&str; 3]) -> Result<Self, WarningType> {
        match class_name {
            "aspect" => Ok(Layout::AspectRatio(AspectRatio::new(args)?)),
            _ => Err(WarningType::ClassNotFound),
        }
    }
}

impl IntoDeclaration for Layout {
    fn into_decl(&self) -> Vec<String> {
        match self {
            Layout::AspectRatio(ar) => ar.into_decl(),
        }
    }
}
