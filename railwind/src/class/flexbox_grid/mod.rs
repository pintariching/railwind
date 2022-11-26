mod align_content;
mod align_items;
mod align_self;
mod basis;
mod flex;
mod flex_direction_wrap;
mod gap;
mod grid_auto_cols_rows;
mod grid_col_start_end;
mod grid_row_start_end;
mod grid_template_col_row_flow;
mod grow;
mod justify;
mod order;
mod place;
mod shrink;

use basis::{parse_basis, BASIS};
use flex::{parse_flex, FLEX};
use flex_direction_wrap::parse_direction_wrap;
use gap::{parse_gap, GAP};
use grid_auto_cols_rows::{parse_grid_auto_cols_rows, GRID_AUTO_COLS, GRID_AUTO_ROWS};
use grid_col_start_end::{
    parse_column_start_end, GRID_COLUMN_END, GRID_COLUMN_SPAN, GRID_COLUMN_START,
};
use grid_row_start_end::{parse_row_start_end, GRID_ROW_END, GRID_ROW_SPAN, GRID_ROW_START};
use grid_template_col_row_flow::{
    parse_grid_template_col_row_flow, GRID_TEMPLATE_COLUMNS, GRID_TEMPLATE_ROWS,
};
use grow::{parse_grow, GROW};
use order::{parse_order, ORDER};
use shrink::{parse_shrink, SHRINK};

use self::{
    align_content::parse_align_content, align_items::parse_align_items,
    align_self::parse_align_self, justify::parse_justify, place::parse_place,
};

use super::Decl;
use crate::warning::WarningType;

pub fn parse_flexbox_grid(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    match class_name {
        "basis" => {
            if let Some(basis) = parse_basis(args, warnings) {
                return Some(basis);
            }
        }
        "flex" => {
            if let Some(direction_wrap) = parse_direction_wrap(args, warnings) {
                return Some(direction_wrap);
            }

            if let Some(flex) = parse_flex(args, warnings) {
                return Some(flex);
            }
        }
        "grow" => {
            if let Some(grow) = parse_grow(args, warnings) {
                return Some(grow);
            }
        }
        "shrink" => {
            if let Some(shrink) = parse_shrink(args, warnings) {
                return Some(shrink);
            }
        }
        "order" => {
            if let Some(order) = parse_order(args, warnings) {
                return Some(order);
            }
        }
        "grid" => {
            if let Some(grid) = parse_grid_template_col_row_flow(args, warnings) {
                return Some(grid);
            }
        }
        "col" => {
            if let Some(col) = parse_column_start_end(args, warnings) {
                return Some(col);
            }
        }
        "row" => {
            if let Some(row) = parse_row_start_end(args, warnings) {
                return Some(row);
            }
        }
        "auto" => {
            if let Some(grid_auto) = parse_grid_auto_cols_rows(args, warnings) {
                return Some(grid_auto);
            }
        }
        "gap" => {
            if let Some(gap) = parse_gap(args, warnings) {
                return Some(gap);
            }
        }
        "justify" => {
            if let Some(justify) = parse_justify(args, warnings) {
                return Some(justify);
            }
        }
        "content" => {
            if let Some(align_content) = parse_align_content(args, warnings) {
                return Some(align_content);
            }
        }
        "items" => {
            if let Some(align_items) = parse_align_items(args, warnings) {
                return Some(align_items);
            }
        }
        "self" => {
            if let Some(align_self) = parse_align_self(args, warnings) {
                return Some(align_self);
            }
        }
        "place" => {
            if let Some(place) = parse_place(args, warnings) {
                return Some(place);
            }
        }
        _ => {}
    }

    None
}
