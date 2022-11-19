mod layout;

use layout::parse_layout;

use crate::warning::WarningType;

pub use layout::ASPECT_RATIO;
pub fn parse_class(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    let warning_count = warnings.len();

    if let Some(layout) = parse_layout(class_name, args, warnings) {
        return Some(layout);
    }

    // prevents duplicate error messages
    // if a message has already been pushed to warnings, then there was a problem elsewhere
    if warning_count == warnings.len() {
        warnings.push(WarningType::ClassNotFound);
    }

    None
}

pub fn check_arg_count(args: &[&str], count: usize, warnings: &mut Vec<WarningType>) {
    let mut non_empty_count = 0;
    for arg in args {
        if !arg.is_empty() {
            non_empty_count += 1;
        }
    }

    if non_empty_count > count {
        warnings.push(WarningType::TooManyArgs(non_empty_count, count));
    }
}
