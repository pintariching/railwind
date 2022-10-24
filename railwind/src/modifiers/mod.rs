mod media_query;
//mod parent_sibling;
mod pseudo_class;
mod pseudo_element;

use std::fmt::format;

pub use media_query::MediaQuery;
//pub use parent_sibling::{Group, Peer};
pub use pseudo_class::PseudoClass;
pub use pseudo_element::PseudoElement;

use crate::{class::SeperatedClass, utils::indent_string};

#[derive(Debug, PartialEq, Eq)]
pub enum Modifier {
    PseudoClass(PseudoClass),
    PseudoElement(PseudoElement),
    MediaQuery(MediaQuery),
}

impl Modifier {
    pub fn from_str(str: &str) -> Option<Self> {
        if let Some(modifier) = PseudoClass::parse_from_str(str) {
            return Some(Modifier::PseudoClass(modifier));
        }

        if let Some(modifier) = PseudoElement::parse_from_str(str) {
            return Some(Modifier::PseudoElement(modifier));
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

    pub fn media_query_from_str(str: &str) -> Option<Modifier> {
        if let Some(media_query) = MediaQuery::parse_from_str(str) {
            return Some(Modifier::MediaQuery(media_query));
        }
        None
    }
}

// pub fn modifiers_to_string(modifiers: Vec<Modifier>) -> String {
//     let pseudo_classes: Vec<&str> = modifiers
//         .iter()
//         .filter_map(|m| m.pseudo_class())
//         .map(|m| m.as_str())
//         .collect();

//     let pseudo_elements: Vec<&str> = modifiers
//         .iter()
//         .filter_map(|m| m.pseudo_element())
//         .map(|m| m.as_str())
//         .collect();

//     let mut class_selector = String::new();

//     if !pseudo_classes.is_empty() {
//         class_selector.push_str(&pseudo_classes.join(":"));
//     }

//     if !pseudo_elements.is_empty() {
//         class_selector.push_str("::");
//         class_selector.push_str(&pseudo_elements.join("::"));
//     }

//     class_selector
// }

// pub fn wrap_with_media_query(generated_class: &str, seperated_class: &SeperatedClass) -> String {
//     let mut c = generated_class.to_string();

//     if let Some(m) = Modifier::media_query_from_str(seperated_class.pseudo_elements) {
//         let media_queries: Vec<&MediaQuery> = m.iter().filter_map(|m| m.media_query()).collect();

//         if !media_queries.is_empty() {
//             for query in media_queries {
//                 match query {
//                     MediaQuery::Sm
//                     | MediaQuery::Md
//                     | MediaQuery::Lg
//                     | MediaQuery::Xl
//                     | MediaQuery::Xxl
//                     | MediaQuery::Dark
//                     | MediaQuery::MotionReduce
//                     | MediaQuery::MotionSafe
//                     | MediaQuery::ContrastMore
//                     | MediaQuery::ContrastLess
//                     | MediaQuery::Portrait
//                     | MediaQuery::Landscape => {
//                         c = format!("@media ({}) {{\n{}}}\n", query.as_str(), indent_string(&c));
//                     }
//                     _ => (),
//                 }
//             }
//         }
//     }

//     c
// }

#[cfg(test)]
mod tests {
    use super::pseudo_class::PseudoClass;
    use super::pseudo_element::PseudoElement;
    use super::{MediaQuery, Modifier};

    #[test]
    fn test_modifier_parse_from_str() {
        assert_eq!(
            Modifier::from_str("hover"),
            Some(Modifier::PseudoClass(PseudoClass::Hover))
        );

        assert_eq!(
            Modifier::from_str("placeholder"),
            Some(Modifier::PseudoElement(PseudoElement::Placeholder))
        );

        assert_eq!(Modifier::from_str("something"), None);
    }
}
