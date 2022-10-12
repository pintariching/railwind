use crate::class::wrap_with_everything;
use crate::modifiers::Modifier;

#[derive(Debug)]
pub struct AspectRatio {
    modifiers: Option<Vec<Modifier>>,
    class_selector: String,
    ratio: Ratio,
}

#[derive(Debug)]
pub enum Ratio {
    Auto,
    Square,
    Video,
}

impl Ratio {
    fn from_str(str: &str) -> Ratio {
        match str {
            "aspect-auto" => Ratio::Auto,
            "aspect-square" => Ratio::Square,
            "aspect-video" => Ratio::Video,
            _ => unreachable!(),
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Ratio::Auto => "auto",
            Ratio::Square => "1 / 1",
            Ratio::Video => "16 / 9",
        }
    }
}

impl AspectRatio {
    fn new(class: &str, selector: &str) -> Self {
        Self {
            modifiers: Modifier::parse_many_from_str(class),
            class_selector: class.into(),
            ratio: Ratio::from_str(selector),
        }
    }

    pub fn parse_from_str(class: &str, selector: &str) -> String {
        Self::generate_class(&Self::new(class, selector))
    }

    fn generate_class(&self) -> String {
        let class = format!(
            r#".[class-selector] {{
  aspect-ratio: {};
}}
"#,
            self.ratio.to_str()
        );

        wrap_with_everything(&class, &self.class_selector, &self.modifiers)
    }
}
