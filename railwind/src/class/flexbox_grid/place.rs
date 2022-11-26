use crate::{
    class::{max_arg_count, min_arg_count, Decl},
    ret_single_decl,
    warning::WarningType,
};

pub fn parse_place(args: &[&str], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("place", args, 2, warnings);
    if min_arg_count(&args, 2, warnings) {
        match args[0] {
            "content" => match args[1] {
                "center" | "start" | "end" | "baseline" | "stretch" => {
                    ret_single_decl!("place-content", args[1])
                }
                "between" | "around" | "evenly" => {
                    ret_single_decl!("place-content", format!("space-{}", args[1]))
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec![
                        "center", "start", "end", "baseline", "stretch", "between", "around",
                        "evenly",
                    ],
                )),
            },
            "items" => match args[1] {
                "center" | "start" | "end" | "baseline" | "stretch" => {
                    ret_single_decl!("place-items", args[1])
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["center", "start", "end", "baseline", "stretch"],
                )),
            },
            "self" => match args[1] {
                "auto" | "start" | "end" | "center" | "stretch" => {
                    ret_single_decl!("place-self", args[1])
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["auto", "start", "end", "center", "stretch"],
                )),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["content", "items", "self"],
            )),
        };
    }

    None
}
