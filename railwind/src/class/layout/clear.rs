use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

pub fn parse_clear(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(args, 1, warnings) {
        match args[0] {
            "left" | "right" | "both" | "none" => {
                max_arg_count("clear", args, 1, warnings);
                ret_single_decl!("clear", args[0])
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["left", "right", "both", "none"],
            )),
        }
    }

    None
}
