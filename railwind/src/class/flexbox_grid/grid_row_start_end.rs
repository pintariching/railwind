use crate::class::utils::get_value;
use crate::class::{max_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GRID_ROW_SPAN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_row_span.ron")).unwrap();
    pub static ref GRID_ROW_START: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_row_start.ron")).unwrap();
    pub static ref GRID_ROW_END: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_row_end.ron")).unwrap();
}

pub fn parse_row_start_end(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("row", args, 2, warnings);

    match args[0] {
        "auto" => {
            ret_lit!("grid-row: auto")
        }
        "span" => {
            if let Some(span) = get_value(args[1], &GRID_ROW_SPAN) {
                ret_single_decl!("row-span", span)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "start" => {
            if let Some(start) = get_value(args[1], &GRID_ROW_START) {
                ret_single_decl!("grid-row-end", start)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "end" => {
            if let Some(end) = get_value(args[1], &GRID_ROW_END) {
                ret_single_decl!("grid-row-end", end)
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
