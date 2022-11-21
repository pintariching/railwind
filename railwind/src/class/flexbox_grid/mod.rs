mod basis;

use basis::parse_basis;

pub use basis::BASIS;

use crate::warning::WarningType;

pub fn parse_flexbox_grid(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    match class_name {
        "basis" => {
            if let Some(basis) = parse_basis(args, warnings) {
                return Some(basis);
            }
        }
        _ => (),
    }

    None
}
