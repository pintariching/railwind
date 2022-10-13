use crate::class::generate_class;

pub struct Flex;

impl Flex {
    pub fn parse_from_str(class: &str, flex: &str) -> Option<String> {
        if let Some(declaration) = match flex {
            "flex-1" => Some("flex: 1 1 0%"),
            "flex-auto" => Some("flex: 1 1 auto"),
            "flex-initial" => Some("flex: 0 1 auto"),
            "flex-none" => Some("flex: none"),
            "flex" => Some("display: flex"),
            _ => None,
        } {
            let template = format!(".[class-selector] {{\n  {};\n}}\n", declaration);
            return Some(generate_class(class, &template));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_str() {
        assert!(Flex::parse_from_str("flex", "flex").is_some());
        assert!(Flex::parse_from_str("flec", "flec").is_none());

        assert_eq!(
            Flex::parse_from_str("flex", "flex").unwrap(),
            ".flex {\n  display: flex;\n}\n"
        );

        assert_eq!(
            Flex::parse_from_str("flex-1", "flex-1").unwrap(),
            ".flex-1 {\n  flex: 1 1 0%;\n}\n"
        )
    }
}
