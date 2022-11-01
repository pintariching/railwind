use crate::class::OneArgDeclaration;

#[derive(Debug)]
pub struct AspectRatio;

impl OneArgDeclaration for AspectRatio {
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
