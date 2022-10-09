use crate::modifiers::{modifiers_to_class_selector, wrap_with_media_query, Modifier};

#[derive(Debug)]
pub struct Container {
    modifiers: Option<Vec<Modifier>>,
    class_selector: String,
}

impl Container {
    fn new(class: &str) -> Self {
        Self {
            modifiers: Modifier::parse_many_from_str(class),
            class_selector: class.into(),
        }
    }

    pub fn parse_from_str(class: &str) -> String {
        Self::generate_class(&Self::new(class))
    }

    fn generate_class(&self) -> String {
        let mut class = r#".[class-selector] {{
  width: 100%;
}}

@media (min-width: 640px) {{
  .[class-selector] {{
    max-width: 640px;
  }}
}}

@media (min-width: 768px) {{
  .[class-selector] {{
    max-width: 768px;
  }}
}}

@media (min-width: 1024px) {{
  .[class-selector] {{
    max-width: 1024px;
  }}
}}

@media (min-width: 1280px) {{
  .[class-selector] {{
    max-width: 1280px;
  }}
}}

@media (min-width: 1536px) {{
  .[class-selector] {{
    max-width: 1536px;
  }}
}}
"#
        .to_string();

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
