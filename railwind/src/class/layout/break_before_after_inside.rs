use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::warning::WarningType;

pub fn parse_breaks(args: &[&str; 3], warnings: &mut Vec<WarningType>) -> Option<Decl> {
    if min_arg_count(args, 2, warnings) {
        if let Some(value) = match args[0] {
            "before" | "after" => match args[1] {
                "auto" | "all" | "page" | "left" | "right" | "column" => {
                    max_arg_count("break", args, 2, warnings);
                    Some(args[1])
                }
                "avoid" => match args[2] {
                    "page" => {
                        max_arg_count("break", args, 3, warnings);
                        Some("avoid-page")
                    }
                    "" => {
                        max_arg_count("break", args, 2, warnings);
                        Some(args[2])
                    }
                    _ => {
                        warnings.push(WarningType::InvalidArg(args[2].into(), vec!["page"]));
                        None
                    }
                },
                _ => {
                    warnings.push(WarningType::InvalidArg(
                        args[1].into(),
                        vec!["auto", "avoid", "all", "page", "left", "right", "column"],
                    ));
                    None
                }
            },
            "inside" => match args[1] {
                "auto" => {
                    max_arg_count("break", args, 2, warnings);
                    Some(args[1])
                }
                "avoid" => match args[2] {
                    "page" => {
                        max_arg_count("break", args, 2, warnings);
                        Some("avoid-page")
                    }
                    "column" => {
                        max_arg_count("break", args, 2, warnings);
                        Some("avoid-column")
                    }
                    "" => {
                        max_arg_count("break", args, 1, warnings);
                        Some(args[1])
                    }
                    _ => {
                        warnings.push(WarningType::InvalidArg(
                            args[2].into(),
                            vec!["page", "column"],
                        ));
                        None
                    }
                },
                _ => {
                    warnings.push(WarningType::InvalidArg(
                        args[1].into(),
                        vec!["auto", "avoid"],
                    ));
                    None
                }
            },
            _ => {
                warnings.push(WarningType::InvalidArg(
                    args[0].into(),
                    vec!["before", "after", "inside"],
                ));
                None
            }
        } {
            return Some(Decl::Single(format!("break-{}: {}", args[0], value)));
        }
    }

    None
}
