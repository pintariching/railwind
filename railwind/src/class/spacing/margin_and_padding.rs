use crate::class::OneArgDeclarationWithDirection;
use crate::config::convert_spacing;

#[derive(Debug)]
pub struct MarginAndPadding;

impl OneArgDeclarationWithDirection for MarginAndPadding {
    fn generate_declaration(class: &str, arg: &str) -> Result<Vec<String>, String> {
        let mut chars_iter = class.chars();

        let first = chars_iter.next();
        let second = chars_iter.next();
        let third = chars_iter.next();

        let value = convert_spacing(arg)?;

        let decl = if let (Some(first_char), None) = (first, second) {
            format!(
                "{}: {}",
                match first_char {
                    'm' => "margin",
                    'p' => "padding",
                    _ => return Err(format!("failed to match class: {}-{}", class, arg)),
                },
                value
            )
        } else if let (Some(first_char), Some(second_char), None) = (first, second, third) {
            format!(
                "{}-{}: {}",
                match first_char {
                    'm' => "margin",
                    'p' => "padding",
                    _ => return Err(format!("failed to match class: {}-{}", class, arg)),
                },
                match second_char {
                    't' => "top",
                    'b' => "bottom",
                    'l' => "left",
                    'r' => "right",
                    _ => return Err(format!("failed to match class: {}-{}", class, arg)),
                },
                value
            )
        } else {
            return Err(format!("failed to match class: {}-{}", class, arg));
        };

        Ok(vec![decl])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class::SeperatedClass;
}
