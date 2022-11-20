use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COLUMNS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("columns.ron")).unwrap();
}

pub fn parse_columns(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Vec<String>> {
    max_arg_count("columns", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(columns) = COLUMNS.get(args[0]) {
            return Some(vec![format!("columns: {}", columns)]);
        }

        warnings.push(WarningType::ValueNotFound(
            format!("columns-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
