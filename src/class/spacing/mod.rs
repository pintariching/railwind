// pub mod margin;
mod padding;
// pub mod space_between;

pub use padding::Padding;

#[derive(Debug, PartialEq)]
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
    pub fn from_char(dir: char) -> Option<Self> {
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

    pub fn to_string(&self) -> &'static str {
        match self {
            Direction::Top => "top",
            Direction::Bottom => "bottom",
            Direction::Left => "left",
            Direction::Right => "right",
            _ => "",
        }
    }
}
