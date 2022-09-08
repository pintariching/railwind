use crate::class::convert_size;

use super::Direction;

#[derive(Debug)]
pub struct Padding {
    direction: Direction,
    class: String,
    size: String,
}

impl Padding {
    pub fn new(before_dash: &str, after_dash: &str) -> Self {
        let mut direction = Direction::Around;

        if before_dash.len() > 1 {
            direction = Direction::new(before_dash.chars().nth(1).unwrap()).unwrap();
        }

        Padding {
            direction,
            class: format!("{}-{}", before_dash, after_dash),
            size: convert_size(after_dash),
        }
    }

    pub fn to_css(&self) -> String {
        if self.direction.is_given() {
            format!(
                ".{} {{\n  padding-{}: {};\n}}\n",
                self.class,
                self.direction.to_string(),
                self.size
            )
            .to_string()
        } else {
            if self.direction.is_horizontal() {
                format!(
                    ".{} {{\n  padding-left: {};\n  padding-right: {};\n}}\n\n",
                    self.class, self.size, self.size
                )
            } else if self.direction.is_vertical() {
                format!(
                    ".{} {{\n  padding-top: {};\n  padding-bottom: {};\n}}\n\n",
                    self.class, self.size, self.size
                )
            } else {
                format!(".{} {{\n  padding: {};\n}}\n\n", self.class, self.size)
            }
        }
    }
}
