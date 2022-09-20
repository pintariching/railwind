use crate::{
    class::helpers::{
        new_component_value_percentage, new_declaration, new_qualified_rule_prelude,
        new_qualified_rule_with_pseudoclass_prelude, new_simple_block,
    },
    modifiers::Modifier,
};
use swc_common::Span;
use swc_css::ast::{QualifiedRule, Rule};

#[derive(Debug)]
pub struct Container(pub Option<Vec<Modifier>>);

impl Container {
    pub fn default() -> Self {
        Self(None)
    }
    pub fn generate_rule(self) -> Rule {
        let block = new_simple_block(new_declaration(
            "width",
            vec![new_component_value_percentage(100)],
        ));

        if let Some(modifiers) = self.0 {
            return Rule::QualifiedRule(Box::new(QualifiedRule {
                span: Span::default(),
                prelude: new_qualified_rule_with_pseudoclass_prelude(
                    "container",
                    modifiers.iter().map(|m| m.to_string()).collect(),
                ),
                block,
            }));
        } else {
            return Rule::QualifiedRule(Box::new(QualifiedRule {
                span: Span::default(),
                prelude: new_qualified_rule_prelude("container"),
                block,
            }));
        }
    }
}
