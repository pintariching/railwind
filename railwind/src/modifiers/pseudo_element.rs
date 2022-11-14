use std::str::FromStr;

use crate::traits::ToStaticStr;

#[derive(Debug, PartialEq, Eq)]
pub enum PseudoElement {
    Before,
    After,
    Placeholder,
    File,
    Marker,
    Selection,
    FirstLine,
    FirstLetter,
    LastLine,
    Backdrop,
}

impl FromStr for PseudoElement {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "before" => Ok(PseudoElement::Before),
            "after" => Ok(PseudoElement::After),
            "placeholder" => Ok(PseudoElement::Placeholder),
            "file" => Ok(PseudoElement::File),
            "marker" => Ok(PseudoElement::Marker),
            "selection" => Ok(PseudoElement::Selection),
            "first-line" => Ok(PseudoElement::FirstLine),
            "first-letter" => Ok(PseudoElement::FirstLetter),
            "last-line" => Ok(PseudoElement::LastLine),
            "backdrop" => Ok(PseudoElement::Backdrop),
            _ => Err(()),
        }
    }
}

impl ToStaticStr for &PseudoElement {
    fn to_static_str(&self) -> &'static str {
        match self {
            PseudoElement::Before => "before",
            PseudoElement::After => "after",
            PseudoElement::Placeholder => "placeholder",
            PseudoElement::File => "file",
            PseudoElement::Marker => "marker",
            PseudoElement::Selection => "selection",
            PseudoElement::FirstLine => "first-line",
            PseudoElement::FirstLetter => "first-letter",
            PseudoElement::LastLine => "last-line",
            PseudoElement::Backdrop => "backdrop",
        }
    }
}
