use swc_css::ast::Rule;

use crate::modifiers::Modifier;

use self::layout::{AspectRatio, Container};

mod helpers;
pub mod layout;
pub mod spacing;

#[derive(Debug)]
pub enum Class {
    AspectRatio(AspectRatio),
    Container(Container),
    // Padding(Padding),
    // Margin(Margin),
    // SpaceBetween(SpaceBetween),
}

impl Class {
    pub fn to_css(&self) -> Option<String> {
        todo!()
    }

    pub fn parse_from_str(str: &str) -> Option<Self> {
        if str.ends_with("container") {
            return Some(Class::Container(Container(BaseClass::parse_from_str(&str))));
        }

        if str.contains("aspect-") {
            if let Some(aspect_ratio) = AspectRatio::new(str) {
                return Some(Class::AspectRatio(aspect_ratio));
            }
        }

        None
    }

    pub fn to_qualified_rule(self) -> Rule {
        match self {
            Class::Container(c) => c.generate_rule(),
            Class::AspectRatio(c) => c.generate_rule(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BaseClass(pub Vec<Modifier>);

impl BaseClass {
    pub fn default() -> Self {
        Self(Vec::new())
    }

    pub fn parse_from_str(class: &str) -> Option<Self> {
        if class.contains(':') {
            let modifiers: Vec<Modifier> = class
                .split(':')
                .filter_map(|m| Modifier::parse_from_str(m))
                .collect();

            return Some(BaseClass(modifiers));
        }

        None
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        self.0.iter().map(|m| m.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::modifiers::{MediaQuery, Modifier, PseudoClass, PseudoElement};

    use super::BaseClass;

    #[test]
    fn test_base_class_parse_from_str() {
        assert_eq!(
            BaseClass::parse_from_str("first-line:hover:sm"),
            Some(BaseClass(vec![
                Modifier::PseudoElement(PseudoElement::FirstLine),
                Modifier::PseudoClass(PseudoClass::Hover),
                Modifier::MediaQuery(MediaQuery::Sm)
            ]))
        )
    }
}

// pub fn convert_size(size: &str) -> String {
//     match size {
//         "0" => "0px",
//         "px" => "1px",
//         "0.5" => "0.125rem",
//         "1" => "0.25rem",
//         "1.5" => "0.375rem",
//         "2" => "0.5rem",
//         "2.5" => "0.625rem",
//         "3" => "0.75rem",
//         "3.5" => "0.875rem",
//         "4" => "1rem",
//         "5" => "1.25rem",
//         "6" => "1.5rem",
//         "7" => "1.75rem",
//         "8" => "2rem",
//         "9" => "2.25rem",
//         "10" => "2.5rem",
//         "11" => "2.75",
//         "12" => "3rem",
//         "14" => "3.5rem",
//         "16" => "4rem",
//         "20" => "5rem",
//         "24" => "6rem",
//         "28" => "7rem",
//         "32" => "8rem",
//         "36" => "9rem",
//         "40" => "10rem",
//         "44" => "11rem",
//         "48" => "12rem",
//         "52" => "13rem",
//         "56" => "14rem",
//         "60" => "15rem",
//         "64" => "16rem",
//         "72" => "18rem",
//         "80" => "20rem",
//         "96" => "24rem",
//         _ => "0px",
//     }
//     .to_string()
// }

// pub fn convert_breakpoint(breakpoint: &str) -> String {
//     match breakpoint {
//         "sm" => "640px",
//         "md" => "786px",
//         "lg" => "1024px",
//         "xl" => "1280px",
//         "2xl" => "1536px",
//         _ => "1024px",
//     }
//     .to_string()
// }
