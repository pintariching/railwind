use crate::{
    class::{max_arg_count, min_arg_count, Decl},
    ret_single_decl,
    warning::WarningType,
};

pub fn parse_justify(args: &[&str], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(&args, 1, warnings) {
        match args[0] {
            "start" | "end" | "center" | "between" | "around" | "evenly" => {
                max_arg_count("justify", args, 1, warnings);
                ret_single_decl!(
                    "justify-content",
                    match args[0] {
                        "start" => "flex-start",
                        "end" => "flex-end",
                        "center" => "center",
                        "between" => "space-between",
                        "around" => "space-around",
                        "evenly" => "space-evenly",
                        _ => unreachable!(),
                    }
                )
            }
            "items" => match args[1] {
                "start" | "end" | "center" | "stretch" => {
                    max_arg_count("justify", args, 2, warnings);
                    ret_single_decl!("justify-items", args[1])
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[0].into(),
                    vec!["start", "end", "center", "stretch"],
                )),
            },
            "self" => match args[1] {
                "auto" | "start" | "end" | "center" | "stretch" => {
                    max_arg_count("justify", args, 2, warnings);
                    ret_single_decl!("justify-self", args[1])
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[0].into(),
                    vec!["auto", "start", "end", "center", "stretch"],
                )),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec![
                    "start", "end", "center", "between", "around", "evenly", "items", "self",
                ],
            )),
        };
    }

    None
}
