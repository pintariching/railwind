use crate::class::utils::get_value_neg;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MARGIN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("margin.ron")).unwrap();
}

pub fn parse_margin(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    max_arg_count(class_name, args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        match class_name {
            "m" | "-m" => {
                if let Some(value) = get_value_neg(class_name, args[0], &MARGIN) {
                    ret_single_decl!("margin", value)
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            "mt" | "-mt" => return margin_single_dir(class_name, "top", args, warnings),
            "mr" | "-mr" => return margin_single_dir(class_name, "right", args, warnings),
            "mb" | "-mb" => return margin_single_dir(class_name, "bottom", args, warnings),
            "ml" | "-ml" => return margin_single_dir(class_name, "left", args, warnings),
            "mx" | "-mx" => {
                if let Some(value) = get_value_neg(class_name, args[0], &MARGIN) {
                    return Some(Decl::Double([
                        format!("margin-left: {}", value),
                        format!("margin-right: {}", value),
                    ]));
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            "my" | "-my" => {
                if let Some(value) = get_value_neg(class_name, args[0], &MARGIN) {
                    return Some(Decl::Double([
                        format!("margin-top: {}", value),
                        format!("margin-bottom: {}", value),
                    ]));
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            _ => warnings.push(WarningType::InvalidArg(
                format!("{}-{}", class_name, args[0]),
                vec!["mt", "mr", "mb", "ml", "mx", "my"],
            )),
        }
    }

    None
}

fn margin_single_dir(
    class_name: &str,
    dir: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    if let Some(value) = get_value_neg(class_name, args[0], &MARGIN) {
        return Some(Decl::Single(format!("margin-{}: {}", dir, value)));
    }

    warnings.push(WarningType::ValueNotFound(args[0].into()));

    None
}
