use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ASPECT_RATIO: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("aspect_ratio.ron")).unwrap();
}

pub fn parse_aspect_ratio(
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    max_arg_count("aspect", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(aspect_ratio) = ASPECT_RATIO.get(args[0]) {
            return Some(vec![format!("aspect-ratio: {}", aspect_ratio)]);
        }

        warnings.push(WarningType::ValueNotFound(
            format!("aspect-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
