use crate::{
    class::{max_arg_count, min_arg_count, Decl},
    ret_single_decl,
    warning::WarningType,
};

pub fn parse_align_content(args: &[&str], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("content", args, 1, warnings);

    if min_arg_count(&args, 1, warnings) {
        match args[0] {
            "center" | "start" | "end" | "between" | "around" | "evenly" | "baseline" => {
                max_arg_count("content", args, 1, warnings);
                ret_single_decl!(
                    "justify-content",
                    match args[0] {
                        "center" => "center",
                        "start" => "flex-start",
                        "end" => "flex-end",
                        "between" => "space-between",
                        "around" => "space-around",
                        "evenly" => "space-evenly",
                        "baseline" => "baseline",
                        _ => unreachable!(),
                    }
                )
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec![
                    "center", "start", "end", "between", "around", "evenly", "baseline",
                ],
            )),
        };
    }

    None
}
