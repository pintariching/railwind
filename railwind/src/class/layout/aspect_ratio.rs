use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ASPECT_RATIO: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("aspect_ratio.ron")).unwrap();
}

pub fn parse_aspect_ratio(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("aspect", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(aspect_ratio) = get_value(args[0], &ASPECT_RATIO) {
            ret_single_decl!("aspect-ratio", aspect_ratio)
        }

        warnings.push(WarningType::ValueNotFound(args[0].into()));
    }

    None
}
