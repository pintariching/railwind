#[derive(Debug)]
pub struct AspectRatio {
    class: String,
    ratio: Option<Ratio>,
}

#[derive(Debug)]
enum Ratio {
    Auto,
    Square,
    Video,
}

impl Ratio {
    fn to_string(&self) -> String {
        match self {
            Ratio::Auto => "auto",
            Ratio::Square => "1 / 1",
            Ratio::Video => "16 / 9",
        }
        .to_string()
    }
}

impl AspectRatio {
    pub fn new(before_dash: &str, after_dash: &str) -> Self {
        let ratio = match after_dash {
            "auto" => Some(Ratio::Auto),
            "square" => Some(Ratio::Square),
            "video" => Some(Ratio::Video),
            _ => None,
        };

        AspectRatio {
            class: format!("{}-{}", before_dash, after_dash),
            ratio: ratio,
        }
    }

    pub fn to_css(&self) -> Option<String> {
        if let Some(ratio) = &self.ratio {
            return Some(format!(
                ".{} {{\n  aspect-ratio: {};\n}}\n\n",
                self.class,
                ratio.to_string()
            ));
        } else {
            None
        }
    }
}
