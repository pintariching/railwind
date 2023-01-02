mod types;

use types::*;

use crate::class::Decl;

#[derive(Debug)]
pub enum Accessibility {
    ScreenReaders(ScreenReaders),
}

impl Accessibility {
    pub fn new(value: &str) -> Option<Self> {
        let accessibility = if let Some(padding) = ScreenReaders::new(value) {
            Accessibility::ScreenReaders(padding)
        } else {
            return None;
        };

        Some(accessibility)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Accessibility::ScreenReaders(s) => s.to_decl(),
        }
    }
}
