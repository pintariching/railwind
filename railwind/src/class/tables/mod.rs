mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BORDER_SPACING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("border_spacing.ron")).unwrap();
}

#[derive(Debug)]
pub enum Table<'a> {
    BorderSpacing(BorderSpacing<'a>),
    BorderCollapse(BorderCollapse),
    Layout(Layout),
}

impl<'a> Table<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let args = if let Ok(str) = get_args(value) {
            str
        } else {
            return Ok(None);
        };

        let class_name = get_class_name(value);

        let table = match class_name {
            "border" => match get_class_name(args) {
                "collapse" => Table::BorderCollapse(BorderCollapse::Collapse),
                "separate" => Table::BorderCollapse(BorderCollapse::Separate),
                "spacing" => Table::BorderSpacing(BorderSpacing::new(get_args(args)?)),
                _ => return Ok(None),
            },
            "table" => Table::Layout(Layout::new(args)?),
            _ => return Ok(None),
        };

        Ok(Some(table))
    }
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Table::BorderSpacing(t) => t.to_decl(),
            Table::BorderCollapse(t) => Ok(t.to_decl()),
            Table::Layout(t) => Ok(t.to_decl()),
        }
    }
}
