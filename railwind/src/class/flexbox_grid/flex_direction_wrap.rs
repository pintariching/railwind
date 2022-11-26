use crate::{
    class::{max_arg_count, min_arg_count, Decl},
    ret_lit,
    warning::WarningType,
};

pub fn parse_direction_wrap(args: &[&str], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(&args, 1, warnings) {
        match args[0] {
            "row" => match args[1] {
                "" => {
                    max_arg_count("flex", args, 1, warnings);
                    ret_lit!("flex-direction: row")
                }
                "reverse" => {
                    max_arg_count("flex", args, 2, warnings);
                    ret_lit!("flex-direction: row-reverse");
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["reverse"])),
            },
            "col" => match args[1] {
                "" => {
                    max_arg_count("flex", args, 1, warnings);
                    ret_lit!("flex-direction: column")
                }
                "reverse" => {
                    max_arg_count("flex", args, 2, warnings);
                    ret_lit!("flex-direction: column-reverse");
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["reverse"])),
            },
            "wrap" => match args[1] {
                "" => {
                    max_arg_count("flex", args, 1, warnings);
                    ret_lit!("flex-wrap: reverse")
                }
                "reverse" => {
                    max_arg_count("flex", args, 2, warnings);
                    ret_lit!("flex-direction: wrap-reverse")
                }
                "nowrap" => {
                    max_arg_count("flex", args, 1, warnings);
                    ret_lit!("flex-wrap: nowrap")
                }
                _ => warnings.push(WarningType::InvalidArg(
                    args[1].into(),
                    vec!["reverse", "nowrap"],
                )),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["row", "col", "wrap"],
            )),
        };
    }

    None
}
