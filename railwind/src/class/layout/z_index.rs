use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref Z_INDEX: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("z_index.ron")).unwrap();
}

pub fn parse_z_index(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Vec<String>> {
    max_arg_count("z", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(columns) = Z_INDEX.get(args[0]) {
            return Some(vec![format!("z-index: {}", columns)]);
        }

        warnings.push(WarningType::ValueNotFound(
            format!("z-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
