use crate::class::convert_size;
use crate::modifiers::{modifiers_to_class_selector, wrap_with_media_query, Modifier};

use super::Direction;

#[derive(Debug, PartialEq)]
pub struct Padding {
    modifiers: Option<Vec<Modifier>>,
    direction: Direction,
    size: f32,
    unit: String,
    class_selector: String,
}

impl Padding {
    fn new(class: &str, selector: &str) -> Self {
        let mut split = selector.split('-');
        let before_dash = split.next();
        let after_dash = split.next();

        if let (Some(bef), Some(aft)) = (before_dash, after_dash) {
            let mut direction = Direction::Around;

            if let Some(dir) = bef.chars().nth(1) {
                if let Some(d) = Direction::from_char(dir) {
                    direction = d;
                }
            }

            let size_and_unit = convert_size(aft);

            return Self {
                modifiers: Modifier::parse_many_from_str(class),
                direction,
                size: size_and_unit.0,
                unit: size_and_unit.1.to_string(),
                class_selector: class.into(),
            };
        }

        unreachable!()
    }

    pub fn parse_from_str(class: &str, selector: &str) -> String {
        Self::generate_class(&Self::new(class, selector))
    }

    fn generate_class(&self) -> String {
        let mut class = format!(
            r#".[class-selector] {{
  {}
}}
"#,
            match self.direction {
                Direction::Around => format!("padding: {}{};", self.size, self.unit),
                Direction::Vertical => format!(
                    "padding-top: {}{};\n  padding-bottom: {}{};",
                    self.size, self.unit, self.size, self.unit
                ),

                Direction::Horizontal => format!(
                    "padding-left: {}{};\n  padding-right: {}{};",
                    self.size, self.unit, self.size, self.unit
                ),
                _ => format!(
                    "padding-{}: {}{};",
                    self.direction.to_string(),
                    self.size,
                    self.unit
                ),
            }
        );

        let mut class_selector = self.class_selector.clone();

        if let Some(modifiers) = &self.modifiers {
            class_selector = format!(
                "{}:{}",
                class_selector,
                modifiers_to_class_selector(modifiers)
            );

            class = wrap_with_media_query(class, modifiers);
        }

        class.replace("[class-selector]", &class_selector)
    }
}
