mod layout;
mod spacing;

use layout::parse_layout;

use crate::warning::WarningType;

pub use layout::ASPECT_RATIO;

use self::spacing::parse_spacing;
pub fn parse_class(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    let warning_count = warnings.len();

    if let Some(layout) = parse_layout(class_name, args, warnings) {
        return Some(layout);
    }

    if let Some(spacing) = parse_spacing(class_name, args, warnings) {
        return Some(spacing);
    }

    // prevents duplicate error messages
    // if a message has already been pushed to warnings, then there was a problem elsewhere
    if warning_count == warnings.len() {
        warnings.push(WarningType::ClassNotFound);
    }

    None
}

pub fn max_arg_count(
    class_name: &str,
    args: &[&str],
    count: usize,
    warnings: &mut Vec<WarningType>,
) {
    let mut non_empty_count = 0;
    for arg in args {
        if !arg.is_empty() {
            non_empty_count += 1;
        }
    }

    if non_empty_count > count {
        warnings.push(WarningType::TooManyArgs(
            non_empty_count,
            count,
            format!("{}-{}", class_name, args[..count].join("-"),),
        ));
    }
}

/// Checks if args containt the minumum count of arguments that are not empty.
/// Returns true, if args containt more than the minimum amount of arguments that are not empty
/// otherwise returns false
pub fn min_arg_count(args: &[&str], count: usize, warnings: &mut Vec<WarningType>) -> bool {
    let mut non_empty_count = 0;
    for arg in args {
        if !arg.is_empty() {
            non_empty_count += 1;
        }
    }

    if non_empty_count < count {
        warnings.push(WarningType::NotEnoughArgs(non_empty_count, count));
        return false;
    }

    true
}
