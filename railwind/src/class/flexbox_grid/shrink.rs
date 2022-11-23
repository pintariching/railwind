use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl, TOP_RIGHT_BOTTOM_LEFT};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref SHRINK: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("shrink.ron")).unwrap();
}

pub fn parse_shrink(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("shrink", args, 1, warnings);

    match args[0] {
        "" => {
            max_arg_count("shrink", args, 0, warnings);
            ret_lit!("flex-shrink: 1")
        }
        _ => {
            if let Some(shrink) = get_value(args[0], &SHRINK) {
                ret_single_decl!("shrink", shrink)
            }

            warnings.push(WarningType::ValueNotFound(
                format!("shrink-{}", args[0]),
                args[0].into(),
            ));
        }
    }

    None
}
