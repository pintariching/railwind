mod aspect_ratio;
mod columns;
mod container;

use aspect_ratio::parse_aspect_ratio;
use columns::parse_columns;

pub use aspect_ratio::ASPECT_RATIO;
pub use columns::COLUMNS;

use crate::warning::WarningType;

pub fn parse_layout(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    match class_name {
        "aspect" => {
            if let Some(aspect_ratio) = parse_aspect_ratio(args, warnings) {
                return Some(aspect_ratio);
            }
        }
        "columns" => {
            if let Some(columns) = parse_columns(args, warnings) {
                return Some(columns);
            }
        }
        _ => return None,
    }

    None
}
