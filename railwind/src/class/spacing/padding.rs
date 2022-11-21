use crate::class::{max_arg_count, min_arg_count};
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
) -> Option<Vec<String>> {
    max_arg_count(class_name, args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        match class_name {
            "p" => {
                if let Some(value) = PADDING.get(args[0]) {
                    return Some(vec![format!("padding: {}", value)]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
            }
            "pt" => return padding_single_dir(class_name, "top", args, warnings),
            "pr" => return padding_single_dir(class_name, "right", args, warnings),
            "pb" => return padding_single_dir(class_name, "bottom", args, warnings),
            "pl" => return padding_single_dir(class_name, "left", args, warnings),
            "px" => {
                if let Some(value) = PADDING.get(args[0]) {
                    return Some(vec![
                        format!("padding-left: {}", value),
                        format!("padding-right: {}", value),
                    ]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
            }
            "py" => {
                if let Some(value) = PADDING.get(args[0]) {
                    return Some(vec![
                        format!("padding-top: {}", value),
                        format!("padding-bottom: {}", value),
                    ]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
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
) -> Option<Vec<String>> {
    if let Some(value) = PADDING.get(args[0]) {
        return Some(vec![format!("padding-{}: {}", dir, value)]);
    }

    warnings.push(WarningType::ValueNotFound(
        format!("{}-{}", class_name, args[0]),
        args[0].into(),
    ));

    None
}
