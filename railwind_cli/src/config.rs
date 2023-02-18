use std::collections::HashMap;

use railwind::CollectionOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub content: Vec<String>,
    pub extend_collection_options: Option<HashMap<String, CollectionOptions>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            content: vec!["index.html".to_string()],
            extend_collection_options: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let config = r#"
            (
                content: [
                    "*.{html,rs}"
                ],
                extend_collection_options: Some({
                    "rs": Html
                })
            )"#;

        let ron = ron::from_str::<Config>(config);
        assert!(ron.is_ok());

        let unwrap_ron = ron.unwrap();
        assert_eq!(unwrap_ron.content, vec!["*.{html,rs}".to_string()]);
        assert!(matches!(
            unwrap_ron
                .extend_collection_options
                .unwrap()
                .get("rs")
                .unwrap()
                .clone(),
            CollectionOptions::Html
        ));
    }
}
