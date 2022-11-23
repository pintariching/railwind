mod basis;
mod flex;
mod flex_direction_wrap;
mod grow;
mod order;
mod shrink;

use basis::parse_basis;

pub use basis::BASIS;

use crate::warning::WarningType;

use self::{
    flex::parse_flex, flex_direction_wrap::parse_direction_wrap, grow::parse_grow,
    order::parse_order, shrink::parse_shrink,
};

use super::Decl;

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
        _ => {}
    }

    None
}
