use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_single_decl, ret_single_decl_suf};

pub fn parse_box_sizing_decoration_break(
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    if min_arg_count(args, 2, warnings) {
        match args[0] {
            "decoration" => match args[1] {
                "clone" | "slice" => {
                    max_arg_count("box", args, 2, warnings);
                    ret_single_decl!("box-decoration-break", args[1])
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["clone", "slice"],
                )),
            },
            "border" | "content" => {
                max_arg_count("box", args, 1, warnings);
                ret_single_decl_suf!("box-sizing", args[0], "box")
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["decoration", "border", "content"],
            )),
        }
    }

    None
}
