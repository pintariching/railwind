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
    pub fn parse_from_str(str: &str) -> Option<Modifier> {
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
}

#[cfg(test)]
mod tests {
    use super::pseudo_class::PseudoClass;
    use super::pseudo_element::PseudoElement;
    use super::Modifier;

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
}
