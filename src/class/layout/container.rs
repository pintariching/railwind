#[derive(Debug)]
pub struct Container {
    class: String,
}

impl Container {
    pub fn new() -> Self {
        Container {
            class: "container".to_string(),
        }
    }

    pub fn to_css(&self) -> String {
        format!(".{} {{\n  width: 100%;\n}}\n\n", self.class)
    }
}
