use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_single_decl;
use crate::warning::WarningType;

pub fn parse_overscroll(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(args, 1, warnings) {
        match args[0] {
            "auto" | "contain" | "none" => {
                max_arg_count("overscroll-behavior", args, 1, warnings);
                ret_single_decl!("overscroll-behavior", args[0])
            }
            "x" | "y" => match args[1] {
                "auto" | "contain" | "none" => {
                    max_arg_count("overscroll-behavior", args, 2, warnings);
                    return Some(Decl::Single(format!(
                        "overscroll-behavior-{}: {}",
                        args[0], args[1]
                    )));
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["auto", "contain", "none"],
                )),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["auto", "contain", "none", "x", "y"],
            )),
        }
    }

    None
}