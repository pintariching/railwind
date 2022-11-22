use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BASIS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("basis.ron")).unwrap();
}

pub fn parse_basis(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("basis", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(value) = get_value(args[0], &BASIS) {
            ret_single_decl!("flex-basis", value)
        }

        warnings.push(WarningType::ValueNotFound(
            format!("basis-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
