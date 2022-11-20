use crate::class::max_arg_count;
use crate::warning::WarningType;

pub fn parse_position(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    match class_name {
        "static" | "fixed" | "absolute" | "relative" | "sticky" => {
            max_arg_count("position", args, 1, warnings);
            return Some(vec![format!("position: {}", args[0])]);
        }
        _ => (),
    }

    None
}
