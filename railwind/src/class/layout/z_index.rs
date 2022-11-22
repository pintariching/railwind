use crate::class::utils::get_value_neg;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref Z_INDEX: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("z_index.ron")).unwrap();
}

pub fn parse_z_index(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    max_arg_count("z", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(z_index) = get_value_neg(class_name, args[0], &Z_INDEX) {
            ret_single_decl!("z-index", z_index)
        }

        warnings.push(WarningType::ValueNotFound(
            format!("z-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
