use crate::class::OneArgDeclarationWithDirection;
use crate::config::convert_spacing;

#[derive(Debug)]
pub struct MarginAndPadding;

impl OneArgDeclarationWithDirection for MarginAndPadding {
    fn generate_declaration(class: &str, arg: &str) -> Result<Vec<String>, String> {
        let mut chars_iter = class.chars();

        let value = if class.starts_with('-') {
            chars_iter.next();
            format!("-{}", convert_spacing(arg)?)
        } else {
            convert_spacing(arg)?.to_string()
        };

        let first = chars_iter.next();
        let second = chars_iter.next();
        let third = chars_iter.next();

        let decl = if let (Some(first_char), None) = (first, second) {
            vec![format!(
                "{}: {}",
                match first_char {
                    'm' => "margin",
                    'p' => "padding",
                    _ => return Err(format!("failed to match class: {}-{}", class, arg)),
                },
                value
            )]
        } else if let (Some(first_char), Some(second_char), None) = (first, second, third) {
            let m_or_p = match first_char {
                'm' => "margin",
                'p' => "padding",
                _ => return Err(format!("failed to match class: {}-{}", class, arg)),
            };

            let dir = match second_char {
                't' => vec!["top"],
                'b' => vec!["bottom"],
                'l' => vec!["left"],
                'r' => vec!["right"],
                'x' => vec!["left", "right"],
                'y' => vec!["top", "bottom"],
                _ => return Err(format!("failed to match class: {}-{}", class, arg)),
            };

            let mut out = Vec::new();
            for d in dir {
                out.push(format!("{}-{}: {}", m_or_p, d, value));
            }

            out
        } else {
            return Err(format!("failed to match class: {}-{}", class, arg));
        };

        Ok(decl)
    }
}
