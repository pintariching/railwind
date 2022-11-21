mod margin;
mod padding;
mod space_between;

use crate::warning::WarningType;

use self::{margin::parse_margin, padding::parse_padding, space_between::parse_space_between};

pub fn parse_spacing(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    if class_name.starts_with('p') {
        if let Some(padding) = parse_padding(class_name, args, warnings) {
            return Some(padding);
        }
    }

    if class_name.starts_with('m') || class_name.starts_with("-m") {
        if let Some(margin) = parse_margin(class_name, args, warnings) {
            return Some(margin);
        }
    }

    match class_name {
        "-space" | "space" => {
            if let Some(spacing) = parse_space_between(class_name, args, warnings) {
                return Some(spacing);
            }
        }
        _ => (),
    }

    None
}
