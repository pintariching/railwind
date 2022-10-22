use crate::modifiers::{generate_class_selector, wrap_with_media_query, Modifier};

use self::{
    background::Background,
    border::Border,
    flex::Flex,
    layout::{AspectRatio, Container},
    spacing::{Margin, Padding},
};

pub mod background;
pub mod border;
pub mod flex;
pub mod grid;
pub mod layout;
pub mod spacing;

fn generate_class(selector: &str, template: &str) -> String {
    let modifiers = Modifier::parse_many_from_str(selector);
    let selector = generate_class_selector(selector, &modifiers);
    let class_body = wrap_with_media_query(template, &modifiers);
    class_body.replace("[class-selector]", &selector)
}

pub fn parse_class_from_str(str: &str) -> Option<String> {
    // css needs to escape ":" with "\:"
    let class_selector = str.replace(':', "\\:");

    if class_selector.ends_with("container") {
        return Some(Container::parse_from_str(&class_selector));
    }

    if let Some(last_selector) = class_selector.split("\\:").last() {
        if last_selector.starts_with("aspect") {
            return AspectRatio::parse_from_str(&class_selector, last_selector);
        }

        if last_selector.starts_with("flex") {
            return Flex::parse_from_str(&class_selector, last_selector);
        }

        if last_selector.starts_with("border") || last_selector.starts_with("rounded") {
            return Border::parse_from_str(&class_selector, last_selector);
        }

        if last_selector.contains('-') {
            if last_selector.starts_with('p') {
                return Padding::parse_from_str(&class_selector, last_selector);
            }

            if last_selector.starts_with('m') {
                return Margin::parse_from_str(&class_selector, last_selector);
            }

            if last_selector.starts_with("bg") {
                return Background::parse_from_str(&class_selector, last_selector);
            }
        }
    }

    None
}

/// Splits a class selector by dash and returns the string
/// before the dash and converts the string after the dash into a CSS unit
///
/// For example `split_by_dash("py-5")` returns a tuple ("py", "1.25rem")
pub fn split_by_dash(str: &str) -> Option<(String, String)> {
    let mut split = str.split('-');
    let before_dash = split.next();
    let after_dash = split.next();

    if let (Some(before), Some(after)) = (before_dash, after_dash) {
        if before.is_empty() || after.is_empty() {
            return None;
        }

        if let Some((size, unit)) = convert_size(after) {
            let value = format!("{}{}", size, unit);
            return Some((before.into(), value));
        }

        return None;
    }

    None
}

pub fn convert_size(size: &str) -> Option<(f32, &'static str)> {
    let result = match size {
        "0" => (0., "px"),
        "px" => (1., "px"),
        "0.5" => (0.125, "rem"),
        "1" => (0.25, "rem"),
        "1.5" => (0.375, "rem"),
        "2" => (0.5, "rem"),
        "2.5" => (0.625, "rem"),
        "3" => (0.75, "rem"),
        "3.5" => (0.875, "rem"),
        "4" => (1., "rem"),
        "5" => (1.25, "rem"),
        "6" => (1.5, "rem"),
        "7" => (1.75, "rem"),
        "8" => (2., "rem"),
        "9" => (2.25, "rem"),
        "10" => (2.5, "rem"),
        "11" => (2.75, "rem"),
        "12" => (3., "rem"),
        "14" => (3.5, "rem"),
        "16" => (4., "rem"),
        "20" => (5., "rem"),
        "24" => (6., "rem"),
        "28" => (7., "rem"),
        "32" => (8., "rem"),
        "36" => (9., "rem"),
        "40" => (10., "rem"),
        "44" => (11., "rem"),
        "48" => (12., "rem"),
        "52" => (13., "rem"),
        "56" => (14., "rem"),
        "60" => (15., "rem"),
        "64" => (16., "rem"),
        "72" => (18., "rem"),
        "80" => (20., "rem"),
        "96" => (24., "rem"),
        _ => return None,
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_dash() {
        assert!(split_by_dash("py-5").is_some());
        assert_eq!(
            split_by_dash("py-5").unwrap(),
            ("py".into(), "1.25rem".into())
        );

        assert!(split_by_dash("-5").is_none());
        assert!(split_by_dash("py-").is_none());
        assert!(split_by_dash("py-f").is_none());
    }
}
