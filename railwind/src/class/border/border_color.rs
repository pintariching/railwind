use crate::class::generate_class;
use crate::colors::convert_color;

pub struct BorderColor;

impl BorderColor {
    pub fn parse_from_str(class: &str, color: &str, value: &str) -> Option<String> {
        if let Some(c) = match color {
            "inherit" => Some("inherit"),
            "current" => Some("currentColor"),
            "transparent" => Some("transparent"),
            _ => None,
        } {
            let template = format!(".[class-selector] {{\n border-color: {};\n}}\n", c);
            return Some(generate_class(class, &template));
        }

        let rgb_value = convert_color(color, value)?;
        let template = format!(".[class-selector] {{\n  --tw-bg-opacity: 1;\n  border-color: rgb({} / var(--tw-bg-opacity));\n}}\n", rgb_value);
        Some(generate_class(class, &template))
    }
}
