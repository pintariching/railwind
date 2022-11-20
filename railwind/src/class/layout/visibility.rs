use crate::class::max_arg_count;
use crate::warning::WarningType;

pub fn parse_visibility(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    match class_name {
        "visible" | "invisible" | "collapse" => {
            max_arg_count("visibility", args, 1, warnings);
            return Some(vec![format!("visibility: {}", args[0])]);
        }
        _ => (),
    }

    None
}
