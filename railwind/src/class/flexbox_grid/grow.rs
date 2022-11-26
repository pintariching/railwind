use crate::class::utils::get_value;
use crate::class::{max_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GROW: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grow.ron")).unwrap();
}

pub fn parse_grow(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("grow", args, 1, warnings);

    match args[0] {
        "" => {
            max_arg_count("grow", args, 0, warnings);
            ret_lit!("flex-grow: 1")
        }
        _ => {
            if let Some(grow) = get_value(args[0], &GROW) {
                ret_single_decl!("grow", grow)
            }

            warnings.push(WarningType::ValueNotFound(args[0].into()));
        }
    }

    None
}
