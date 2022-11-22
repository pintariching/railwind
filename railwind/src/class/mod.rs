mod flexbox_grid;
mod layout;
mod macros;
mod spacing;
mod utils;

use layout::parse_layout;
use spacing::parse_spacing;

use crate::warning::WarningType;

pub use layout::{ASPECT_RATIO, COLUMNS, TOP_RIGHT_BOTTOM_LEFT, Z_INDEX};
pub use spacing::{MARGIN, PADDING, SPACE_BETWEEN};

use self::flexbox_grid::parse_flexbox_grid;

#[derive(Debug)]
pub enum Decl {
    Lit(&'static str),
    Single(String),
    Double([String; 2]),
    Quad([String; 4]),
}

impl Decl {
    pub fn to_string(self) -> String {
        match self {
            Decl::Lit(lit) => lit.to_string(),
            Decl::Single(s) => s,
            Decl::Double(d) => d.join(";\n    "),
            Decl::Quad(q) => q.join(";\n    "),
        }
    }
}

pub fn parse_class(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    let warning_count = warnings.len();

    if let Some(layout) = parse_layout(class_name, args, warnings) {
        return Some(layout);
    }

    if let Some(spacing) = parse_spacing(class_name, args, warnings) {
        return Some(spacing);
    }

    if let Some(flexbox_grid) = parse_flexbox_grid(class_name, args, warnings) {
        return Some(flexbox_grid);
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
