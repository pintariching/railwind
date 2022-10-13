use crate::class::{generate_class, split_by_dash};

#[derive(Debug)]
pub struct Padding;

impl Padding {
    pub fn parse_from_str(class: &str, padding: &str) -> Option<String> {
        if let Some((before, value)) = split_by_dash(padding) {
            let declaration = match before.as_str() {
                "p" => format!("padding: {};", value),
                "pt" => format!("padding-top: {};", value),
                "pb" => format!("padding-bottom: {};", value),
                "pl" => format!("padding-left: {};", value),
                "pr" => format!("padding-right: {};", value),
                "px" => format!("padding-left: {};\n  padding-right: {};", value, value),
                "py" => format!("padding-top: {};\n  padding-bottom: {};", value, value),
                _ => return None,
            };

            let template = format!(".[class-selector] {{\n  {}\n}}\n", declaration);

            return Some(generate_class(class, &template));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_string() {
        let result = Padding::parse_from_str("p-5", "p-5");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), ".p-5 {\n  padding: 1.25rem;\n}\n");

        let result = Padding::parse_from_str("py-5", "py-5");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            ".py-5 {\n  padding-top: 1.25rem;\n  padding-bottom: 1.25rem;\n}\n"
        );
    }
}
