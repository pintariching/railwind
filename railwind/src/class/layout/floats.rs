use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

pub fn parse_floats(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Vec<String>> {
    if min_arg_count(args, 1, warnings) {
        match args[0] {
            "left" | "right" | "none" => {
                max_arg_count("float", args, 1, warnings);
                return Some(vec![format!("float: {}", args[0])]);
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["left", "right", "none"],
            )),
        }
    }

    None
}
