use crate::class::utils::get_value_neg;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref SPACE_BETWEEN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("space_between.ron")).unwrap();
}

pub fn parse_space_between(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    max_arg_count(class_name, args, 2, warnings);

    if min_arg_count(args, 2, warnings) {
        match args[0] {
            "x" => {
                if let Some(value) = get_value_neg(class_name, args[1], &SPACE_BETWEEN) {
                    ret_single_decl!("margin-left", value)
                }

                if args[1] == "reverse" {
                    ret_lit!("--tw-space-x-reverse: 1")
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            "y" => {
                if let Some(value) = get_value_neg(class_name, args[1], &SPACE_BETWEEN) {
                    ret_single_decl!("margin-top", value)
                }

                if args[1] == "reverse" {
                    ret_lit!("--tw-space-y-reverse: 1")
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            _ => warnings.push(WarningType::InvalidArg(
                format!("{}-{}", class_name, args[0]),
                vec!["x", "y"],
            )),
        }
    }

    None
}
