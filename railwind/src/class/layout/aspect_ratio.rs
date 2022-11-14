use crate::{class::check_arg_count, traits::IntoDeclaration, warning::WarningType};

#[derive(Debug, PartialEq)]
pub enum AspectRatio {
    Auto,
    Square,
    Video,
}

impl AspectRatio {
    pub fn new(args: &[&str; 3]) -> Result<Self, WarningType> {
        check_arg_count(args, 1)?;

        match args[0] {
            "auto" => Ok(AspectRatio::Auto),
            "square" => Ok(AspectRatio::Square),
            "video" => Ok(AspectRatio::Video),
            _ => Err(WarningType::InvalidArg(
                args[0].into(),
                vec!["auto", "square", "video"],
            )),
        }
    }
}

impl IntoDeclaration for AspectRatio {
    fn into_decl(&self) -> Vec<String> {
        let value = match self {
            AspectRatio::Auto => "auto",
            AspectRatio::Square => "1 / 1",
            AspectRatio::Video => "16 / 9",
        };

        vec![format!("aspect-ratio: {}", value)]
    }
}
