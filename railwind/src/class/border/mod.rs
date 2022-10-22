use self::border_color::BorderColor;
use self::border_radius::BorderRadius;
use self::border_width::BorderWidth;

use super::generate_class;

mod border_color;
mod border_radius;
mod border_width;

pub struct Border;

impl Border {
    pub fn parse_from_str(class: &str, border: &str) -> Option<String> {
        if let Some(brd) = BorderRadius::parse_from_str(class, border) {
            return Some(brd);
        }

        if let Some(brd) = BorderWidth::parse_from_str(class, border) {
            return Some(brd);
        }

        let mut split = border.split('-');
        split.next();
        let utility = split.next().unwrap_or("");
        let value = split.next().unwrap_or("");

        let declaration = match utility {
            "solid" | "dashed" | "dotted" | "double" | "hidden" | "none" if value.is_empty() => {
                format!("border-style: {};", utility)
            }
            _ => return BorderColor::parse_from_str(class, utility, value),
        };

        let template = format!(".[class-selector] {{\n  {}\n}}\n", declaration);
        Some(generate_class(class, &template))
    }
}
