use crate::class::utils::get_value_neg;
use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TOP_RIGHT_BOTTOM_LEFT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("top_right_bottom_left.ron")).unwrap();
}

pub fn parse_top_right_bottom_left(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    if min_arg_count(args, 1, warnings) {
        match class_name {
            "top" | "right" | "bottom" | "left" => {
                // top-1 right-1 bottom-1 left-1
                if let Some(value) = get_value_neg(class_name, args[0], &TOP_RIGHT_BOTTOM_LEFT) {
                    max_arg_count(class_name, args, 1, warnings);
                    ret_single_decl!(class_name, value)
                }

                warnings.push(WarningType::ValueNotFound(args[0].into()))
            }
            "inset" => match args[0] {
                // inset-x-1 inset-y-1
                "x" => {
                    if min_arg_count(args, 2, warnings) {
                        if let Some(value) =
                            get_value_neg(class_name, args[1], &TOP_RIGHT_BOTTOM_LEFT)
                        {
                            max_arg_count(class_name, args, 3, warnings);
                            return Some(Decl::Double([
                                format!("left: {}", value),
                                format!("right: {}", value),
                            ]));
                        }

                        warnings.push(WarningType::ValueNotFound(args[1].into()))
                    }
                }
                "y" => {
                    if min_arg_count(args, 2, warnings) {
                        if let Some(value) =
                            get_value_neg(class_name, args[1], &TOP_RIGHT_BOTTOM_LEFT)
                        {
                            max_arg_count(class_name, args, 3, warnings);
                            return Some(Decl::Double([
                                format!("top: {}", value),
                                format!("bottom: {}", value),
                            ]));
                        }

                        warnings.push(WarningType::ValueNotFound(args[1].into()))
                    }
                }
                // inset-1
                _ => {
                    if let Some(value) = get_value_neg(class_name, args[0], &TOP_RIGHT_BOTTOM_LEFT)
                    {
                        max_arg_count(class_name, args, 1, warnings);
                        return Some(Decl::Quad([
                            format!("top: {}", value),
                            format!("right: {}", value),
                            format!("bottom: {}", value),
                            format!("left: {}", value),
                        ]));
                    }

                    warnings.push(WarningType::ValueNotFound(args[0].into()))
                }
            },
            _ => (),
        }
    }

    None
}
