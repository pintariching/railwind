mod aspect_ratio;
mod box_sizing_decoration_break;
mod break_before_after_inside;
mod clear;
mod columns;
mod display;
mod floats;
mod isolation;
mod object_fit_position;
mod overflow;
mod overscroll;
mod position;
mod top_right_bottom_left;
mod visibility;
mod z_index;

use aspect_ratio::parse_aspect_ratio;
use box_sizing_decoration_break::parse_box_sizing_decoration_break;
use break_before_after_inside::parse_breaks;
use clear::parse_clear;
use columns::parse_columns;
use display::parse_display;
use floats::parse_floats;
use isolation::parse_isolation;
use object_fit_position::parse_object_fit_position;
use overflow::parse_overflow;
use overscroll::parse_overscroll;
use position::parse_position;
use top_right_bottom_left::parse_top_right_bottom_left;
use visibility::parse_visibility;
use z_index::parse_z_index;

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
        "break" => {
            if let Some(break_) = parse_breaks(args, warnings) {
                return Some(break_);
            }
        }
        "box" => {
            if let Some(box_sizing_decoration_break) =
                parse_box_sizing_decoration_break(args, warnings)
            {
                return Some(box_sizing_decoration_break);
            }
        }
        "float" => {
            if let Some(floats) = parse_floats(args, warnings) {
                return Some(floats);
            }
        }
        "clear" => {
            if let Some(clear) = parse_clear(args, warnings) {
                return Some(clear);
            }
        }
        "object" => {
            if let Some(object_fit) = parse_object_fit_position(args, warnings) {
                return Some(object_fit);
            }
        }
        "overflow" => {
            if let Some(overflow) = parse_overflow(args, warnings) {
                return Some(overflow);
            }
        }
        "overscroll" => {
            if let Some(overscroll) = parse_overscroll(args, warnings) {
                return Some(overscroll);
            }
        }
        "z" => {
            if let Some(z_index) = parse_z_index(args, warnings) {
                return Some(z_index);
            }
        }

        _ => {
            if let Some(display) = parse_display(class_name, args, warnings) {
                return Some(display);
            }
            if let Some(isolation) = parse_isolation(class_name, args, warnings) {
                return Some(isolation);
            }
            if let Some(position) = parse_position(class_name, args, warnings) {
                return Some(position);
            }
            if let Some(top_right_bottom_left) =
                parse_top_right_bottom_left(class_name, args, warnings)
            {
                return Some(top_right_bottom_left);
            }
            if let Some(visibility) = parse_visibility(class_name, args, warnings) {
                return Some(visibility);
            }
        }
    }

    None
}
