use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl};

pub fn parse_object_fit(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(args, 1, warnings) {
        match args[0] {
            "contain" | "cover" | "fill" | "none" => {
                max_arg_count("object-fit", args, 1, warnings);
                ret_single_decl!("object-fit", args[0])
            }
            "scale" => match args[1] {
                "down" => {
                    max_arg_count("object-fit", args, 2, warnings);
                    ret_lit!("object-fit: scale-down")
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["down"])),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["contain", "cover", "fill", "none"],
            )),
        }
    }

    None
}
