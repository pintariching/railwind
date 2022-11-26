use crate::{
    class::{max_arg_count, min_arg_count, Decl},
    ret_single_decl,
    warning::WarningType,
};

pub fn parse_align_self(args: &[&str], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    max_arg_count("self", args, 1, warnings);

    if min_arg_count(&args, 1, warnings) {
        match args[0] {
            "auto" | "start" | "end" | "center" | "baseline" | "stretch" => {
                max_arg_count("self", args, 1, warnings);
                ret_single_decl!(
                    "justify-content",
                    match args[0] {
                        "auto" => "auto",
                        "start" => "flex-start",
                        "end" => "flex-end",
                        "center" => "center",
                        "baseline" => "baseline",
                        "stretch" => "stretch",
                        _ => unreachable!(),
                    }
                )
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["auto", "start", "end", "center", "baseline", "stretch"],
            )),
        };
    }

    None
}
