use crate::class::{
    helpers::{
        new_component_value_percentage, new_component_value_ratio, new_component_value_str,
        new_declaration, new_qualified_rule_prelude, new_qualified_rule_with_pseudoclass_prelude,
        new_simple_block,
    },
    BaseClass,
};
use swc_common::Span;
use swc_css::ast::{ComponentValue, QualifiedRule, Rule, SimpleBlock, StyleBlock};

#[derive(Debug)]
pub struct AspectRatio {
    pub base: Option<BaseClass>,
    pub ratio: Ratio,
}

#[derive(Debug)]
pub enum Ratio {
    Auto,
    Square,
    Video,
}

impl Ratio {
    fn from_str(str: &str) -> Option<Ratio> {
        match str {
            "aspect-auto" => Some(Ratio::Auto),
            "aspect-square" => Some(Ratio::Square),
            "aspect-video" => Some(Ratio::Video),
            _ => None,
        }
    }
}

impl AspectRatio {
    pub fn new(str: &str) -> Option<Self> {
        if let Some(ratio) = Ratio::from_str(str) {
            return Some(Self {
                base: BaseClass::parse_from_str(str),
                ratio,
            });
        }

        None
    }

    fn get_class_selector(&self) -> String {
        match self.ratio {
            Ratio::Auto => "aspect-auto".into(),
            Ratio::Square => "aspect-square".into(),
            Ratio::Video => "aspect-video".into(),
        }
    }

    pub fn generate_rule(self) -> Rule {
        let values: Vec<ComponentValue> = match self.ratio {
            Ratio::Auto => vec![new_component_value_str("auto")],
            Ratio::Square => vec![new_component_value_ratio(1, Some(1))],
            Ratio::Video => vec![new_component_value_ratio(16, Some(9))],
        };

        let block = new_simple_block(new_declaration("aspect-ratio", values));

        if let Some(base_class) = self.base {
            return Rule::QualifiedRule(Box::new(QualifiedRule {
                span: Span::default(),
                prelude: new_qualified_rule_with_pseudoclass_prelude(
                    "container",
                    base_class.to_string_vec(),
                ),
                block,
            }));
        } else {
            return Rule::QualifiedRule(Box::new(QualifiedRule {
                span: Span::default(),
                prelude: new_qualified_rule_prelude(self.get_class_selector().as_str()),
                block,
            }));
        }
    }
}
