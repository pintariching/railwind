use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ORDER: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("order.ron")).unwrap();
}

pub fn parse_order(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("order", args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        if let Some(value) = get_value(args[0], &ORDER) {
            ret_single_decl!("order", value)
        }

        warnings.push(WarningType::ValueNotFound(
            format!("order-{}", args[0]),
            args[0].into(),
        ));
    }

    None
}
