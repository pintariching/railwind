use swc_css::ast::Rule;

use crate::{
    class::{
        convert_size,
        helpers::{new_component_value_length, new_declaration, new_rule, new_simple_block},
    },
    modifiers::Modifier,
};

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
    pub fn parse_from_str(class: &str, modifiers: &Option<String>) -> Option<Self> {
        let mut split = class.split('-');
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

            return Some(Self {
                modifiers: Modifier::parse_many_from_str(modifiers),
                direction,
                size: size_and_unit.0,
                unit: size_and_unit.1.to_string(),
                class_selector: class.to_string(),
            });
        }
        None
    }

    pub fn generate_rule(self) -> Rule {
        let declarations = match self.direction {
            Direction::Top | Direction::Bottom | Direction::Left | Direction::Right => {
                vec![new_declaration(
                    format!("padding-{}", self.direction.to_string()).as_str(),
                    vec![new_component_value_length(self.size, self.unit.as_str())],
                )]
            }
            Direction::Horizontal => vec![
                new_declaration(
                    "padding-left",
                    vec![new_component_value_length(self.size, self.unit.as_str())],
                ),
                new_declaration(
                    "padding-right",
                    vec![new_component_value_length(self.size, self.unit.as_str())],
                ),
            ],
            Direction::Vertical => vec![
                new_declaration(
                    "padding-top",
                    vec![new_component_value_length(self.size, self.unit.as_str())],
                ),
                new_declaration(
                    "padding-bottom",
                    vec![new_component_value_length(self.size, self.unit.as_str())],
                ),
            ],
            Direction::Around => vec![new_declaration(
                "padding",
                vec![new_component_value_length(self.size, self.unit.as_str())],
            )],
        };

        let block = new_simple_block_many(declarations);

        new_rule(self.modifiers, &self.class_selector, block)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        class::spacing::Direction,
        modifiers::{Modifier, PseudoClass},
    };

    use super::Padding;

    #[test]
    fn test_padding_all() {
        assert_eq!(
            Padding {
                modifiers: None,
                direction: Direction::Around,
                size: 10.,
                unit: "rem".to_string(),
                class_selector: "p-40".to_string()
            },
            Padding::parse_from_str("p-40", &None).unwrap()
        );
    }

    #[test]
    fn test_padding_directions() {
        assert_eq!(
            Padding {
                modifiers: None,
                direction: Direction::Horizontal,
                size: 10.,
                unit: "rem".to_string(),
                class_selector: "px-40".to_string()
            },
            Padding::parse_from_str("px-40", &None).unwrap()
        );

        assert_eq!(
            Padding {
                modifiers: None,
                direction: Direction::Vertical,
                size: 10.,
                unit: "rem".to_string(),
                class_selector: "py-40".to_string()
            },
            Padding::parse_from_str("py-40", &None).unwrap()
        );

        assert_eq!(
            Padding {
                modifiers: None,
                direction: Direction::Top,
                size: 10.,
                unit: "rem".to_string(),
                class_selector: "pt-40".to_string()
            },
            Padding::parse_from_str("pt-40", &None).unwrap()
        );
    }

    #[test]
    fn test_padding_with_pseudo_class() {
        assert_eq!(
            Padding {
                modifiers: Some(vec![Modifier::PseudoClass(PseudoClass::Hover)]),
                direction: Direction::Top,
                size: 10.,
                unit: "rem".to_string(),
                class_selector: "pt-40".to_string()
            },
            Padding::parse_from_str("pt-40", &Some("hover".to_string())).unwrap()
        );
    }
}
