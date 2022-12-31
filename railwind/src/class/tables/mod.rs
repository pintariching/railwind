mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

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
    pub fn new(value: &'a str) -> Option<Self> {
        let args = get_args(value)?;
        let class_name = get_class_name(value);

        let table = match class_name {
            "border" => match get_class_name(args) {
                "collapse" | "separate" => {
                    Table::BorderCollapse(BorderCollapse::new(get_class_name(args))?)
                }
                "spacing" => Table::BorderSpacing(BorderSpacing::new(get_args(args)?)?),
                _ => return None,
            },
            "table" => Table::Layout(Layout::new(args)?),
            _ => return None,
        };
        Some(table)
    }
    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Table::BorderSpacing(t) => t.to_decl(),
            Table::BorderCollapse(t) => Some(t.to_decl()),
            Table::Layout(t) => Some(t.to_decl()),
        }
    }
}
