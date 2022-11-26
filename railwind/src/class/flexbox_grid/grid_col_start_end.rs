use crate::class::utils::get_value;
use crate::class::{max_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GRID_COLUMN_SPAN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_col_span.ron")).unwrap();
    pub static ref GRID_COLUMN_START: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_col_start.ron")).unwrap();
    pub static ref GRID_COLUMN_END: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_col_end.ron")).unwrap();
}

pub fn parse_column_start_end(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("col", args, 2, warnings);

    match args[0] {
        "auto" => {
            ret_lit!("grid-column: auto")
        }
        "span" => {
            if let Some(span) = get_value(args[1], &GRID_COLUMN_SPAN) {
                ret_single_decl!("grid-column", span)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "start" => {
            if let Some(start) = get_value(args[1], &GRID_COLUMN_START) {
                ret_single_decl!("grid-column-start", start)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "end" => {
            if let Some(end) = get_value(args[1], &GRID_COLUMN_END) {
                ret_single_decl!("grid-column-end", end)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        _ => {
            warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["auto", "span", "start", "end"],
            ));
        }
    }

    None
}
