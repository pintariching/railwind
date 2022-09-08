use crate::class::convert_size;

use super::Direction;

#[derive(Debug)]
pub struct Margin {
    direction: Direction,
    class: String,
    size: String,
}

impl Margin {
    pub fn new(before_dash: &str, after_dash: &str) -> Self {
        let mut direction = Direction::Around;

        if before_dash.len() > 1 {
            direction = Direction::new(before_dash.chars().nth(1).unwrap()).unwrap();
        }

        Margin {
            direction,
            class: format!("{}-{}", before_dash, after_dash),
            size: convert_size(after_dash),
        }
    }

    pub fn to_css(&self) -> String {
        if self.direction.is_given() {
            format!(
                ".{} {{\n  margin-{}: {};\n}}\n\n",
                self.class,
                self.direction.to_string(),
                self.size
            )
            .to_string()
        } else {
            if self.direction.is_horizontal() {
                format!(
                    ".{} {{\n  margin-left: {};\n  margin-right: {};\n}}\n\n",
                    self.class, self.size, self.size
                )
            } else if self.direction.is_vertical() {
                format!(
                    ".{} {{\n  margin-top: {};\n  margin-bottom: {};\n}}\n\n",
                    self.class, self.size, self.size
                )
            } else {
                format!(".{} {{\n  margin: {};\n}}\n\n", self.class, self.size)
            }
        }
    }
}
