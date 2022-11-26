use crate::class::utils::get_value;
use crate::class::{max_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref GRID_TEMPLATE_COLUMNS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_template_columns.ron")).unwrap();
    pub static ref GRID_TEMPLATE_ROWS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_template_rows.ron")).unwrap();
}

pub fn parse_grid_template_col_row_flow(
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    max_arg_count("grid", args, 2, warnings);

    match args[0] {
        "cols" => {
            if let Some(cols) = get_value(args[1], &GRID_TEMPLATE_COLUMNS) {
                ret_single_decl!("grid-template-rows", cols)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "rows" => {
            if let Some(rows) = get_value(args[1], &GRID_TEMPLATE_ROWS) {
                ret_single_decl!("grid-template-rows", rows)
            }

            warnings.push(WarningType::ValueNotFound(args[1].into()));
        }
        "flow" => match args[1] {
            "row" => match args[2] {
                "" => {
                    max_arg_count("grid", args, 2, warnings);
                    ret_lit!("grid-auto-flow: row")
                }
                "dense" => {
                    max_arg_count("grid", args, 3, warnings);
                    ret_lit!("grid-auto-flow: row dense")
                }
                _ => warnings.push(WarningType::InvalidArg(args[2].into(), vec!["dense"])),
            },
            "col" => match args[2] {
                "" => {
                    max_arg_count("grid", args, 2, warnings);
                    ret_lit!("grid-auto-flow: column")
                }
                "dense" => {
                    max_arg_count("grid", args, 3, warnings);
                    ret_lit!("grid-auto-flow: column dense")
                }
                _ => warnings.push(WarningType::InvalidArg(args[2].into(), vec!["dense"])),
            },
            "dense" => {
                max_arg_count("grid", args, 2, warnings);
                ret_lit!("grid-auto-flow: dense")
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[1].into(),
                vec!["row", "col", "dense"],
            )),
        },
        _ => {
            warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["cols", "rows", "flow"],
            ));
        }
    }

    None
}
