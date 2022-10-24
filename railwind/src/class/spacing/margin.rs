use crate::class::OneArgDeclarationWithDirection;
use crate::config::convert_spacing;

#[derive(Debug)]
pub struct Margin;

impl OneArgDeclarationWithDirection for Margin {
    fn generate_declaration(class: &str, arg: &str) -> Result<Vec<String>, String> {
        let mut res = Vec::new();

        let value = convert_spacing(arg)?;

        match class {
            "m" => res.push(format!("margin: {}", value)),
            "mt" => res.push(format!("margin-top: {}", value)),
            "mb" => res.push(format!("margin-bottom: {}", value)),
            "ml" => res.push(format!("margin-left: {}", value)),
            "mr" => res.push(format!("margin-right: {}", value)),
            "mx" => {
                res.push(format!("margin-left: {}", value));
                res.push(format!("margin-right: {}", value));
            }
            "my" => {
                res.push(format!("margin-top: {}", value));
                res.push(format!("margin-bottom: {}", value));
            }
            _ => return Err(format!("failed to match margin class: {}-{}", class, arg)),
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class::SeperatedClass;

    #[test]
    fn test_generate_declaration() {
        let class = SeperatedClass {
            class: "m",
            raw_class: "m-5",
            args: Some(vec!["5"]),
            pseudo_classes: None,
            pseudo_elements: None,
            media_queries: None,
        };

        let result = Margin::generate(&class);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["margin: 1.25rem"]);
    }

    #[test]
    fn test_generate_declaration_multi_line() {
        let class = SeperatedClass {
            class: "mx",
            raw_class: "mx-5",
            args: Some(vec!["5"]),
            pseudo_classes: None,
            pseudo_elements: None,
            media_queries: None,
        };

        let result = Margin::generate(&class);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec!["margin-left: 1.25rem", "margin-right: 1.25rem"]
        );
    }
}
