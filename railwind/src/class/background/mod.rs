use self::background_color::BackgroundColor;

use super::generate_class;

mod background_color;

pub struct Background;

impl Background {
    pub fn parse_from_str(class: &str, background: &str) -> Option<String> {
        let mut split = background.split('-');
        split.next();
        let utility = split.next().unwrap_or("");
        let value = split.next().unwrap_or("");

        let declaration = match utility {
            "fixed" | "local" | "scroll" if value.is_empty() => {
                format!("background-attachment: {};", utility)
            }
            "clip" => format!(
                "background-clip: {};",
                match value {
                    "border" => "border-box",
                    "padding" => "padding-box",
                    "content" => "content-box",
                    "text" => "text",
                    _ => return None,
                }
            ),
            "origin" => format!(
                "background-origin: {};",
                match value {
                    "border" => "border-box",
                    "padding" => "padding-box",
                    "content" => "content-box",
                    _ => return None,
                }
            ),
            "bottom" | "center" | "top" if value.is_empty() => {
                format!("background-position: {};", utility)
            }
            "left" | "right" => match value {
                "top" | "bottom" => format!("background-position: {} {};", utility, value),
                "" => format!("background-position: {};", utility),
                _ => return None,
            },
            "repeat" if value.is_empty() => "background-repeat: repeat;".into(),
            "repeat" => format!(
                "background-repeat: {};",
                match value {
                    "x" => "repeat-x",
                    "y" => "repeat-y",
                    "round" => "round",
                    "space" => "space",
                    _ => return None,
                }
            ),
            "no" if value == "repeat" => "background-repeat: no-repeat;".into(),
            "auto" | "cover" | "contain" if value.is_empty() => {
                format!("background-size: {};", utility)
            }
            _ => return BackgroundColor::parse_from_str(class, utility, value),
        };

        let template = format!(".[class-selector] {{\n  {}\n}}\n", declaration);
        Some(generate_class(class, &template))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_parse_from_str() {
        let result = Background::parse_from_str("bg-clip-border", "bg-clip-border");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            ".bg-clip-border {\n  background-clip: border-box;\n}\n"
        );
    }
}
