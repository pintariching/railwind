use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub content: Vec<String>,
}
