mod media_query;
mod pseudo_class;
mod pseudo_element;

use std::str::FromStr;

pub use media_query::MediaQuery;
pub use pseudo_class::PseudoClass;
pub use pseudo_element::PseudoElement;

use crate::{traits::ToStaticStr, warning::WarningType};

#[derive(Debug, PartialEq)]
pub enum State {
    PseudoClass(PseudoClass),
    PseudoElement(PseudoElement),
    MediaQuery(MediaQuery),
}

impl State {
    pub fn new(value: &str, warnings: &mut Vec<WarningType>) -> Option<Self> {
        let warning_count = warnings.len();

        if let Some(pc) = PseudoClass::from_str(value).ok() {
            return Some(State::PseudoClass(pc));
        }

        if let Some(pe) = PseudoElement::from_str(value).ok() {
            return Some(State::PseudoElement(pe));
        }

        if let Some(mq) = MediaQuery::from_str(value).ok() {
            return Some(State::MediaQuery(mq));
        }

        // prevents duplicate error messages
        // if a message has already been pushed to warnings, then there was a problem elsewhere
        if warning_count == warnings.len() {
            warnings.push(WarningType::StateNotFound(value.into()));
        }

        None
    }
}

pub fn generate_state_selector(states: &Vec<State>) -> String {
    let mut pseudo_classes = Vec::new();
    let mut pseudo_elements = Vec::new();

    for state in states {
        match state {
            State::PseudoClass(pc) => pseudo_classes.push(pc.to_static_str()),
            State::PseudoElement(pe) => pseudo_elements.push(pe.to_static_str()),
            _ => (),
        }
    }

    let pc = pseudo_classes.join(":");
    let pe = pseudo_elements.join("::");

    if pe.is_empty() {
        return pc;
    }

    if pc.is_empty() {
        return pe;
    }

    format!("{}::{}", pc, pe)
}

#[cfg(test)]
mod tests {
    use crate::modifiers::{PseudoClass, PseudoElement};

    use super::*;

    #[test]
    fn test_generate_state_selector() {
        let states = vec![State::PseudoClass(PseudoClass::Active)];
        assert_eq!(generate_state_selector(&states), "active".to_string());

        let states = vec![
            State::PseudoClass(PseudoClass::Active),
            State::PseudoClass(PseudoClass::Hover),
        ];
        assert_eq!(generate_state_selector(&states), "active:hover".to_string());

        let states = vec![State::PseudoElement(PseudoElement::Before)];
        assert_eq!(generate_state_selector(&states), "before".to_string());

        let states = vec![
            State::PseudoElement(PseudoElement::Before),
            State::PseudoElement(PseudoElement::After),
        ];
        assert_eq!(
            generate_state_selector(&states),
            "before::after".to_string()
        );

        let states = vec![
            State::PseudoClass(PseudoClass::Active),
            State::PseudoElement(PseudoElement::Before),
            State::PseudoClass(PseudoClass::Hover),
            State::PseudoElement(PseudoElement::After),
        ];
        assert_eq!(
            generate_state_selector(&states),
            "active:hover::before::after".to_string()
        );
    }
}
