use crate::class::{max_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

pub fn parse_position(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    match class_name {
        "static" | "fixed" | "absolute" | "relative" | "sticky" => {
            max_arg_count("position", args, 1, warnings);
            ret_single_decl!("position", class_name)
        }
        _ => (),
    }

    None
}
