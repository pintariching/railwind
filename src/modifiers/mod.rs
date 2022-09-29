mod media_query;
mod parent_sibling;
mod pseudo_class;
mod pseudo_element;

pub use media_query::MediaQuery;
pub use parent_sibling::{Group, Peer};
pub use pseudo_class::PseudoClass;
pub use pseudo_element::PseudoElement;

#[derive(Debug, PartialEq)]
pub enum Modifier {
    PseudoClass(PseudoClass),
    PseudoElement(PseudoElement),
    MediaQuery(MediaQuery),
}

impl Modifier {
    fn parse_from_str(str: &str) -> Option<Self> {
        if let Some(modifier) = PseudoClass::parse_from_str(str) {
            return Some(Modifier::PseudoClass(modifier));
        }

        if let Some(modifier) = PseudoElement::parse_from_str(str) {
            return Some(Modifier::PseudoElement(modifier));
        }

        if let Some(modifier) = MediaQuery::parse_from_str(str) {
            return Some(Modifier::MediaQuery(modifier));
        }

        None
    }

    pub fn to_string(&self) -> String {
        match self {
            Modifier::PseudoClass(m) => m.as_str().to_string(),
            Modifier::PseudoElement(m) => m.as_str().to_string(),
            Modifier::MediaQuery(m) => m.as_str().to_string(),
        }
    }

    pub fn parse_many_from_str(modifiers: &str) -> Option<Vec<Self>> {
        if modifiers.contains("\\:") {
            let modifiers: Vec<Modifier> = modifiers
                .split("\\:")
                .filter_map(|m| Modifier::parse_from_str(m))
                .collect();

            return Some(modifiers);
        }

        if let Some(modif) = Modifier::parse_from_str(&modifiers) {
            return Some(vec![modif]);
        }

        None
    }

    pub fn pseudo_class(&self) -> Option<&PseudoClass> {
        match self {
            Modifier::PseudoClass(m) => Some(m),
            _ => None,
        }
    }

    pub fn pseudo_element(&self) -> Option<&PseudoElement> {
        match self {
            Modifier::PseudoElement(m) => Some(m),
            _ => None,
        }
    }

    pub fn media_query(&self) -> Option<&MediaQuery> {
        match self {
            Modifier::MediaQuery(m) => Some(m),
            _ => None,
        }
    }
}

pub fn modifiers_to_class_selector(modifiers: &Vec<Modifier>) -> String {
    let pseudo_classes: Vec<&str> = modifiers
        .iter()
        .filter_map(|m| m.pseudo_class())
        .map(|m| m.as_str())
        .collect();

    let pseudo_elements: Vec<&str> = modifiers
        .iter()
        .filter_map(|m| m.pseudo_element())
        .map(|m| m.as_str())
        .collect();

    let mut class_selector = String::new();

    if !pseudo_classes.is_empty() {
        class_selector.push_str(&pseudo_classes.join(":"));
    }

    if !pseudo_elements.is_empty() {
        class_selector.push_str("::");
        class_selector.push_str(&pseudo_elements.join("::"));
    }

    class_selector
}

pub fn wrap_with_media_query(mut class: String, modifiers: &Vec<Modifier>) -> String {
    let media_queries: Vec<&MediaQuery> =
        modifiers.iter().filter_map(|m| m.media_query()).collect();

    if !media_queries.is_empty() {
        for query in media_queries {
            match query {
                MediaQuery::Sm
                | MediaQuery::Md
                | MediaQuery::Lg
                | MediaQuery::Xl
                | MediaQuery::Xxl
                | MediaQuery::Dark
                | MediaQuery::MotionReduce
                | MediaQuery::MotionSafe
                | MediaQuery::ContrastMore
                | MediaQuery::ContrastLess
                | MediaQuery::Portrait
                | MediaQuery::Landscape => {
                    class = format!(
                        r#"@media ({}) {{
{}
}}"#,
                        query.as_str(),
                        class
                    );
                }
                _ => (),
            }
        }
    }

    class
}

#[cfg(test)]
mod tests {
    use super::pseudo_class::PseudoClass;
    use super::pseudo_element::PseudoElement;
    use super::{modifiers_to_string, Modifier};

    #[test]
    fn test_modifier_parse_from_str() {
        assert_eq!(
            Modifier::parse_from_str("hover"),
            Some(Modifier::PseudoClass(PseudoClass::Hover))
        );

        assert_eq!(
            Modifier::parse_from_str("placeholder"),
            Some(Modifier::PseudoElement(PseudoElement::Placeholder))
        );

        assert_eq!(Modifier::parse_from_str("something"), None);
    }

    #[test]
    fn test_modifiers_to_string() {
        let modifiers = Modifier::parse_many_from_str("hover\\:before\\:target").unwrap();

        assert_eq!(modifiers_to_string(&modifiers), ":hover:target::before")
    }
}
