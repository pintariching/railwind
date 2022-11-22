use crate::class::{max_arg_count, Decl};
use crate::warning::WarningType;
use crate::{ret_lit, ret_single_decl, ret_single_decl_pref};

pub fn parse_display(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    match class_name {
        "block" | "flex" | "grid" | "contents" => {
            max_arg_count(class_name, args, 0, warnings);
            ret_single_decl!("display", class_name)
        }
        "hidden" => {
            max_arg_count(class_name, args, 0, warnings);
            ret_lit!("display: none")
        }
        "inline" => match args[0] {
            "" => {
                max_arg_count(class_name, args, 0, warnings);
                ret_lit!("display: inline")
            }
            "block" | "flex" | "table" | "grid" => {
                max_arg_count(class_name, args, 1, warnings);
                ret_single_decl_pref!("display", "inline", args[0])
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["block", "flex", "table", "grid"],
            )),
        },
        "table" => match args[0] {
            "" => {
                max_arg_count(class_name, args, 0, warnings);
                ret_lit!("display: table")
            }
            "caption" | "cell" => {
                max_arg_count(class_name, args, 1, warnings);
                ret_single_decl_pref!("display", "table", args[1])
            }
            "column" => match args[1] {
                "" => {
                    max_arg_count(class_name, args, 1, warnings);
                    ret_lit!("display: table-column")
                }
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    ret_lit!("display: table-column-group")
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            "footer" => match args[1] {
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    ret_lit!("display: table-footer-group")
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            "header" => match args[1] {
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    ret_lit!("display: table-header-group")
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            "row" => match args[1] {
                "" => {
                    max_arg_count(class_name, args, 1, warnings);
                    ret_lit!("display: table-footer")
                }
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    ret_lit!("display: table-row-group")
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["caption", "cell", "column", "footer", "header", "row"],
            )),
        },
        "flow" => match args[0] {
            "root" => {
                max_arg_count(class_name, args, 1, warnings);
                ret_lit!("display: flow-root")
            }
            _ => warnings.push(WarningType::InvalidArg(args[0].into(), vec!["root"])),
        },
        "list" => match args[0] {
            "item" => {
                max_arg_count(class_name, args, 1, warnings);
                ret_lit!("display: list-item")
            }
            _ => warnings.push(WarningType::InvalidArg(args[0].into(), vec!["item"])),
        },

        _ => (),
    }

    None
}
