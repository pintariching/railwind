use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref OBJECT_POSITION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("object_position.ron")).unwrap();
}

pub fn parse_object_position(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("object", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(value) = get_value(args[0], &OBJECT_POSITION) {
            ret_single_decl!("object-position", value.replace("_", " "))
        }

        warnings.push(WarningType::ValueNotFound(args[0].into()));
    }

    None
}
