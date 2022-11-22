use crate::class::{max_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

pub fn parse_visibility(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    match class_name {
        "visible" | "invisible" | "collapse" => {
            max_arg_count("visibility", args, 1, warnings);
            ret_single_decl!("visibility", args[0])
        }
        _ => (),
    }

    None
}
