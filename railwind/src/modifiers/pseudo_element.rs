#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn new(value: &str) -> Option<Self> {
        let pe = match value {
            "before" => PseudoElement::Before,
            "after" => PseudoElement::After,
            "placeholder" => PseudoElement::Placeholder,
            "file" => PseudoElement::File,
            "marker" => PseudoElement::Marker,
            "selection" => PseudoElement::Selection,
            "first-line" => PseudoElement::FirstLine,
            "first-letter" => PseudoElement::FirstLetter,
            "last-line" => PseudoElement::LastLine,
            "backdrop" => PseudoElement::Backdrop,
            _ => return None,
        };

        Some(pe)
    }

    pub fn to_static_str(self) -> &'static str {
        match self {
            PseudoElement::Before => "before",
            PseudoElement::After => "after",
            PseudoElement::Placeholder => "placeholder",
            PseudoElement::File => "file-selector-button",
            PseudoElement::Marker => "marker",
            PseudoElement::Selection => "selection",
            PseudoElement::FirstLine => "first-line",
            PseudoElement::FirstLetter => "first-letter",
            PseudoElement::LastLine => "last-line",
            PseudoElement::Backdrop => "backdrop",
        }
    }
}
