// pub mod margin;
// pub mod padding;
// pub mod space_between;

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
