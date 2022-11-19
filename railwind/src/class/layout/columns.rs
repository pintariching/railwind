use crate::class::check_arg_count;
use crate::utils::get_keys;
use crate::warning::WarningType;

use indexmap::IndexMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COLUMNS: IndexMap<&'static str, &'static str> =
        ron::from_str(include_str!("columns.ron")).unwrap();
}

pub fn parse_columns(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Vec<String>> {
    check_arg_count(args, 1, warnings);

    if let Some(columns) = COLUMNS.get(args[0]) {
        return Some(vec![format!("columns: {}", columns)]);
    }

    warnings.push(WarningType::InvalidArg(
        args[0].into(),
        get_keys(COLUMNS.keys()),
    ));

    None
}
