use crate::class::{generate_class, split_by_dash};

#[derive(Debug)]
pub struct Margin;

impl Margin {
    pub fn parse_from_str(class: &str, margin: &str) -> Option<String> {
        if let Some((before, value)) = split_by_dash(margin) {
            let declaration = match before.as_str() {
                "p" => format!("margin: {};", value),
                "pt" => format!("margin-top: {};", value),
                "pb" => format!("margin-bottom: {};", value),
                "pl" => format!("margin-left: {};", value),
                "pr" => format!("margin-right: {};", value),
                "px" => format!("margin-left: {};\n  margin-right: {};", value, value),
                "py" => format!("margin-top: {};\n  margin-bottom: {};", value, value),
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
        let result = Margin::parse_from_str("p-5", "p-5");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), ".p-5 {\n  margin: 1.25rem;\n}\n");

        let result = Margin::parse_from_str("py-5", "py-5");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            ".py-5 {\n  margin-top: 1.25rem;\n  margin-bottom: 1.25rem;\n}\n"
        );
    }
}
