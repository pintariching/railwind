#[derive(Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    Horizontal,
    Vertical,
    Around,
}

impl Direction {
    pub fn new(dir: char) -> Option<Self> {
        match dir {
            't' => Some(Direction::Top),
            'b' => Some(Direction::Bottom),
            'l' => Some(Direction::Left),
            'r' => Some(Direction::Right),
            'x' => Some(Direction::Horizontal),
            'y' => Some(Direction::Vertical),
            _ => None,
        }
    }

    pub fn is_given(&self) -> bool {
        match self {
            Direction::Horizontal => false,
            Direction::Vertical => false,
            Direction::Around => false,
            _ => true,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Horizontal => true,
            _ => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::Vertical => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Direction::Top => "top".to_string(),
            Direction::Bottom => "bottom".to_string(),
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Padding {
    direction: Direction,
    class: String,
    size: String,
}

pub fn convert_size(size: &str) -> String {
    match size {
        "0" => "0px".to_string(),
        "px" => "1px".to_string(),
        "0.5" => "0.125rem".to_string(),
        "1" => "0.25rem".to_string(),
        "1.5" => "0.375rem".to_string(),
        "2" => "0.5rem".to_string(),
        "2.5" => "0.625rem".to_string(),
        "3" => "0.75rem".to_string(),
        "3.5" => "0.875rem".to_string(),
        "4" => "1rem".to_string(),
        "5" => "1.25rem".to_string(),
        "6" => "1.5rem".to_string(),
        _ => "0px".to_string(),
    }
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

    pub fn to_css(&self) -> String {
        if self.direction.is_horizontal() {
            format!(
                ".{} > *:not(:first-child) {{\n  margin-left: {};\n}}\n\n",
                self.class, self.size
            )
        } else {
            format!(
                ".{} > *:not(:first-child) {{\n  margin-top: {};\n}}\n\n",
                self.class, self.size
            )
        }
    }
}
