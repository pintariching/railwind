use crate::class::MultiArgsDeclaration;
use crate::config::convert_spacing;

#[derive(Debug)]
pub struct SpaceBetween;

impl MultiArgsDeclaration for SpaceBetween {
    fn generate_declaration(class: &str, args: &Vec<&str>) -> Result<Vec<String>, String> {
        let mut args_iter = args.iter();

        let first = args_iter.next();
        let second = args_iter.next();
        let third = args_iter.next();

        if let (Some(direction), Some(value_str), None) = (first, second, third) {
            if *value_str == "reverse" {
                let decl = format!(
                    "--tw-space-{}-reverse: 1",
                    match *direction {
                        "x" | "y" => direction,
                        _ =>
                            return Err(format!(
                                "failed to match space between: {}-{}",
                                class,
                                args.join("-")
                            )),
                    }
                );

                return Ok(vec![decl]);
            }

            let value = convert_spacing(value_str)?;

            let decl = format!(
                "margin-{}: {}",
                match *direction {
                    "x" => "left",
                    "y" => "top",
                    _ =>
                        return Err(format!(
                            "failed to match space between: {}-{}",
                            class,
                            args.join("-")
                        )),
                },
                value
            );

            return Ok(vec![decl]);
        }

        return Err(format!(
            "failed to match space between: {}-{}",
            class,
            args.join("-")
        ));
    }
}
