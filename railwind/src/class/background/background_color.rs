use crate::{class::generate_class, colors::convert_color};

pub struct BackgroundColor;

impl BackgroundColor {
    pub fn parse_from_str(class: &str, color: &str, value: &str) -> Option<String> {
        if let Some(c) = match color {
            "inherit" => Some("inherit"),
            "current" => Some("currentColor"),
            "transparent" => Some("transparent"),
            _ => None,
        } {
            let template = format!(".[class-selector] {{\n background-color: {};\n}}\n", c);
            return Some(generate_class(class, &template));
        }

        let rgb_value = convert_color(color, value)?;
        let template = format!(".[class-selector] {{\n  --tw-bg-opacity: 1;\n  background-color: rgb({} / var(--tw-bg-opacity));\n}}\n", rgb_value);
        return Some(generate_class(class, &template));
    }
}
