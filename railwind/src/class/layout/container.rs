use crate::class::generate_class;

#[derive(Debug)]
pub struct Container;

impl Container {
    pub fn parse_from_str(class: &str) -> String {
        generate_class(
            class,
            r#".[class-selector] {
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
"#,
        )
    }
}
