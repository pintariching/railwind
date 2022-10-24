use crate::class::OneArgSingleDeclaration;

#[derive(Debug)]
pub struct AspectRatio;

impl OneArgSingleDeclaration for AspectRatio {
    fn generate_declaration(arg: &str) -> Result<Vec<String>, String> {
        let decl = format!(
                "aspect-ratio: {}",
                match arg {
                    "auto" => "auto",
                    "square" => "1 / 1",
                    "video" => "16 / 9",
                    _ => return Err("class doesn't contain a valid ratio, should be either 'auto', 'square' or 'video'".into()),
                }
            );

        return Ok(vec![decl]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class::SeperatedClass;

    #[test]
    fn test_generate_declaration() {
        let class = SeperatedClass {
            class: "aspect",
            raw_class: "aspect-square",
            args: Some(vec!["square"]),
            pseudo_classes: None,
            pseudo_elements: None,
            media_queries: None,
        };

        let result = AspectRatio::generate(&class);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["aspect-ratio: 1 / 1"]);
    }
}
