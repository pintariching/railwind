use crate::class::Decl;

#[derive(Debug, PartialEq, Hash)]
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

    pub fn to_decl(self) -> Decl {
        match self {
            Self::SROnly => Decl::Vec(vec![
                "position: absolute".into(),
                "width: 1px".into(),
                "height: 1px".into(),
                "padding: 0".into(),
                "margin: -1px".into(),
                "overflow: hidden".into(),
                "clip: rect(0, 0, 0, 0)".into(),
                "white-space: nowrap".into(),
                "border-width: 0".into(),
            ]),
            Self::NotSROnly => Decl::Vec(vec![
                "position: static".into(),
                "width: auto".into(),
                "height: auto".into(),
                "padding: 0".into(),
                "margin: 0".into(),
                "overflow: visible".into(),
                "clip: auto".into(),
                "white-space: normal".into(),
            ]),
        }
    }
}
