use crate::class::{max_arg_count, min_arg_count};
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
) -> Option<Vec<String>> {
    max_arg_count(class_name, args, 1, warnings);

    if min_arg_count(args, 1, warnings) {
        let (negative, cls) = if class_name.starts_with('-') {
            (true, &class_name[1..])
        } else {
            (false, class_name)
        };

        match cls {
            "m" => {
                if let Some(value) = get_value(args[0], negative) {
                    return Some(vec![format!("margin: {}", value)]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
            }
            "mt" => return margin_single_dir(cls, "top", args, warnings),
            "mr" => return margin_single_dir(cls, "right", args, warnings),
            "mb" => return margin_single_dir(cls, "bottom", args, warnings),
            "ml" => return margin_single_dir(cls, "left", args, warnings),
            "mx" => {
                if let Some(value) = get_value(args[0], negative) {
                    return Some(vec![
                        format!("margin-left: {}", value),
                        format!("margin-right: {}", value),
                    ]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
            }
            "my" => {
                if let Some(value) = get_value(args[0], negative) {
                    return Some(vec![
                        format!("margin-top: {}", value),
                        format!("margin-bottom: {}", value),
                    ]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
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
) -> Option<Vec<String>> {
    if let Some(value) = MARGIN.get(args[0]) {
        return Some(vec![format!("margin-{}: {}", dir, value)]);
    }

    warnings.push(WarningType::ValueNotFound(
        format!("{}-{}", class_name, args[0]),
        args[0].into(),
    ));

    None
}

fn get_value(arg: &str, negative: bool) -> Option<String> {
    if let Some(value) = MARGIN.get(arg) {
        return Some(format!("{}{}", if negative { "-" } else { "" }, value));
    }
    None
}
