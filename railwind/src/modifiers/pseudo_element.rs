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

impl PseudoElement {
    pub fn as_str(&self) -> &'static str {
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

    pub fn parse_from_str(str: &str) -> Option<PseudoElement> {
        match str {
            "before" => Some(PseudoElement::Before),
            "after" => Some(PseudoElement::After),
            "placeholder" => Some(PseudoElement::Placeholder),
            "file" => Some(PseudoElement::File),
            "marker" => Some(PseudoElement::Marker),
            "selection" => Some(PseudoElement::Selection),
            "first-line" => Some(PseudoElement::FirstLine),
            "first-letter" => Some(PseudoElement::FirstLetter),
            "last-line" => Some(PseudoElement::LastLine),
            "backdrop" => Some(PseudoElement::Backdrop),
            _ => None,
        }
    }
}
