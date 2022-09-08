use crate::class::convert_size;

#[derive(Debug)]
pub enum SpaceBetweenDirection {
    Horizontal,
    Vertical,
}

impl SpaceBetweenDirection {
    fn is_horizontal(&self) -> bool {
        match self {
            SpaceBetweenDirection::Horizontal => true,
            SpaceBetweenDirection::Vertical => false,
        }
    }
}

#[derive(Debug)]
pub struct SpaceBetween {
    direction: SpaceBetweenDirection,
    class: String,
    size: String,
}

impl SpaceBetween {
    pub fn new(mid_dash: &str, after_dash: &str) -> Self {
        let direction = match mid_dash {
            "x" => SpaceBetweenDirection::Horizontal,
            "y" => SpaceBetweenDirection::Vertical,
            _ => SpaceBetweenDirection::Vertical,
        };

        SpaceBetween {
            direction,
            class: format!("space-{}-{}", mid_dash, after_dash),
            size: convert_size(after_dash),
        }
    }

    pub fn to_css(&self) -> Option<String> {
        if self.direction.is_horizontal() {
            Some(format!(
                ".{} > *:not(:first-child) {{\n  margin-left: {};\n}}\n\n",
                self.class, self.size
            ))
        } else {
            Some(format!(
                ".{} > *:not(:first-child) {{\n  margin-top: {};\n}}\n\n",
                self.class, self.size
            ))
        }
    }
}
