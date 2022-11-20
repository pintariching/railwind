use crate::class::max_arg_count;
use crate::warning::WarningType;

pub fn parse_display(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    match class_name {
        "block" | "flex" | "grid" | "contents" => {
            max_arg_count(class_name, args, 0, warnings);
            return Some(vec![format!("display: {}", class_name)]);
        }
        "hidden" => {
            max_arg_count(class_name, args, 0, warnings);
            return Some(vec!["display: none".into()]);
        }
        "inline" => match args[0] {
            "" => {
                max_arg_count(class_name, args, 0, warnings);
                return Some(vec!["display: inline".into()]);
            }
            "block" | "flex" | "table" | "grid" => {
                max_arg_count(class_name, args, 1, warnings);
                return Some(vec![format!("display: inline-{}", args[0])]);
            }
            _ => warnings.push(WarningType::InvalidArg(
                args[0].into(),
                vec!["block", "flex", "table", "grid"],
            )),
        },
        "table" => match args[0] {
            "" => {
                max_arg_count(class_name, args, 0, warnings);
                return Some(vec!["display: table".into()]);
            }
            "caption" | "cell" => {
                max_arg_count(class_name, args, 1, warnings);
                return Some(vec![format!("display: table-{}", args[1])]);
            }
            "column" => match args[1] {
                "" => {
                    max_arg_count(class_name, args, 1, warnings);
                    return Some(vec!["display: table-column".into()]);
                }
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    return Some(vec!["display: table-column-group".into()]);
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            "footer" => match args[1] {
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    return Some(vec!["display: table-footer-group".into()]);
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            "header" => match args[1] {
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    return Some(vec!["display: table-header-group".into()]);
                }
                _ => warnings.push(WarningType::InvalidArg(args[1].into(), vec!["group"])),
            },
            "row" => match args[1] {
                "" => {
                    max_arg_count(class_name, args, 1, warnings);
                    return Some(vec!["display: table-footer".into()]);
                }
                "group" => {
                    max_arg_count(class_name, args, 2, warnings);
                    return Some(vec!["display: table-row-group".into()]);
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
                return Some(vec!["display: flow-root".into()]);
            }
            _ => warnings.push(WarningType::InvalidArg(args[0].into(), vec!["root"])),
        },
        "list" => match args[0] {
            "item" => {
                max_arg_count(class_name, args, 1, warnings);
                return Some(vec!["display: list-item".into()]);
            }
            _ => warnings.push(WarningType::InvalidArg(args[0].into(), vec!["item"])),
        },

        _ => (),
    }

    None
}
