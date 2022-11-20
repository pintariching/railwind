use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

pub fn parse_object_fit_position(
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    if min_arg_count(args, 1, warnings) {
        match args[0] {
            "contain" | "cover" | "fill" | "none" => {
                max_arg_count("object-fit", args, 1, warnings);
                return Some(vec![format!("object-fit: {}", args[0])]);
            }
            "scale" => match args[1] {
                "down" => {
                    max_arg_count("object-fit", args, 2, warnings);
                    return Some(vec!["object-fit: scale-down".into()]);
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["down"])),
            },

            "bottom" | "center" | "top" => {
                max_arg_count("object-position", args, 1, warnings);
                return Some(vec![format!("object-position: {}", args[0])]);
            }
            "left" | "right" => match args[1] {
                "" => {
                    max_arg_count("object-position", args, 1, warnings);
                    return Some(vec![format!("object-position: {}", args[0])]);
                }
                "bottom" | "top" => {
                    max_arg_count("object-position", args, 2, warnings);
                    return Some(vec![format!("object-position: {} {}", args[0], args[1])]);
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["bottom", "top"],
                )),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec![
                    "contain", "cover", "fill", "none", "bottom", "center", "left", "right", "top",
                ],
            )),
        }
    }

    None
}
