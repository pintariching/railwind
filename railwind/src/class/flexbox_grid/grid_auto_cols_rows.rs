use crate::class::utils::get_value;
use crate::class::Decl;
use crate::ret_single_decl;
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GRID_AUTO_COLS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_auto_cols.ron")).unwrap();
    pub static ref GRID_AUTO_ROWS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_auto_rows.ron")).unwrap();
}

pub fn parse_grid_auto_cols_rows(
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    match args[0] {
        "cols" => {
            if let Some(cols) = get_value(args[1], &GRID_AUTO_COLS) {
                ret_single_decl!("grit-auto-columns", cols)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "rows" => {
            if let Some(rows) = get_value(args[1], &GRID_AUTO_ROWS) {
                ret_single_decl!("grid-column-rows", rows)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        _ => {
            warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["cols", "rows"],
            ));
        }
    }

    None
}
