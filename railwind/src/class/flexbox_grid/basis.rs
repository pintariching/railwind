use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BASIS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("basis.ron")).unwrap();
}

pub fn parse_basis(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Vec<String>> {
    max_arg_count("basis", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(columns) = BASIS.get(args[0]) {
            return Some(vec![format!("flex-basis: {}", columns)]);
        }

        warnings.push(WarningType::ValueNotFound(
            format!("basis-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
