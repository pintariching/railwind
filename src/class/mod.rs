use self::spacing::{Margin, Padding};

pub mod spacing;

#[derive(Debug)]
pub enum Class {
    Padding(Padding),
    Margin(Margin),
}

impl Class {
    pub fn to_css(&self) -> String {
        match self {
            Class::Padding(c) => c.to_css(),
            Class::Margin(c) => c.to_css(),
        }
    }
}
