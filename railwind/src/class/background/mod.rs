use crate::config::convert_color;

use super::MultiArgsDeclaration;

pub struct Background;

impl MultiArgsDeclaration for Background {
    fn generate_declaration(class: &str, args: &Vec<&str>) -> Result<Vec<String>, String> {
        let mut args_iter = args.iter();

        let first = args_iter.next();
        let second = args_iter.next();
        let third = args_iter.next();

        if let (Some(first_str), None) = (first, second) {
            let decl = match *first_str {
                // background attachment
                "fixed" | "local" | "scroll" => format!("background-attachment: {}", first_str),

                // background size
                "auto" | "cover" | "contain" => format!("background-size: {}", first_str),

                // background position
                "bottom" | "center" | "left" | "right" | "top" => {
                    format!("background-position: {}", first_str)
                }

                // background repeat
                "repeat" => "background-repeat: repeat".into(),

                // background color
                _ => {
                    if let Some(color) = convert_color(first_str, None) {
                        return Ok(vec![
                            "--tw-bg-opacity: 1".into(),
                            format!("background-color: rgb({} / var(--tw-bg-opacity))", color),
                        ]);
                    } else {
                        return Err(format!(
                            "failed to match background class: {}-{}",
                            class,
                            args.join("-")
                        ));
                    }
                }
            };

            return Ok(vec![decl]);
        }

        if let (Some(first_str), Some(second_str), None) = (first, second, third) {
            let decl = match *first_str {
                // background clip and origin
                "clip" | "origin" => match *second_str {
                    "border" | "padding" | "content" => {
                        format!("background-{}: {}-box", first_str, second_str)
                    }
                    "clip" if *first_str == "clip" => "background-clip: text".into(),
                    _ => {
                        return Err(format!(
                            "failed to match background class: {}-{}",
                            class,
                            args.join("-")
                        ))
                    }
                },

                // background position
                "left" | "right" => match *second_str {
                    "top" | "bottom" => {
                        format!("background-position: {} {}", first_str, second_str)
                    }
                    _ => {
                        return Err(format!(
                            "failed to match background class: {}-{}",
                            class,
                            args.join("-")
                        ))
                    }
                },

                // background repeat
                "repeat" => match *second_str {
                    "x" | "y" => format!("background-repeat: repeat-{}", second_str),
                    "round" | "space" => format!("background-repeat: {}", second_str),
                    _ => {
                        return Err(format!(
                            "failed to match background class: {}-{}",
                            class,
                            args.join("-")
                        ))
                    }
                },
                "no" if *second_str == "repeat" => "background-repeat: no-repeat".into(),

                // background color
                _ => {
                    if let Some(color) = convert_color(first_str, Some(second_str)) {
                        return Ok(vec![
                            "--tw-bg-opacity: 1".into(),
                            format!("background-color: rgb({} / var(--tw-bg-opacity))", color),
                        ]);
                    } else {
                        return Err(format!(
                            "failed to match background class: {}-{}",
                            class,
                            args.join("-")
                        ));
                    }
                }
            };

            return Ok(vec![decl]);
        }

        Err(format!(
            "failed to match background class: {}-{}",
            class,
            args.join("-")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_parse_from_str() {
        // let result = Background::parse_from_str("bg-clip-border", "bg-clip-border");
        // assert!(result.is_some());
        // assert_eq!(
        //     result.unwrap(),
        //     ".bg-clip-border {\n  background-clip: border-box;\n}\n"
        // );
    }
}
