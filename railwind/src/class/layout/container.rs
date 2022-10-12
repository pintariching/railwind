use crate::class::wrap_with_everything;
use crate::modifiers::Modifier;

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
        let class = r#".[class-selector] {
  width: 100%;
}

@media (min-width: 640px) {
  .[class-selector] {
    max-width: 640px;
  }
}

@media (min-width: 768px) {
  .[class-selector] {
    max-width: 768px;
  }
}

@media (min-width: 1024px) {
  .[class-selector] {
    max-width: 1024px;
  }
}

@media (min-width: 1280px) {
  .[class-selector] {
    max-width: 1280px;
  }
}

@media (min-width: 1536px) {
  .[class-selector] {
    max-width: 1536px;
  }
}
"#
        .to_string();

        wrap_with_everything(&class, &self.class_selector, &self.modifiers)
    }
}
