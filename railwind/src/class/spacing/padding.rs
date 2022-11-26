use crate::class::utils::get_value;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PADDING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("padding.ron")).unwrap();
}

pub fn parse_padding(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    max_arg_count(class_name, args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        match class_name {
            "p" => {
                if let Some(value) = get_value(args[0], &PADDING) {
                    ret_single_decl!("padding", value)
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            "pt" => return padding_single_dir(class_name, "top", args, warnings),
            "pr" => return padding_single_dir(class_name, "right", args, warnings),
            "pb" => return padding_single_dir(class_name, "bottom", args, warnings),
            "pl" => return padding_single_dir(class_name, "left", args, warnings),
            "px" => {
                if let Some(value) = get_value(args[0], &PADDING) {
                    return Some(Decl::Double([
                        format!("padding-left: {}", value),
                        format!("padding-right: {}", value),
                    ]));
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            "py" => {
                if let Some(value) = get_value(args[0], &PADDING) {
                    return Some(Decl::Double([
                        format!("padding-top: {}", value),
                        format!("padding-bottom: {}", value),
                    ]));
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            _ => warnings.push(WarningType::InvalidArg(
                format!("{}-{}", class_name, args[0]),
                vec!["pt", "pr", "pb", "pl", "px", "py"],
            )),
        }
    }

    None
}

fn padding_single_dir(
    class_name: &str,
    dir: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    if let Some(value) = get_value(args[0], &PADDING) {
        return Some(Decl::Single(format!("padding-{}: {}", dir, value)));
    }

    warnings.push(WarningType::ValueNotFound(args[0].into()));

    None
}
