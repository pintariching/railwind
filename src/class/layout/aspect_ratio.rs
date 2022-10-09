use crate::modifiers::{modifiers_to_class_selector, wrap_with_media_query, Modifier};

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
        let mut class = format!(
            r#".[class-selector] {{
  aspect-ratio: {};
}}
"#,
            self.ratio.to_str()
        );

        let mut class_selector = self.class_selector.clone();

        if let Some(modifiers) = &self.modifiers {
            class_selector = format!(
                "{}:{}",
                class_selector,
                modifiers_to_class_selector(modifiers)
            );

            class = wrap_with_media_query(class, modifiers);
        }

        class.replace("[class-selector]", &class_selector)
    }
}
