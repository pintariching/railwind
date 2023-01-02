use crate::class::Decl;

#[derive(Debug)]
pub enum ScreenReaders {
    SROnly,
    NotSROnly,
}

impl ScreenReaders {
    pub fn new(value: &str) -> Option<Self> {
        let value = match value {
            "sr-only" => Self::SROnly,
            "not-sr-only" => Self::NotSROnly,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::SROnly => {
                let mut strings = vec![];
                strings.push(String::from("position: absolute"));
                strings.push(String::from("width: 1px"));
                strings.push(String::from("height: 1px"));
                strings.push(String::from("padding: 0"));
                strings.push(String::from("margin: -1px"));
                strings.push(String::from("overflow: hidden"));
                strings.push(String::from("clip: rect(0, 0, 0, 0)"));
                strings.push(String::from("white-space: nowrap"));
                strings.push(String::from("border-width: 0"));
                Some(Decl::Multiple(strings))
            }
            Self::NotSROnly => {
                let mut strings = vec![];
                strings.push(String::from("position: static"));
                strings.push(String::from("width: auto"));
                strings.push(String::from("height: auto"));
                strings.push(String::from("padding: 0"));
                strings.push(String::from("margin: 0"));
                strings.push(String::from("overflow: visible"));
                strings.push(String::from("clip: auto"));
                strings.push(String::from("white-space: normal"));
                Some(Decl::Multiple(strings))
            }
        }
    }
}
