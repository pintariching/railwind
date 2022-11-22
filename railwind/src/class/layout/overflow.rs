use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

pub fn parse_overflow(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(args, 1, warnings) {
        match args[0] {
            "auto" | "hidden" | "clip" | "visible" | "scroll" => {
                max_arg_count("overflow", args, 1, warnings);
                ret_single_decl!("overflow", args[0])
            }
            "x" | "y" => match args[1] {
                "auto" | "hidden" | "clip" | "visible" | "scroll" => {
                    max_arg_count("overflow", args, 2, warnings);
                    return Some(Decl::Single(format!("overflow-{}: {}", args[0], args[1])));
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["auto", "hidden", "clip", "visible", "scroll"],
                )),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["auto", "hidden", "clip", "visible", "scroll", "x", "y"],
            )),
        }
    }

    None
}
