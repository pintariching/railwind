mod types;

use types::*;

use crate::class::Decl;

#[derive(Debug, PartialEq, Hash)]
pub enum Accessibility {
    ScreenReaders(ScreenReaders),
}

impl Accessibility {
    pub fn new(value: &str) -> Option<Self> {
        let accessibility = if let Some(padding) = ScreenReaders::new(value) {
            Self::ScreenReaders(padding)
        } else {
            return None;
        };

        Some(accessibility)
    }

    pub fn to_decl(self) -> Decl {
        match self {
            Self::ScreenReaders(s) => s.to_decl(),
        }
    }
}
