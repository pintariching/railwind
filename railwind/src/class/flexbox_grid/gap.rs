use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GAP: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("gap.ron")).unwrap();
}

pub fn parse_gap(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("gap", args, 2, warnings);

    match args[0] {
        "x" => {
            if let Some(gap_x) = get_value(args[1], &GAP) {
                ret_single_decl!("column-gap", gap_x)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()))
        }
        "y" => {
            if let Some(gap_y) = get_value(args[1], &GAP) {
                ret_single_decl!("row-gap", gap_y)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()))
        }
        _ => {
            if let Some(gap) = get_value(args[0], &GAP) {
                ret_single_decl!("gap", gap)
            }

            warnings.push(WarningType::ValueNotFound(args[0].into()))
        }
    }

    None
}
