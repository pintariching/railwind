#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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
            "before" => Self::Before,
            "after" => Self::After,
            "placeholder" => Self::Placeholder,
            "file" => Self::File,
            "marker" => Self::Marker,
            "selection" => Self::Selection,
            "first-line" => Self::FirstLine,
            "first-letter" => Self::FirstLetter,
            "last-line" => Self::LastLine,
            "backdrop" => Self::Backdrop,
            _ => return None,
        };

        Some(pe)
    }

    pub fn to_static_str(self) -> &'static str {
        match self {
            Self::Before => "before",
            Self::After => "after",
            Self::Placeholder => "placeholder",
            Self::File => "file-selector-button",
            Self::Marker => "marker",
            Self::Selection => "selection",
            Self::FirstLine => "first-line",
            Self::FirstLetter => "first-letter",
            Self::LastLine => "last-line",
            Self::Backdrop => "backdrop",
        }
    }
}
