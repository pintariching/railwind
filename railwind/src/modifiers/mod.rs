mod group;
mod media_query;
mod peer;
mod pseudo_class;
mod pseudo_element;

pub use group::Group;
pub use media_query::MediaQuery;
pub use peer::Peer;
pub use pseudo_class::PseudoClass;
pub use pseudo_element::PseudoElement;

use crate::warning::{Position, Warning, WarningType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    PseudoClass(PseudoClass),
    PseudoElement(PseudoElement),
    MediaQuery(MediaQuery),
    Group(Group),
    Peer(Peer),
}

impl State {
    pub fn new(raw_class: &str, value: &str, position: &Position) -> Result<Self, Warning> {
        if let Some(mq) = MediaQuery::new(value) {
            return Ok(State::MediaQuery(mq));
        }

        if let Some(pc) = PseudoClass::new(value) {
            return Ok(State::PseudoClass(pc));
        }

        if let Some(pe) = PseudoElement::new(value) {
            return Ok(State::PseudoElement(pe));
        }

        if let Some(g) = Group::new(value) {
            return Ok(State::Group(g));
        }

        if let Some(p) = Peer::new(value) {
            return Ok(State::Peer(p));
        }

        Err(Warning::new(
            raw_class,
            &position,
            WarningType::StateNotFound(value.to_string()),
        ))
    }
}

pub fn generate_state_selector(states: Vec<State>) -> String {
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
        return format!(":{}", pe);
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
        assert_eq!(generate_state_selector(states), "active".to_string());

        let states = vec![
            State::PseudoClass(PseudoClass::Active),
            State::PseudoClass(PseudoClass::Hover),
        ];
        assert_eq!(generate_state_selector(states), "active:hover".to_string());

        let states = vec![State::PseudoElement(PseudoElement::Before)];
        assert_eq!(generate_state_selector(states), ":before".to_string());

        let states = vec![
            State::PseudoElement(PseudoElement::Before),
            State::PseudoElement(PseudoElement::After),
        ];
        assert_eq!(
            generate_state_selector(states),
            ":before::after".to_string()
        );

        let states = vec![
            State::PseudoClass(PseudoClass::Active),
            State::PseudoElement(PseudoElement::Before),
            State::PseudoClass(PseudoClass::Hover),
            State::PseudoElement(PseudoElement::After),
        ];
        assert_eq!(
            generate_state_selector(states),
            "active:hover::before::after".to_string()
        );
    }
}
